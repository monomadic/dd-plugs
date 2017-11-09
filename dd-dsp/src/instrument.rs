//use std;
// use std::collections::HashMap;
// use std::collections::hash_map::Entry;
//use midi::midi_note_to_hz;
use types::*;
use Voice;
use Envelope;

pub struct Instrument<E> where E:Envelope {
    pub voices: Vec<(MidiNote, Voice)>,
    pub envelope: E,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VoiceState {
    Playing,
    Released(Playhead),
    Retriggered(f64),
}

//use time;

impl <E:Envelope> Instrument<E> {

    pub fn note_on(&mut self, note: MidiNote, playhead: Playhead) {
        self.voices.push((note, Voice {
            started_at: playhead,
            state: VoiceState::Playing,
        }));
    }

    pub fn note_off(&mut self, note: MidiNote, playhead: Playhead) {
        // info!("note off {}", note);

        for &mut (n, ref mut voice) in self.voices.iter_mut() {
            if n == note && voice.state == VoiceState::Playing {
                voice.state = VoiceState::Released(playhead)
            }
        }
    }

    pub fn cleanup(&mut self, playhead: Playhead) {
        // let voices = self.voices.iter().filter(|(n,v) v == VoiceState::Released(_)|).collect::<Vec<_>>();

        // for (index, &mut (n, ref mut voice)) in self.voices.iter_mut().enumerate() {
        //     match voice.state {
        //         VoiceState::Released(release_time) => {
        //             self.voices.remove(index);
        //         },
        //         _ => (),
        //     }
        // }

        let envelope = &self.envelope;

        self.voices.retain(|&(_, ref v)|
            // v.state == VoiceState::Playing
            match v.state {
                VoiceState::Released(release_time) => {
                    envelope.expired(playhead, release_time)
                },
                _ => true,
            }
        );
    }

//    fn kill(&mut self, note: MidiNote) {
//        info!("note kill {}", note);
//        self.voices.remove(&note);
//    }
}
