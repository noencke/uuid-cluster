use std::io::Write;

use flate2::Compression;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn leak_test() {
    let zero_vec = vec![0; 24];
    let mut compressor = flate2::write::DeflateEncoder::new(Vec::new(), Compression::new(6));
    compressor.write_all(&zero_vec).unwrap();
}
