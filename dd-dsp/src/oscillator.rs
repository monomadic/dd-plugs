//! Sinewave oscillator.

use std::f64::consts::PI;
pub const TAU : f64 = PI * 2.0;

use types::*;

pub fn sine(sample_rate: f64, frequency: f64, playhead: Playhead) -> f64 {
    (frequency * TAU * (playhead as f64 / sample_rate)).sin()
}
