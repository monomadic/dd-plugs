#[macro_use] 
extern crate vst2;

#[macro_use] 
extern crate log;
extern crate simplelog;

use simplelog::*;
use std::fs::File;

#[macro_use]
extern crate serde_derive;
extern crate bincode;

use bincode::{serialize, deserialize, Infinite};

use vst2::buffer::AudioBuffer;
use vst2::plugin::{Category, Plugin, Info};
use vst2::event::{Event};

/// Size of VST params.
type Param = f32;

struct Chunker {
    preset: ChunkerPreset,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct ChunkerPreset {
    volume: Param,
}

impl Default for Chunker {
    fn default() -> Self {
        let _ = CombinedLogger::init(
            vec![
                WriteLogger::new(LogLevelFilter::Error, Config::default(), File::create("/tmp/simplesynth.log").unwrap()),
            ]
        );
        info!("Loaded dd-chunks...");
        Self {
            preset: ChunkerPreset{ volume: 0.5 }
        }
    }
}
use vst2::editor::*;

impl Plugin for Chunker {

    fn get_preset_data(&mut self) -> Vec<u8> {
        error!("[SAVE TO PRESET] get_preset_data: {:?}", self.preset);
        let encoded: Vec<u8> = serialize(&self.preset, Infinite).unwrap();

        encoded
    }

    fn get_bank_data(&mut self) -> Vec<u8> {
        error!("[SAVE TO PRESET] get_bank_data: {:?}", self.preset);
        self.get_preset_data()
    }

    fn load_preset_data(&mut self, data: &[u8]) {
        let decoded: ChunkerPreset = deserialize(&data[..]).unwrap();
        error!("[LOAD FROM PRESET] load_preset_data called: {:?}", decoded);

        self.preset = decoded;
    }

    fn load_bank_data(&mut self, data: &[u8]) {
        self.load_preset_data(data);
        error!("[LOAD FROM PRESET] load_bank_data called: {:?}", self.preset);
    }

    fn get_preset_num(&self) -> i32 { 0 }
    fn change_preset(&mut self, preset: i32) { }

    fn get_preset_name(&self, preset: i32) -> String {
        match preset {
            0 => "Preset 1".to_string(),
            1 => "Preset 2".to_string(),
            2 => "Preset 3".to_string(),
            _ => "Other Preset".to_string(),
        }
    }

    fn set_preset_name(&mut self, name: String) { }

    fn get_info(&self) -> Info {
        Info {
            name: "DD-Chunker".to_string(),
            vendor: "DeathDisco".to_string(),
            unique_id: 99887766,
            category: Category::Synth,
            inputs: 0,
            outputs: 1,
            parameters: 1,
            initial_delay: 0,
            preset_chunks: true,
            ..Info::default()
        }
    }

    fn get_parameter(&self, index: i32) -> f32 { self.preset.volume }
    fn set_parameter(&mut self, index: i32, value: f32) { self.preset.volume = value }

    fn can_be_automated(&self, index: i32) -> bool { error!("can_be_automated called"); true }

    // Some hosts, like Bitwig, break the VST2.4 spec and won't call getChunk/setChunk unless
    // a gui is present or params have been altered prior to saving a preset.
    fn get_editor(&mut self) -> Option<&mut Editor> { Some(self) }

}

impl Editor for Chunker {
    fn size(&self) -> (i32, i32) { (200, 100) }
    fn position(&self) -> (i32, i32) { (0, 0) }
    fn open(&mut self, window: *mut std::os::raw::c_void) {}
    fn is_open(&mut self) -> bool { true }
}

plugin_main!(Chunker);
