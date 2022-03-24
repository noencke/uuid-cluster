extern crate alloc;
use alloc::boxed::Box;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn leak_test() {
    // This leaks
    CompressorOxide {
        params: ParamsOxide {
            local_buf: Box::new(LocalBuf {
                b: [0; OUT_BUF_SIZE],
            }),
        },
        dict: { DictOxide { b: Box::default() } },
    };

    // This does not leak:
    // LZOxide::new();
    // ParamsOxide::new(0);
    // Box::<HuffmanOxide>::default();
    // DictOxide::new(0);
}

struct CompressorOxide {
    pub params: ParamsOxide,
    pub dict: DictOxide,
}

/// Size of the chained hash table.
const LZ_DICT_SIZE: usize = 32_768;

/// The maximum length of a match.
const MAX_MATCH_LEN: usize = 258;

struct DictOxide {
    /// Buffer of input data.
    /// Padded with 1 byte to simplify matching code in `compress_fast`.
    pub b: Box<HashBuffers>,
}

struct ParamsOxide {
    pub local_buf: Box<LocalBuf>,
}

/// Size of the buffer of lz77 encoded data.
const LZ_CODE_BUF_SIZE: usize = 64 * 1024;
/// Size of the output buffer.
const OUT_BUF_SIZE: usize = (LZ_CODE_BUF_SIZE * 13) / 10;
const LZ_DICT_FULL_SIZE: usize = LZ_DICT_SIZE + MAX_MATCH_LEN - 1 + 1;

struct HashBuffers {
    pub dict: [u8; LZ_DICT_FULL_SIZE],
    pub next: [u16; LZ_DICT_SIZE],
    pub hash: [u16; LZ_DICT_SIZE],
}

impl Default for HashBuffers {
    fn default() -> HashBuffers {
        HashBuffers {
            dict: [0; LZ_DICT_FULL_SIZE],
            next: [0; LZ_DICT_SIZE],
            hash: [0; LZ_DICT_SIZE],
        }
    }
}

struct LocalBuf {
    pub b: [u8; OUT_BUF_SIZE],
}
