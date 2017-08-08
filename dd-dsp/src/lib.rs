extern crate hound;
extern crate basic_dsp;
#[macro_use] extern crate log;

extern crate time;

pub mod envelope;
pub use envelope::*;

pub mod oscillator;

pub mod midi;
pub use midi::*;

//pub mod sampler;

mod sample;

pub mod sampler {
//    pub use sample::Sample;
    pub use sample::SampleFile;
}

mod instrument;
pub use instrument::{Instrument, VoiceState };
// pub use voice_manager::PlayingVoice;

mod voice;
pub use voice::Voice;

pub mod types {
    pub type MidiNote = u8;
    pub type Gain = f64;
    pub type Playhead = u64;
    pub type NoteFreq = f64;
    /// Size of samples.
    pub type Sample = f32;
}
