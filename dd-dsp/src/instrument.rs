use std;
use std::collections::HashMap;

use midi::midi_note_to_hz;

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

use time;

impl Instrument {

    pub fn new() -> Self {
        Instrument {
            voices: HashMap::new(),
        }
    }


//    // todo: get rid of this and just iterate properly.
//    pub fn next(&mut self) -> Vec<PlayingVoice> {
//        let mut playing_voices = Vec::new();
//
//        // remove dead notes
////        for (note, voice) in self.voices.iter() {
////            match voice.state {
////                VoiceState::Released(released_at) => {
////                    let time_since_release_in_ms = time::Duration::from_std( std::time::Instant::now() - released_at).unwrap().num_milliseconds() as u64;
////                    if voice.release_time_in_ms > time_since_release_in_ms {
////                        self.kill(*note);
////                    }
////                },
////                _ => (),
////            }
////        }
//
//        for (note, mut voice) in self.voices.iter_mut() {
//            playing_voices.push(PlayingVoice{
//                samples_since_start: voice.next(),
//                pitch: midi_note_to_hz(*note),
//                state: voice.state,
//                note: *note,
//            });
//        }
//        playing_voices
//    }

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
        self.voices.remove(&note);

//        use std::collections::hash_map::Entry;
//        if let Entry::Occupied(mut voice) = self.voices.entry(note) {
//            voice.get_mut().state = VoiceState::Released(playhead);
//        }
    }

    fn kill(&mut self, note: MidiNote) {
        info!("note kill {}", note);

        self.voices.remove(&note);
    }
}
