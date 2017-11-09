// based on mverb
// https://github.com/martineastwood/mverb/blob/master/VstPlugin.cpp

#[macro_use] extern crate vst2;

use vst2::buffer::AudioBuffer;
use vst2::plugin::{Category, Plugin, Info};

extern crate dsp;
extern crate lanceverb;
use lanceverb::Reverb;
use dsp::{Frame, Node};

const SAMPLE_RATE: f32 = 44100.;
const PRE_DELAY_TIME: f32 = 100. * (SAMPLE_RATE / 1000.);

struct CombVerb {

    // params:
    dampening_freq: f32,
    density: f32,
    bandwidth_freq: f32,
    decay: f32,
    pre_delay: f32,
    size: f32,
    gain: f32,
    mix: f32,
    early_mix: f32,

    // state:
    prev_left_tank: f32,
    prev_right_tank: f32,
    sample_rate: f32,

    reverb: Reverb,

}

impl Default for CombVerb {
    fn default() -> CombVerb {
        CombVerb {
            // presets
            dampening_freq: 18000.,
            density: 1.,
            bandwidth_freq: 18000.,
            decay: 0.5,
            pre_delay: 100. * (SAMPLE_RATE / 1000.),
            gain: 1.,
            mix: 1.,
            size: 1.,
            early_mix: 1.,
            // state
            prev_left_tank: 0.,
            prev_right_tank: 0.,
            sample_rate: 0.,
            reverb: Reverb::new(),
        }
    }
}

impl Plugin for CombVerb {
    fn get_info(&self) -> Info {
        Info {
            name: "CombVerb".to_string(),
            vendor: "DeathDisco".to_string(),
            unique_id: 12356677,
            category: Category::Effect,

            inputs: 2,
            outputs: 2,
            parameters: 1,

            ..Info::default()
        }
    }

    // fn get_preset_data(&mut self) -> Vec<u8> { Vec::new() }
    // fn get_bank_data(&mut self) -> Vec<u8> { Vec::new() }
    // fn load_preset_data(&mut self, data: &[u8]) { }
    // fn load_bank_data(&mut self, data: &[u8]) { }

    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.bandwidth_freq,
            1 => self.dampening_freq,
            _ => 0.0,
        }
    }

    fn set_parameter(&mut self, index: i32, value: f32) {
        match index {
            // We don't want to divide by zero, so we'll clamp the value
            0 => self.bandwidth_freq = value.max(0.01),
            1 => self.dampening_freq = value,
            _ => (),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Room Size".to_string(),
            1 => "Dampening".to_string(),
            _ => "".to_string(),
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            // Convert to a percentage
            0 => format!("{}", self.bandwidth_freq * 100.0),
            1 => format!("{}", self.dampening_freq * 100.0),
            _ => "".to_string(),
        }
    }

    fn get_parameter_label(&self, index: i32) -> String {
        match index {
            0 => "%".to_string(),
            1 => "%".to_string(),
            _ => "".to_string(),
        }
    }

    fn set_sample_rate(&mut self, rate: f32) { 
        self.sample_rate = rate;
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        // Split out the input and output buffers into two vectors
        let (input_buffer, mut output_buffer) = buffer.split();

        // Assume 2 channels
        if input_buffer.len() < 2 || output_buffer.len() < 2 { return; }

        // Iterate over inputs as (&f32, &f32)
        let (l, r) = input_buffer.split_at(1);
        let stereo_in = l[0].iter().zip(r[0].iter());

        // Iterate over outputs as (&mut f32, &mut f32)
        let (mut l, mut r) = output_buffer.split_at_mut(1);
        let stereo_out = l[0].iter_mut().zip(r[0].iter_mut());

        for ((left_in, right_in), (left_out, right_out)) in stereo_in.zip(stereo_out) {
            *left_out = *left_in;
            *right_out = *right_in;
            self.reverb.audio_requested(dsp::Frame::new(&mut [left_out]), SAMPLE_RATE as f64);
        }
    }
}

plugin_main!(CombVerb);
