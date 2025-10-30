use crate::DEFAULT_SAMPLE_RATE;
use crate::{buffer, oscillator};
use wasm_bindgen::prelude::*;

fn parse_amplitude(amplitude_text: &str, duration: usize) -> Vec<f32>  {
    let details: Vec<&str> = amplitude_text.split('-').collect();
    if details[0] == "scalar" {
        let amplitude: f32 = details[1].parse().unwrap_or(1.0);
        vec![amplitude; duration]
    } else if details[0] == "lfo" {
        let oscillator_type = details[1];
        let frequency: f32 = details[2].parse().unwrap();
        let lfo = oscillator::Lfo::new(oscillator_type, frequency, None);
        lfo.take(duration).collect()
    } else {
        vec![0.0; duration]
    }
}

#[wasm_bindgen]
pub struct InstrumentSource {
    data: Vec<buffer::AudioBuffer>
}

#[wasm_bindgen]
impl InstrumentSource {
    pub fn from_oscillator(oscillator_type: &str, amplitude_text: &str) -> Self {
        let amplitude_buffer = parse_amplitude(amplitude_text, DEFAULT_SAMPLE_RATE / 2);
        let create_buffer = |i| {
            let osc = oscillator::Oscillator::new(oscillator_type, i, None);
            let source: Vec<f32> = osc.take(DEFAULT_SAMPLE_RATE / 2).collect();
            let mix_iter = source.iter().zip(amplitude_buffer.iter()).map(|(s, a)| s * a);
            let mix: Vec<f32> = mix_iter.collect();
            buffer::AudioBuffer::new(&mix, None)
        };
        Self { data: (0..=127).map(create_buffer).collect() }
    }

    pub fn get_data(self) -> Vec<buffer::AudioBuffer> {
        return self.data;
    }
}