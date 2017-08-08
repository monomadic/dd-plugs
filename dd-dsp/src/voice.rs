use std;
use VoiceState;
use types::*;

pub struct Voice {
    pub started_at: Playhead, // samples since voice started.
    pub state: VoiceState,
}

impl Voice {
    pub fn reset(&mut self, playhead: Playhead) {
        self.started_at = playhead;
        self.state = VoiceState::Playing;
    }
}
