use crate::DEFAULT_SAMPLE_RATE;

const PI: f32 = 3.14159265358979323846264338327950288_f32;

fn sgn(x: f32) -> f32 {
    if x > 0.0 {
        1.0
    } else if x < 0.0 {
        -1.0
    } else {
        0.0
    }
}

fn calculate_oscillator_sample(frame: f32, oscillator_type: &str) -> f32 {
    let s = (2.0 * PI * frame).sin();
    match oscillator_type {
        "sine" => s,
        "square" => sgn(s),
        "triangle" => (PI / 2.0) * s.asin(),
        "sawtooth" => 2.0 * (frame - (0.5 + frame).floor()),
        _ => panic!("wave type not supported")
    }
}

pub struct Lfo<'a> {
    oscillator_type: &'a str,
    frequency: f32,
    sample_rate: usize,
    curr: usize
}

impl<'a> Lfo<'a> {
    pub fn new(oscillator_type: &'a str, frequency: f32, sample_rate: Option<usize>) -> Self {
        Self {
            oscillator_type,
            frequency,
            sample_rate: sample_rate.unwrap_or(DEFAULT_SAMPLE_RATE),
            curr: 0
        }
    }
}

impl Iterator for Lfo<'_> {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let frame = self.frequency * self.curr as f32 / self.sample_rate as f32;
        let sample = calculate_oscillator_sample(frame, self.oscillator_type);
        self.curr += 1;
        Some(sample)
    }
}


pub struct Oscillator<'a> {
    oscillator_type: &'a str,
    midi_number: u8,
    sample_rate: usize,
    curr: usize
}

impl<'a> Oscillator<'a> {
    pub fn new(oscillator_type: &'a str, midi_number: u8, sample_rate: Option<usize>) -> Self {
        Self {
            oscillator_type,
            midi_number,
            sample_rate: sample_rate.unwrap_or(DEFAULT_SAMPLE_RATE),
            curr: 0
        }
    }
}

impl Iterator for Oscillator<'_> {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let freq = 2.0_f32.powf((self.midi_number as f32 - 69.0) / 12.0) * 440.0;
        let frame = freq * self.curr as f32 / self.sample_rate as f32;
        let sample = calculate_oscillator_sample(frame, self.oscillator_type);
        self.curr += 1;
        Some(sample)
    }
}
