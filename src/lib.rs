use flate2::Compression;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn leak_test() {
    flate2::write::DeflateEncoder::new(Vec::new(), Compression::new(1));
}
