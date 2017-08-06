extern crate hound;
extern crate basic_dsp;
#[macro_use] extern crate log;

pub mod envelope;
pub use envelope::*;

pub mod oscillator;

pub mod midi;
pub use midi::*;

pub mod sampler;

mod sample;
pub use sample::SampleFile;

mod voice_manager;
pub use voice_manager::VoiceManager;

mod types {
    pub type MidiNote = u8;
    pub type Gain = f64;
    pub type Playhead = u64;
    pub type NoteFreq = f64;
}
