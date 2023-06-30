mod dynamic_tree;
mod error;
mod math;
mod types;
mod utils;

use wasm_bindgen::prelude::*;

use crate::{
    error::Result,
    types::{OutputResistorPair, ResistorSets},
};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, resist-o-calc!");
}

#[wasm_bindgen]
pub fn find_by_voltage(v1: f64, v2: f64, resistor_set: ResistorSets) -> Result<JsValue> {
    Ok(
        crate::math::find_by_voltage(v1, v2, resistor_set, 5).map(|vec| {
            let output_pairs: Vec<_> = vec
                .iter()
                .map(std::convert::Into::<OutputResistorPair>::into)
                .collect();
            serde_wasm_bindgen::to_value(&output_pairs)
        })??,
    )
}
