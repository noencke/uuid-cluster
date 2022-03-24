extern crate alloc;
use alloc::boxed::Box;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn leak_test() {
    // This leaks
    CompressorOxide {
        lz: LZOxide::new(),
        params: ParamsOxide::new(0),
        huff: Box::<HuffmanOxide>::default(),
        dict: DictOxide::new(0),
    };

    // This does not leak:
    // LZOxide::new();
    // ParamsOxide::new(0);
    // Box::<HuffmanOxide>::default();
    // DictOxide::new(0);
}

// pub mod deflate;

/// Strategy setting for compression.
///
/// The non-default settings offer some special-case compression variants.
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum CompressionStrategy {
    /// Don't use any of the special strategies.
    Default = 0,
    /// Only use matches that are at least 5 bytes long.
    Filtered = 1,
    /// Don't look for matches, only huffman encode the literals.
    HuffmanOnly = 2,
    /// Only look for matches with a distance of 1, i.e do run-length encoding only.
    RLE = 3,
    /// Only use static/fixed blocks. (Blocks using the default huffman codes
    /// specified in the deflate specification.)
    Fixed = 4,
}

/// A list of deflate flush types.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum TDEFLFlush {
    /// Normal operation.
    ///
    /// Compress as much as there is space for, and then return waiting for more input.
    None = 0,

    /// Try to flush all the current data and output an empty raw block.
    Sync = 2,

    /// Same as [`Sync`][Self::Sync], but reset the dictionary so that the following data does not
    /// depend on previous data.
    Full = 3,

    /// Try to flush everything and end the deflate stream.
    ///
    /// On success this will yield a [`TDEFLStatus::Done`] return status.
    Finish = 4,
}

/// Return status of compression.
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum TDEFLStatus {
    /// Usage error.
    ///
    /// This indicates that either the [`CompressorOxide`] experienced a previous error, or the
    /// stream has already been [`TDEFLFlush::Finish`]'d.
    BadParam = -2,

    /// Error putting data into output buffer.
    ///
    /// This usually indicates a too-small buffer.
    PutBufFailed = -1,

    /// Compression succeeded normally.
    Okay = 0,

    /// Compression succeeded and the deflate stream was ended.
    ///
    /// This is the result of calling compression with [`TDEFLFlush::Finish`].
    Done = 1,
}

const MAX_HUFF_SYMBOLS: usize = 288;

/// The number of huffman tables used by the compressor.
/// Literal/length, Distances and Length of the huffman codes for the other two tables.
const MAX_HUFF_TABLES: usize = 3;

/// Size of the chained hash table.
const LZ_DICT_SIZE: usize = 32_768;

/// The maximum length of a match.
const MAX_MATCH_LEN: usize = 258;

/// Main compression struct.
struct CompressorOxide {
    pub lz: LZOxide,
    pub params: ParamsOxide,
    pub huff: Box<HuffmanOxide>,
    pub dict: DictOxide,
}

/// A struct containing data about huffman codes and symbol frequencies.
///
/// NOTE: Only the literal/lengths have enough symbols to actually use
/// the full array. It's unclear why it's defined like this in miniz,
/// it could be for cache/alignment reasons.
struct HuffmanOxide {
    /// Number of occurrences of each symbol.
    pub count: [[u16; MAX_HUFF_SYMBOLS]; MAX_HUFF_TABLES],
    /// The bits of the huffman code assigned to the symbol
    pub codes: [[u16; MAX_HUFF_SYMBOLS]; MAX_HUFF_TABLES],
    /// The length of the huffman code assigned to the symbol.
    pub code_sizes: [[u8; MAX_HUFF_SYMBOLS]; MAX_HUFF_TABLES],
}

impl Default for HuffmanOxide {
    fn default() -> Self {
        HuffmanOxide {
            count: [[0; MAX_HUFF_SYMBOLS]; MAX_HUFF_TABLES],
            codes: [[0; MAX_HUFF_SYMBOLS]; MAX_HUFF_TABLES],
            code_sizes: [[0; MAX_HUFF_SYMBOLS]; MAX_HUFF_TABLES],
        }
    }
}

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

    pub flush: TDEFLFlush,
    pub flush_ofs: u32,
    pub flush_remaining: u32,
    pub finished: bool,

    pub adler32: u32,

    pub src_pos: usize,

    pub out_buf_ofs: usize,
    pub prev_return_status: TDEFLStatus,

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
            flush: TDEFLFlush::None,
            flush_ofs: 0,
            flush_remaining: 0,
            finished: false,
            adler32: 0,
            src_pos: 0,
            out_buf_ofs: 0,
            prev_return_status: TDEFLStatus::Okay,
            saved_bit_buffer: 0,
            saved_bits_in: 0,
            local_buf: Box::default(),
        }
    }
}

struct LZOxide {
    pub codes: [u8; LZ_CODE_BUF_SIZE],
    pub code_position: usize,
    pub flag_position: usize,

    // The total number of bytes in the current block.
    // (Could maybe use usize, but it's not possible to exceed a block size of )
    pub total_bytes: u32,
    pub num_flags_left: u32,
}

impl LZOxide {
    pub const fn new() -> Self {
        LZOxide {
            codes: [0; LZ_CODE_BUF_SIZE],
            code_position: 1,
            flag_position: 0,
            total_bytes: 0,
            num_flags_left: 8,
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

impl HashBuffers {
    #[inline]
    pub fn reset(&mut self) {
        *self = HashBuffers::default();
    }
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
