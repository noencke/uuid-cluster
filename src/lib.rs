extern crate alloc;
use alloc::boxed::Box;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn leak_test() {
    for _ in 0..1000 {
        // This leaks when using wee_alloc
        let a = Box::new([0; 8190]);
        let b = Box::new([0; 8191]);
        drop(b);
        drop(a);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn does_it_leak() {
        loop {
            super::leak_test()
        }
    }
}
