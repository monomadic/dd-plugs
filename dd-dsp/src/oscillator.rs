//! Sinewave oscillator.

use std::f64::consts::PI;
pub const TAU : f64 = PI * 2.0;

#[derive(Clone)]
pub struct SineOsc {
    sample_rate: f64,
    sample_number: u64,
    frequency: f64,
}

impl SineOsc {
    pub fn new(sample_rate: f64, frequency: f64) -> SineOsc {
        SineOsc {
            sample_rate: sample_rate,
            sample_number: 0,
            frequency: frequency,
        }
    }

    pub fn process(&mut self) -> f64 {
        self.sample_number += 1;
        (self.frequency * TAU * ((self.sample_number - 1) as f64 / self.sample_rate)).sin()
    }
}
