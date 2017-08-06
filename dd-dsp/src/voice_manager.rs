use std;
use std::collections::HashMap;

use midi::midi_note_to_hz;

use types::*;

pub struct VoiceManager {
    // max_voices: usize,
    pub voices: HashMap<MidiNote, Voice>,
    // envelope: E,
}

pub struct Voice {
    pub playhead_position: Playhead, // samples since voice started.
    pub time_of_note_on: std::time::Instant,
}

impl Voice {
    fn next(&mut self) -> Playhead {
        self.playhead_position += 1;
        self.playhead_position - 1
    }
}

#[derive(Debug)]
pub struct PlayingVoice {
    // time_of_note_on: std::time::Instant,
    pub samples_since_start: u64,
    pub gain: f64,
    pub freq: f64,
    // vel: u8,
}

impl VoiceManager {

    pub fn new() -> Self {
        VoiceManager {
            voices: HashMap::new(),
        }
    }

    // pub fn playing_voices(&mut self) -> Vec<ActiveVoice> {
    //     let playing_voices = Vec::new();
    //     for voice in self.voices {
    //         playing_voices.push(ActiveVoice {
    //             time_of_note_on: voice.time_of_note_on,

    //         });
    //     }

    //     playing_voices
    // }

    //     self.voices.map(|voice| (0.0, ))

    // }

    pub fn next(&mut self) -> Vec<PlayingVoice> {
        let mut playing_voices = Vec::new();
        for ref mut voice in self.voices.iter_mut() {
            playing_voices.push(PlayingVoice{
                samples_since_start: voice.1.next(),
                gain: 0.0,
                freq: midi_note_to_hz(*voice.0),
            });
            // voice.1.playhead_position += 1;
        }
        playing_voices
    }

    pub fn note_on(&mut self, note: MidiNote) {

        if self.voices.contains_key(&note) {
            info!("retrig voice {}", note);
            // Note is already playing, retrigger the envelope.
            match self.voices.get_mut(&note) {
                Some(voice) => { /*voice.envelope.retrigger();*/ }
                None => ()
            };
        } else {
            // Create a new voice.
            info!("creating voice {}", note);

            // let keydown_frequency = midi_note_to_hz(note);

            // let voice = Voice {
            //     samples_elapsed: 0,
            //     pitch_in_hz: midi::midi_note_to_hz(note),
            //     released_at: None,
            //     envelope: Envelope::new(self.sample_rate as f32, self.attack_time, self.attack_ratio, self.release_time, self.release_ratio),
            //     oscillator: SineOsc::new(self.sample_rate, midi::midi_note_to_hz(note)),
            //     sampler: sampler,
            // };

            self.voices.insert(note, Voice {
                playhead_position: 0,
                time_of_note_on: std::time::Instant::now(),
            });
        }
    }

    pub fn note_off(&mut self, note: MidiNote) {
        // use std::collections::hash_map::Entry::*;

        self.voices.remove(&note);

        // match self.voices.entry(note) {
        //     Occupied(mut entry) => {
        //         let voice = entry.get_mut();
        //         // voice.envelope.release();
        //         // voice.released_at = Some(voice.samples_elapsed);
        //     }
        //     Vacant(_) => (), // If the note off event doesn't correspond to a voice, don't do anything.
        // }
    }

    // /// Set or reset the number of voices that the Instrument can use.
    // pub fn set_num_voices(&mut self, num_voices: usize) {
    //     if num_voices == 0 {
    //         println!("A Synth must have at least one voice, but the requested number is 0.");
    //     } else {
    //         let len = self.voices.len();
    //         if len < num_voices {
    //             let last_voice = self.voices[len-1].clone();
    //             self.voices.extend(std::iter::repeat(last_voice).take(num_voices - len));
    //         } else if len > num_voices {
    //             self.voices.truncate(num_voices);
    //         }
    //     }
    // }
}

// pub trait Instrument {
//     pub note_on();
//     pub process();
// }