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
use vst2::editor::Editor;

extern crate dd_dsp;
//use dd_dsp::oscillator;
use dd_dsp::sampler::{ Sampler };

extern crate log_panics;

/// Size of VST params.
type Param = f32;

/// Used for timings of samples (eg position into voice)
type SampleTiming = u64;

struct SimpleSampler {
    sample_rate: f64,
    attack_time: Param,
    release_time: Param,
    attack_ratio: Param,
    release_ratio: Param,
    sampler: Sampler,
}

#[derive(Clone)]
struct Voice {
    samples_elapsed: u64,
    pitch_in_hz: f64,

//    oscillator: SineOsc,

    /// Time when note_off was fired.
    released_at: Option<SampleTiming>,
}

impl Default for SimpleSampler {
    fn default() -> SimpleSampler {
        SimpleSampler {
            sample_rate: 0.0,
            attack_time: 0.02,
            release_time: 0.02,
            attack_ratio: 0.02,
            release_ratio: 0.0001,
            sampler: Sampler::new(44100.0).expect("sampler should initialise"),
        }
    }
}

impl SimpleSampler {
    fn process_midi_event(&mut self, data: [u8; 3]) {
        match data[0] {
            128 => self.note_off(data[1]),
            144 => self.note_on(data[1]),
            _ => info!("unsupported midi opcode: {}", data[0])
        }
    }

    fn note_on(&mut self, note: u8) { self.sampler.note_on(note); }
    fn note_off(&mut self, note: u8) { self.sampler.note_off(note); }
}

impl Plugin for SimpleSampler {
    fn get_info(&self) -> Info {
        use log_panics;
        log_panics::init();
        let _ = CombinedLogger::init(
            vec![
                // TermLogger::new( LevelFilter::Warn, Config::default()).unwrap(),
                WriteLogger::new(LogLevelFilter::Info, Config::default(), File::create("/tmp/simplesynth.log").unwrap()),
            ]
        );

        Info {
            name: "DD-SimpleSynth".to_string(),
            vendor: "DeathDisco".to_string(),
            unique_id: 6666,
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
    //        self.sampler.process_buffer(*output_buffer);

    fn process(&mut self, buffer: AudioBuffer<f32>) {
        let (_, mut output_buffer) = buffer.split();
        self.sampler.process_buffer(&mut output_buffer);

//        for output_channel in *output_buffer {
//            for output_sample in output_channel.iter_mut() {
//                *output_sample = self.process()
//            }
//        };
    }

    fn set_sample_rate(&mut self, rate: f32) { 
        info!("sample rate is assigned to {}", rate);
        self.sample_rate = rate as f64;
    }

    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.attack_time,
            1 => self.release_time,
            2 => self.attack_ratio,
            3 => self.release_ratio,
            _ => 0.0,
        }
    }

    fn set_parameter(&mut self, index: i32, value: f32) {
        match index {
            0 => self.attack_time = value.max(0.001), // avoid pops by always having at least a tiny attack.
            1 => self.release_time = value.max(0.001), // same with release.
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
            0 => format!("{}ms", (self.attack_time * 100.0)),
            1 => format!("{}ms", (self.release_time * 100.0)),
            2 => format!("{}", (self.attack_ratio * 100.0)),
            3 => format!("{}", (self.release_ratio * 100.0)),
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
    fn get_editor(&mut self) -> Option<&mut Editor> { None }
}

plugin_main!(SimpleSampler);
