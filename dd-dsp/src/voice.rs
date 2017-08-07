use std;
use VoiceState;
use types::*;

pub struct Voice {
    pub playhead_position: Playhead, // samples since voice started.
    pub time_of_note_on: std::time::Instant,
    pub state: VoiceState,
    pub release_time_in_ms: u64,
}

impl Voice {
    pub fn next(&mut self) -> Playhead {
        self.playhead_position += 1;
        self.playhead_position - 1
    }

    pub fn reset(&mut self) {
        self.playhead_position = 0;
        self.state = VoiceState::Playing;
        self.time_of_note_on = std::time::Instant::now();
    }
}
