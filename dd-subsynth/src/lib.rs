#[macro_use] 
extern crate vst2;

#[macro_use] 
extern crate log;
extern crate simplelog;

use simplelog::*;
use std::fs::File;

use vst2::buffer::AudioBuffer;
use vst2::plugin::{Category, Plugin, Info};
use vst2::event::{Event};

use std::collections::HashMap;

extern crate dd_dsp;
use dd_dsp::oscillator;
use dd_dsp::Instrument;
use dd_dsp::midi;
use dd_dsp::envelope;
use dd_dsp::types::*;

/// Size of VST params.
type Param = f32;

/// Used for timings of samples (eg position into voice)
type SampleTiming = u64;

struct SimpleSynth {
    playhead: Playhead,
    sample_rate: f64,
    attack_ratio: Param,
    release_ratio: Param,
    instrument: Instrument,
    envelope: envelope::ADSR,
}

//#[derive(Clone)]
//struct Voice {
//    samples_elapsed: u64,
//    pitch_in_hz: f64,
//
//    /// Volume envelope for this voice.
//    oscillator: oscillator::Sine,
//
//    /// Time when note_off was fired.
//    released_at: Option<SampleTiming>,
//}

impl Default for SimpleSynth {
    fn default() -> Self {
        let _ = CombinedLogger::init(
            vec![
                // TermLogger::new( LevelFilter::Warn, Config::default()).unwrap(),
                WriteLogger::new(LogLevelFilter::Info, Config::default(), File::create("/tmp/simplesynth.log").unwrap()),
            ]
        );
        Self {
            playhead: 0,
            sample_rate: 0.0,
            attack_ratio: 0.75,
            release_ratio: 0.0001,
            instrument: Instrument::new(),
            envelope: envelope::ADSR{ attack_time: 50.0, release_time: 90.0 },
        }
    }

}

use dd_dsp::VoiceState;
use std::f64::consts::PI;
pub const TAU:f64 = PI * 2.0;

impl SimpleSynth {

    fn process_sample(&self, playhead: Playhead) -> Sample {
        let mut output_sample = 0.0;

        for (note, voice) in self.instrument.voices.iter() {
            output_sample += oscillator::sine(
                self.sample_rate,
                midi::midi_note_to_hz(*note),
                playhead
            );
        }

        (output_sample / 4.0) as Sample
    }

    fn process_midi_event(&mut self, data: [u8; 3]) {
        match data[0] {
            128 => self.note_off(data[1]),
            144 => self.note_on(data[1]),
            _ => info!("unsupported midi opcode: {}", data[0])
        }
    }

    fn note_on(&mut self, note: u8) { self.instrument.note_on(note, self.playhead); }
    fn note_off(&mut self, note: u8) { self.instrument.note_off(note, self.playhead); }
}

impl Plugin for SimpleSynth {

    fn get_info(&self) -> Info {
        Info {
            name: "DD-SimpleSynth".to_string(),
            vendor: "DeathDisco".to_string(),
            unique_id: 6667,
            category: Category::Synth,
            inputs: 0,
            outputs: 1,
            parameters: 4,
            initial_delay: 0,
            ..Info::default()
        }
    }

    fn process_events(&mut self, events: Vec<Event>) {
        for event in events {
            match event {
                Event::Midi { data, .. } => self.process_midi_event(data),
                Event::SysEx { .. } => info!("sysex"),
                Event::Deprecated { .. } => info!("deprecated"),
            }
        }
    }

    fn process(&mut self, buffer: AudioBuffer<f32>) {
        let (_, output_buffer) = buffer.split();
        let mut buffer_size:u64 = 0;

        for output_channel in output_buffer {
            buffer_size = output_channel.len() as u64;

            for time in 0..buffer_size {
                output_channel[time as usize] = self.process_sample(self.playhead + time);
            }
        }
        self.playhead += buffer_size;
    }

    fn set_sample_rate(&mut self, rate: f32) { 
        info!("sample rate is assigned to {}", rate);
        self.sample_rate = rate as f64;
    }

    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => (self.envelope.attack_time / 1000.0) as f32,
            1 => (self.envelope.release_time / 1000.0) as f32,
            2 => self.attack_ratio,
            3 => self.release_ratio,
            _ => 0.0,
        }
    }

    fn set_parameter(&mut self, index: i32, value: f32) {
        match index {
            0 => self.envelope.attack_time = (value.max(0.001) * 1000.0) as f64, // avoid pops by always having at least a tiny attack.
            1 => self.envelope.release_time = (value.max(0.001) * 1000.0) as f64, // same with release.
            2 => self.attack_ratio = value.max(0.00001), // same with release.
            3 => self.release_ratio = value.max(0.00001), // same with release.
            _ => (),
        };
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Attack".to_string(),
            1 => "Release".to_string(),
            2 => "Attack Curve".to_string(),
            3 => "Release Curve".to_string(),
            _ => "".to_string(),
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => format!("{}ms", (self.envelope.attack_time)),
            1 => format!("{}ms", (self.envelope.release_time)),
            2 => format!("{}", (self.attack_ratio * 1000.0)),
            3 => format!("{}", (self.release_ratio * 1000.0)),
            _ => "".to_string(),
        }
    }

    fn get_parameter_label(&self, index: i32) -> String {
        match index {
            0 => "ms".to_string(),
            1 => "ms".to_string(),
            2 => "%".to_string(),
            3 => "%".to_string(),
            _ => "".to_string(),
        }
    }
}

plugin_main!(SimpleSynth);
