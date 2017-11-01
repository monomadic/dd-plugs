//use std;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
//use midi::midi_note_to_hz;
use types::*;
use Voice;
use Envelope;

pub struct Instrument<E> where E:Envelope {
    pub voices: HashMap<MidiNote, Voice>,
    pub envelope: E,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VoiceState {
    Playing,
    Released(Playhead),
}

//use time;

impl <E:Envelope> Instrument<E> {

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

    pub fn note_off(&mut self, note: MidiNote, playhead: Playhead) {
        info!("note off {}", note);
        // self.voices.remove(&note);
       if let Entry::Occupied(mut voice) = self.voices.entry(note) {
           voice.get_mut().state = VoiceState::Released(playhead);
       }
    }

//    fn kill(&mut self, note: MidiNote) {
//        info!("note kill {}", note);
//        self.voices.remove(&note);
//    }
}
