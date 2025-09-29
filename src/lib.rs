mod buffer;
mod oscillator;

use wasm_bindgen::prelude::*;

const DEFAULT_SAMPLE_RATE: usize = 44100;

#[wasm_bindgen]
pub fn create_beatblox_oscillator(oscillator_type: &str) -> Vec<buffer::AudioBuffer> {
    let create_buffer = |i| {
        let osc = oscillator::Oscillator::new(oscillator_type, i, None);
        let raw_data: Vec<f32> = osc.take(DEFAULT_SAMPLE_RATE).collect();
        let buffer = buffer::AudioBuffer::new(&raw_data, None);
        buffer
    };
    (0..=127).map(create_buffer).collect::<Vec<buffer::AudioBuffer>>()
}