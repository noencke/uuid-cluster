use wasm_bindgen::prelude::*;

struct CompressedCluster {
    session_index: u64,
    capacity: u64,
    count: u64,
}

#[wasm_bindgen]
pub struct ClusterCompressor {
    clusters: Vec<CompressedCluster>,
}

#[wasm_bindgen]
impl ClusterCompressor {
    pub fn new() -> ClusterCompressor {
        ClusterCompressor { clusters: vec![] }
    }

    pub fn decompress(compressed: String, decompressed_cluster: &js_sys::Function) {
        let decompressed_bytes = base64::decode(compressed).unwrap();
        let mut previous_session_index: u64 = 0;
        for chunk in decompressed_bytes.chunks(24) {
            let session_index_delta = i64::from_le_bytes(chunk[0..8].try_into().unwrap());
            let session_index: u64 = previous_session_index + session_index_delta as u64;
            previous_session_index = session_index;
            let capacity = u64::from_le_bytes(chunk[8..16].try_into().unwrap());
            let count = u64::from_le_bytes(chunk[16..24].try_into().unwrap());
            decompressed_cluster.call3(&JsValue::NULL, &JsValue::from_f64(session_index as f64), &JsValue::from_f64(capacity as f64), &JsValue::from_f64(count as f64)).unwrap();
        }
    }

    pub fn add(&mut self, session_index: f64, capacity: f64, count: f64) {
        self.clusters.push(CompressedCluster {
            session_index: validate_number(session_index),
            capacity: validate_number(capacity),
            count: validate_number(count),
        });
    }

    pub fn compress(self) -> String {
        let mut foo = vec![];
        let mut previous_session_index: u64 = 0;
        for cluster in self.clusters {
            let current_session_index_delta: i64 = cluster.session_index as i64 - previous_session_index as i64;
            previous_session_index = cluster.session_index;
            foo.extend_from_slice(&current_session_index_delta.to_le_bytes());
            foo.extend_from_slice(&cluster.capacity.to_le_bytes());
            foo.extend_from_slice(&cluster.count.to_le_bytes());
        }
        base64::encode(foo)
    }
}

fn validate_number(number: f64) -> u64 {
    let result = number as u64;
    if result as f64 != number {
        panic!("expected convertible integer")
    }
    if result >= 1 << 53 {
        panic!("number larger than javascript max safe integer")
    }
    result
}
