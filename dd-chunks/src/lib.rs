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

struct Chunker {
}

impl Default for Chunker {
    fn default() -> Self {
        let _ = CombinedLogger::init(
            vec![
                // TermLogger::new( LevelFilter::Warn, Config::default()).unwrap(),
                WriteLogger::new(LogLevelFilter::Info, Config::default(), File::create("/tmp/simplesynth.log").unwrap()),
            ]
        );
        info!("Loaded dd-chunks...");
        Self {
        }

    }
}

impl Chunker {
}

impl Plugin for Chunker {
    fn get_preset_data(&mut self) -> Vec<u8> { let r: Vec<u8> = (0..1000).collect(); info!("[SAVE TO PRESET] get_preset_data called: {:?}", r); r }
    fn get_bank_data(&mut self) -> Vec<u8> { let r: Vec<u8> = (0..1000).collect(); info!("[SAVE TO PRESET] get_bank_data called: {:?}", r); r }
    fn load_preset_data(&mut self, data: &[u8]) { info!("[LOAD FROM PRESET] load_preset_data called: {:?}", data); }
    fn load_bank_data(&mut self, data: &[u8]) { info!("[LOAD FROM PRESET] load_bank_data called: {:?}", data); }

    fn get_info(&self) -> Info {
        Info {
            name: "DD-Chunker".to_string(),
            vendor: "DeathDisco".to_string(),
            unique_id: 6644,
            category: Category::Synth,
            inputs: 0,
            outputs: 1,
            parameters: 0,
            initial_delay: 0,
            preset_chunks: true,
            ..Info::default()
        }
    }
}

plugin_main!(Chunker);
