use crate::DEFAULT_SAMPLE_RATE;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct AudioBuffer {
    pub sample_rate: usize,
    pub size: usize,
    data: Vec<f32>
}

impl AudioBuffer {
    pub fn new(raw_data: &[f32], sample_rate: Option<usize>) -> Self {
        let sample_rate = sample_rate.unwrap_or(DEFAULT_SAMPLE_RATE);
        let size = raw_data.len();
        let data = raw_data.to_vec();
        Self { sample_rate, size, data }
    }
}

#[wasm_bindgen]
impl AudioBuffer {
    pub fn get_data(self) -> Vec<f32> {
        self.data
    }
}