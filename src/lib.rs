extern crate alloc;
use alloc::boxed::Box;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn leak_test() {
    // This leaks
    CompressorOxide {
        params: ParamsOxide::new(0),
        dict: DictOxide::new(0),
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
    /// The maximum number of checks in the hash chain, for the initial,
    /// and the lazy match respectively.
    pub max_probes: [u32; 2],
    /// Buffer of input data.
    /// Padded with 1 byte to simplify matching code in `compress_fast`.
    pub b: Box<HashBuffers>,

    pub code_buf_dict_pos: usize,
    pub lookahead_size: usize,
    pub lookahead_pos: usize,
    pub size: usize,
}

const fn probes_from_flags(flags: u32) -> [u32; 2] {
    [
        1 + ((flags & 0xFFF) + 2) / 3,
        1 + (((flags & 0xFFF) >> 2) + 2) / 3,
    ]
}

impl DictOxide {
    pub fn new(flags: u32) -> Self {
        DictOxide {
            max_probes: probes_from_flags(flags),
            b: Box::default(),
            code_buf_dict_pos: 0,
            lookahead_size: 0,
            lookahead_pos: 0,
            size: 0,
        }
    }
}

struct ParamsOxide {
    pub flags: u32,
    pub greedy_parsing: bool,
    pub block_index: u32,

    pub saved_match_dist: u32,
    pub saved_match_len: u32,
    pub saved_lit: u8,

    pub flush_ofs: u32,
    pub flush_remaining: u32,
    pub finished: bool,

    pub adler32: u32,

    pub src_pos: usize,

    pub out_buf_ofs: usize,
    pub prev_return_status: i32,

    pub saved_bit_buffer: u32,
    pub saved_bits_in: u32,

    pub local_buf: Box<LocalBuf>,
}

impl ParamsOxide {
    pub fn new(flags: u32) -> Self {
        ParamsOxide {
            flags,
            greedy_parsing: false,
            block_index: 0,
            saved_match_dist: 0,
            saved_match_len: 0,
            saved_lit: 0,
            flush_ofs: 0,
            flush_remaining: 0,
            finished: false,
            adler32: 0,
            src_pos: 0,
            out_buf_ofs: 0,
            prev_return_status: 0,
            saved_bit_buffer: 0,
            saved_bits_in: 0,
            local_buf: Box::default(),
        }
    }
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

impl Default for LocalBuf {
    fn default() -> LocalBuf {
        LocalBuf {
            b: [0; OUT_BUF_SIZE],
        }
    }
}
