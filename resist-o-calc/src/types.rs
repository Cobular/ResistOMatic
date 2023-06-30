use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::utils::float_to_val;

#[wasm_bindgen]
pub enum ResistorSets {
    E3,
    E6,
    E12,
    E24,
}

#[derive(Debug)]
pub struct ResistorPair<'a> {
    data: &'a [f64],
    top_idx: usize,
    bottom_idx: usize,
}

impl<'a> ResistorPair<'a> {
    pub fn new(data: &'a [f64], top_idx: usize, bottom_idx: usize) -> Self {
        Self {
            data,
            top_idx,
            bottom_idx,
        }
    }

    pub fn top_value(&self) -> f64 {
        float_to_val(self.data, self.top_idx)
    }

    pub fn bottom_value(&self) -> f64 {
        float_to_val(self.data, self.bottom_idx)
    }

    pub fn total_resistance(&self) -> f64 {
        self.top_value() + self.bottom_value()
    }

    pub fn overall_ratio(&self) -> f64 {
        self.top_value() / self.bottom_value()
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct OutputResistorPair {
    top_val: f64,
    bottom_val: f64,
}

impl<'a> From<ResistorPair<'a>> for OutputResistorPair {
    fn from(value: ResistorPair) -> Self {
        Self {
            top_val: value.top_value(),
            bottom_val: value.bottom_value(),
        }
    }
}


impl<'a> From<&ResistorPair<'a>> for OutputResistorPair {
  fn from(value: &ResistorPair) -> Self {
      Self {
          top_val: value.top_value(),
          bottom_val: value.bottom_value(),
      }
  }
}
