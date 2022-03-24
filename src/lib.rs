use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn leak_test() {
    // Leaks ~131 kb per call
    miniz_oxide::deflate::core::CompressorOxide::new(0);

    // Leaks ~393 kb per call
    //flate2::write::DeflateEncoder::new(Vec::new(), Compression::new(0));
}
