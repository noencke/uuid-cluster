// Ignore link from wasm_bindgen: https://github.com/rustwasm/wasm-bindgen/issues/2774
#![allow(clippy::unused_unit)]

use byteorder::ByteOrder;
use byteorder::LittleEndian;
use miniz_oxide::deflate::compress_to_vec;
use miniz_oxide::inflate::decompress_to_vec;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Setup `console_error_panic_hook` for more friendly panic handling in debug builds.
#[cfg(all(debug_assertions, feature = "console_error_panic_hook"))]
#[wasm_bindgen]
pub fn setup() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    console_error_panic_hook::set_once();
}

#[derive(Clone, Copy)]
pub(crate) struct CompressedCluster {
    session_index: u64,
    capacity: u64,
    count: u64,
}

#[wasm_bindgen]
#[derive(Default)]
pub struct ClusterCompressor {
    clusters: Vec<CompressedCluster>,
}

#[wasm_bindgen]
impl ClusterCompressor {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        ClusterCompressor { clusters: vec![] }
    }

    pub fn decompress(compressed: String, decompressed_cluster: &js_sys::Function) {
        ClusterCompressor::decompress_rust(compressed, |cluster| {
            decompressed_cluster
                .call3(
                    &JsValue::NULL,
                    &JsValue::from_f64(cluster.session_index as f64),
                    &JsValue::from_f64(cluster.capacity as f64),
                    &JsValue::from_f64(cluster.count as f64),
                )
                .unwrap();
        });
    }

    pub fn add(&mut self, session_index: f64, capacity: f64, count: f64) {
        self.add_rust(CompressedCluster {
            session_index: validate_number(session_index),
            capacity: validate_number(capacity),
            count: validate_number(count),
        });
    }

    /// Level is deflate compression level. Should be integer from 0-9 (inclusive). Default 6.
    pub fn compress(&self, level: u8) -> String {
        let mut uncompressed = Vec::with_capacity(self.clusters.len() * 24);
        let mut previous_session_index: u64 = 0;
        for cluster in &self.clusters {
            let current_session_index_delta: i64 =
                cluster.session_index as i64 - previous_session_index as i64;
            previous_session_index = cluster.session_index;
            uncompressed.extend_from_slice(&current_session_index_delta.to_le_bytes());
            uncompressed.extend_from_slice(&cluster.capacity.to_le_bytes());
            uncompressed.extend_from_slice(&cluster.count.to_le_bytes());
        }

        base64::encode(compress_to_vec(uncompressed.as_slice(), level))
    }
}

impl ClusterCompressor {
    pub(crate) fn decompress_rust<T: FnMut(CompressedCluster)>(
        compressed: String,
        mut call_back: T,
    ) {
        let compressed_bytes = base64::decode(compressed).unwrap();
        let bytes = decompress_to_vec(compressed_bytes.as_slice()).unwrap();
        let mut previous_session_index: u64 = 0;
        for chunk in bytes.chunks(24) {
            let session_index_delta = LittleEndian::read_i64(chunk);
            let session_index: u64 = previous_session_index + session_index_delta as u64;
            previous_session_index = session_index;
            let capacity = LittleEndian::read_u64(&chunk[8..16]);
            let count = LittleEndian::read_u64(&chunk[16..24]);
            call_back(CompressedCluster {
                session_index,
                capacity,
                count,
            })
        }
    }

    pub(crate) fn add_rust(&mut self, cluster: CompressedCluster) {
        self.clusters.push(cluster);
    }
}

pub(crate) fn validate_number(number: f64) -> u64 {
    let result = number as u64;
    if result as f64 != number || result >= 1 << 53 {
        panic!("expected convertible integer")
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_number() {
        assert_eq!(validate_number(5f64), 5);
        assert_eq!(validate_number(0f64), 0);
        assert_eq!(validate_number((1u64 << 53 - 1) as f64), 1u64 << 53 - 1);
        assert_eq!(validate_number(0f64), 0);
    }

    #[test]
    #[should_panic]
    fn test_reject_number1() {
        validate_number((1u64 << 53) as f64);
    }

    #[test]
    #[should_panic]
    fn test_reject_number2() {
        validate_number(0.00001);
    }

    #[test]
    #[should_panic]
    fn test_reject_number3() {
        validate_number(-1.0);
    }

    #[test]
    #[should_panic]
    fn test_reject_number4() {
        validate_number(f64::NAN);
    }

    #[test]
    #[should_panic]
    fn test_reject_number5() {
        validate_number(f64::INFINITY);
    }

    #[test]
    #[should_panic]
    fn test_reject_number6() {
        validate_number((1u64 << 55) as f64);
    }

    #[test]
    fn test_cluster_compressor_empty() {
        check_compressor_roundtrip(&vec![]);
    }

    #[test]
    fn test_cluster_compressor_simple() {
        check_compressor_roundtrip(&vec![CompressedCluster {
            session_index: 1,
            capacity: 2,
            count: 2,
        }]);
    }

    fn check_compressor_roundtrip(data: &[CompressedCluster]) {
        let mut cc = ClusterCompressor::new();
        for c in data {
            cc.add_rust(*c)
        }
        let x = cc.compress(6);

        let mut cc2 = ClusterCompressor::new();
        ClusterCompressor::decompress_rust(x.clone(), |cluster| cc2.add_rust(cluster));

        assert_eq!(&cc2.compress(6), &x);
    }
}
