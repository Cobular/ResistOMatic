mod utils;
mod math;
mod error;
mod tree;
mod dynamic_tree;

use wasm_bindgen::prelude::*;

use crate::error::Result;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, resist-o-calc!");
}

#[wasm_bindgen]
pub fn find_by_voltage(v1: f64, v2: f64) -> Result<()> {
    crate::math::find_by_voltage(v1, v2)
}
