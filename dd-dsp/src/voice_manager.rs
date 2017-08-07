use std;
use std::collections::HashMap;

use midi::midi_note_to_hz;

use types::*;

use Voice;

pub struct VoiceManager {
    // max_voices: usize,
    pub voices: HashMap<MidiNote, Voice>,
    // envelope: E,
}



#[derive(Debug, Clone, Copy)]
pub enum VoiceState {
    Playing,
    Released(std::time::Instant),
}

#[derive(Debug)]
pub struct PlayingVoice {
    // time_of_note_on: std::time::Instant,
    pub samples_since_start: u64,
    pub gain: f64,
    pub freq: f64,
    pub state: VoiceState,
    // vel: u8,
}

impl VoiceManager {

    pub fn new() -> Self {
        VoiceManager {
            voices: HashMap::new(),
        }
    }

    pub fn next(&mut self) -> Vec<PlayingVoice> {
        let mut playing_voices = Vec::new();
        for ref mut voice in self.voices.iter_mut() {
            playing_voices.push(PlayingVoice{
                samples_since_start: voice.1.next(),
                gain: 0.0,
                freq: midi_note_to_hz(*voice.0),
                state: voice.1.state,
            });
        }
        playing_voices
    }

    pub fn note_on(&mut self, note: MidiNote) {

        if self.voices.contains_key(&note) {
            info!("retrig voice {}", note);
            // Note is already playing, retrigger the envelope.
            match self.voices.get_mut(&note) {
                Some(voice) => { voice.reset(); }
                None => ()
            };
        } else {
            // Create a new voice.
            info!("creating voice {}", note);

            self.voices.insert(note, Voice {
                playhead_position: 0,
                time_of_note_on: std::time::Instant::now(),
                state: VoiceState::Playing,
            });
        }
    }

    pub fn note_off(&mut self, note: MidiNote) {
         use std::collections::hash_map::Entry::*;

         match self.voices.entry(note) {
             Occupied(mut entry) => {
                 let voice = entry.get_mut();
                 voice.state = VoiceState::Released(std::time::Instant::now());
             }
             Vacant(_) => (), // If the note off event doesn't correspond to a voice, don't do anything.
         }
    }

    fn kill(&mut self, note: MidiNote) {
        self.voices.remove(&note);
    }
}
