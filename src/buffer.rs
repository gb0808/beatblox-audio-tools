use crate::DEFAULT_SAMPLE_RATE;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct AudioBuffer {
    sample_rate: usize,
    data: Vec<u8>
}

impl AudioBuffer {
    pub fn new(raw_data: &[f32], sample_rate: Option<usize>) -> Self {
        let sample_rate = sample_rate.unwrap_or(DEFAULT_SAMPLE_RATE);
        let mut data: Vec<u8> = Vec::new();
        raw_data.iter().for_each(|frame: &f32| {
            let bytes = frame.to_ne_bytes();
            bytes.iter().for_each(|byte| data.push(*byte));
        });
        Self { sample_rate, data }
    }
}

#[wasm_bindgen]
impl AudioBuffer {
    pub fn get_data(self) -> Vec<u8> {
        self.data
    }
}