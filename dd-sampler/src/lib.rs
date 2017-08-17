#[macro_use]
extern crate vst2;
#[macro_use]
extern crate log;
extern crate simplelog;

extern crate nfd;

use simplelog::*;
use std::fs::File;

use vst2::buffer::AudioBuffer;
use vst2::plugin::{ Category, Plugin, Info };
use vst2::event::{ Event };
use vst2::editor::Editor;

extern crate dd_dsp;
use dd_dsp::sampler::SampleFile;
use dd_dsp::{ Instrument, midi } ;
use dd_dsp::types::*;

extern crate log_panics;

/// Size of VST params.
type Param = f32;

/// Used for timings of samples (eg position into voice)
type SampleTiming = u64;

struct SimpleSampler {
    playhead: Playhead,
    instrument: Instrument,
    sample: SampleFile,
    sample_rate: f64,
    attack_time: Param,
    release_time: Param,
    attack_ratio: Param,
    release_ratio: Param,
    preset: Vec<u8>,
}

#[derive(Clone)]
struct Voice {
    samples_elapsed: u64,
    pitch_in_hz: f64,

    /// Time when note_off was fired.
    released_at: Option<SampleTiming>,
}

impl Default for SimpleSampler {
    fn default() -> Self {
        use log_panics;
        log_panics::init();
        let _ = CombinedLogger::init(
            vec![
                // TermLogger::new( LevelFilter::Warn, Config::default()).unwrap(),
                WriteLogger::new(LogLevelFilter::Error, Config::default(), File::create("/tmp/simplesynth.log").unwrap()),
            ]
        );
        error!("Loaded dd-sampler.");

//        use nfd::Response;
//        let result = nfd::open_file_dialog(None, None).unwrap_or_else(|e| {
//            panic!(e);
//        });

        SimpleSampler {
            playhead: 0,
            instrument: Instrument::new(),
            sample_rate: 0.0,
            attack_time: 0.02,
            release_time: 0.02,
            attack_ratio: 0.02,
            release_ratio: 0.0001,
            preset: vec![44_u8, 55_u8, 66_u8],
            sample: SampleFile::from_static_file(
                include_bytes!("../../dd-sampler/assets/snare.wav")).unwrap(),
        }
    }
}

impl SimpleSampler {
    fn process_sample(&self, playhead: Playhead) -> Sample {
        let mut output_sample: f64 = 0.0;
        let amplitude = std::u16::MAX as f64;

        for (note, voice) in self.instrument.voices.iter() {
            output_sample += self.sample.sample_at(
                (playhead - voice.started_at),
                midi::midi_note_to_hz(*note),
             ) as f64 / amplitude;
        }

        output_sample as Sample
    }

    fn process_midi_event(&mut self, data: [u8; 3]) {
        match data[0] {
            128 => self.note_off(data[1]),
            144 => self.note_on(data[1]),
            _ => info!("unsupported midi opcode: {}", data[0])
        }
    }

    fn note_on(&mut self, note: u8) {
        if note == 0 {
            error!("changing file...");
            self.sample = SampleFile::from_static_file(
                include_bytes!("../../dd-sampler/assets/bass.wav")).unwrap();

//            use nfd::Response;
//            nfd::open_file_dialog(None, None);

//            let result = nfd::open_file_dialog(None, None);
//                .unwrap_or_else(|e| {
//                error!("{:?}", e);
//            });
//            match result {
//                Response::Okay(file_path) => {
//                    error!("File path = {:?}", file_path);
//                    self.sample = SampleFile::from_path(file_path).unwrap();
//                    error!("loaded.");
//                },
//                Response::OkayMultiple(files) => error!("Files {:?}", files),
//                Response::Cancel => error!("User canceled"),
//            }
        };
        self.instrument.note_on(note, self.playhead);
    }

    fn note_off(&mut self, note: u8) { self.instrument.note_off(note, self.playhead); }
}

impl Plugin for SimpleSampler {

    fn get_info(&self) -> Info {
        Info {
            name: "DD-SimpleSynth".to_string(),
            vendor: "DeathDisco".to_string(),
            unique_id: 6600,
            category: Category::Synth,
            inputs: 0,
            outputs: 1,
            parameters: 4,
            initial_delay: 0,
            preset_chunks: true,
            f64_precision: true,
            silent_when_stopped: false,
            presets: 1,
            version: 0001,
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
            0 => self.attack_time,
            1 => self.release_time,
            2 => self.attack_ratio,
            3 => self.release_ratio,
            _ => 0.0,
        }
    }

    fn get_preset_data(&mut self) -> Vec<u8> {
        error!("[SAVE TO PRESET] get_preset_data");
        self.preset.clone()
    }
    fn get_bank_data(&mut self) -> Vec<u8> {
        error!("[SAVE TO PRESET] get_bank_data");
        self.preset.clone()
    }
    fn load_preset_data(&mut self, data: &[u8]) { error!("[LOAD FROM PRESET] load_preset_data called: {:?}", (data, data[0], data.len())); }
    fn load_bank_data(&mut self, data: &[u8]) { error!("[LOAD FROM PRESET] load_bank_data called: {:?}", (data, data[0], data.len())); }

    fn set_parameter(&mut self, index: i32, value: f32) {
        error!("set_parameter called.");
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
