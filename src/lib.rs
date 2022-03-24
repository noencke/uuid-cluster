extern crate alloc;
use alloc::boxed::Box;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn leak_test() {
    // This leaks ~ 721 kb when using wee_alloc
    let a = Box::new([0; 85196]);
    let b = Box::new([0; 164098]);
    drop(a);
    drop(b);
}
