#[macro_use] 
extern crate vst;

#[macro_use] 
extern crate log;
extern crate simplelog;

use simplelog::*;
use std::fs::File;

use vst::buffer::AudioBuffer;
use vst::plugin::{Category, Plugin, Info};
use vst::event::{Event};
use vst::api::Events;

use std::collections::HashMap;

extern crate dd_dsp;
use dd_dsp::{ Instrument, oscillator, midi, Envelope, SimpleEnvelope };
use dd_dsp::types::*;

/// Size of VST params.
type VSTParam = f32;

// struct Preset {
//     name: String,
//     attack: VSTParam,
//     release: VSTParam,
// }

struct SimpleSynth {
    playhead: Playhead,
    sample_rate: f64,
    instrument: Instrument<SimpleEnvelope>,
    current_preset: i32,
}

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
            current_preset: 0,
            // attack_ratio: 0.75,
            // release_ratio: 0.0001,
            instrument: Instrument {
                voices: Vec::new(),
                envelope: SimpleEnvelope{ attack: 0.2, release: 0.4 }
            },
        }
    }
}

impl SimpleSynth {

    fn process_sample(&mut self, playhead: Playhead) -> Sample {
        let mut output_sample = 0.0;

        self.instrument.cleanup(playhead);

        for &(note, ref voice) in self.instrument.voices.iter() {

            let envelope = self.instrument.envelope.ratio(
                playhead,
                voice,
                self.sample_rate
            );

            let signal = oscillator::sine(
                self.sample_rate,
                midi::midi_note_to_hz(note),
                playhead
            );

            // self.instrument.voices.remove(note);
            
            output_sample += signal * envelope;
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

    // fn get_preset_data(&mut self) -> Vec<u8> { info!("get_preset_data called"); Vec::new() }
    // fn get_bank_data(&mut self) -> Vec<u8> { info!("get_bank_data called"); Vec::new() }
    // fn load_preset_data(&mut self, data: &[u8]) { info!("load_preset_data called"); }
    // fn load_bank_data(&mut self, data: &[u8]) { info!("load_bank_data called"); }

    fn get_info(&self) -> Info {
        Info {
            name: "DD-SimpleSynth".to_string(),
            vendor: "DeathDisco".to_string(),
            unique_id: 6667,
            category: Category::Synth,
            inputs: 0,
            outputs: 1,
            parameters: 2,
            presets: 2,
            initial_delay: 0,
            ..Info::default()
        }
    }

    fn process_events(&mut self, events: &Events) {
        for event in events.events() {
            match event {
                Event::Midi(ev) => self.process_midi_event(ev.data),
                _ => (),
            }
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let (_, output_buffer) = buffer.split();
        let mut buffer_size:u64 = 0;

        for output_channel in output_buffer {
            buffer_size = output_channel.len() as u64;

            for time in 0..buffer_size {
                let current_playhead = self.playhead + time;
                output_channel[time as usize] = self.process_sample(current_playhead );
            }
        }
        self.playhead += buffer_size;
    }

    fn set_sample_rate(&mut self, rate: f32) { 
        info!("sample rate is assigned to {}", rate);
        self.sample_rate = rate as f64;
    }

    fn get_parameter(&self, index: i32) -> VSTParam {
        match index {
            0 => self.instrument.envelope.attack as VSTParam,
            1 => self.instrument.envelope.release as VSTParam,
            _ => 0.0,
        }
    }

    fn set_parameter(&mut self, index: i32, value: VSTParam) {
        match index {
           0 => self.instrument.envelope.attack = (value.max(0.01)) as f64, // avoid pops by always having at least a tiny attack.
           1 => self.instrument.envelope.release = (value.max(0.01)) as f64, // same with release.
            // 2 => self.attack_ratio = value.max(0.00001), // same with release.
            // 3 => self.release_ratio = value.max(0.00001), // same with release.
            _ => (),
        };
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Attack".to_string(),
            1 => "Release".to_string(),
            // 2 => "Attack Curve".to_string(),
            // 3 => "Release Curve".to_string(),
            _ => "".to_string(),
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => format!("{:.0}ms", self.instrument.envelope.attack * 100.),
            1 => format!("{:.0}ms", self.instrument.envelope.release * 100.),
            // 2 => format!("{}", (self.attack_ratio * 1000.0)),
            // 3 => format!("{}", (self.release_ratio * 1000.0)),
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

    fn get_preset_name(&self, index: i32) -> String {
        match index {
            0 => "Sub Bass".to_string(),
            1 => "Pad".to_string(),
            _ => "".to_string(),
        }
    }

    fn set_preset_name(&mut self, name: String) { info!("set_preset_name called: {:?}", name); }

    fn get_preset_num(&self) -> i32 { self.current_preset }

    fn change_preset(&mut self, preset: i32) {
        info!("change_preset: {:?}", preset);
        self.current_preset = preset;
        match preset {
            0 => { self.instrument.envelope.attack = 0.01; self.instrument.envelope.release = 0.05 },
            1 => { self.instrument.envelope.attack = 0.4; self.instrument.envelope.release = 0.6 },
            _ => { self.instrument.envelope.attack = 0.0; self.instrument.envelope.release = 0.05 },
        };
    }
}

plugin_main!(SimpleSynth);
