//use std;
use std::collections::HashMap;
//use midi::midi_note_to_hz;
use types::*;
use Voice;

pub struct Instrument {
    pub voices: HashMap<MidiNote, Voice>,
}

#[derive(Debug, Clone, Copy)]
pub enum VoiceState {
    Playing,
    Released(Playhead),
}

//use time;

impl Instrument {

    pub fn new() -> Self {
        Instrument {
            voices: HashMap::new(),
        }
    }

    pub fn note_on(&mut self, note: MidiNote, playhead: Playhead) {
        if self.voices.contains_key(&note) {
            info!("retrig voice {}", note);
            // Note is already playing, retrigger the envelope.
            match self.voices.get_mut(&note) {
                Some(voice) => { voice.reset(playhead); }
                None => ()
            };
        } else {
            // Create a new voice.
            info!("creating voice {}", note);
            self.voices.insert(note, Voice {
                started_at: playhead,
                state: VoiceState::Playing,
            });
        }
    }

    pub fn note_off(&mut self, note: MidiNote, _: Playhead) {
        info!("note off {}", note);
        self.voices.remove(&note);
//        use std::collections::hash_map::Entry;
//        if let Entry::Occupied(mut voice) = self.voices.entry(note) {
//            voice.get_mut().state = VoiceState::Released(playhead);
//        }
    }

//    fn kill(&mut self, note: MidiNote) {
//        info!("note kill {}", note);
//        self.voices.remove(&note);
//    }
}
