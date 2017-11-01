use VoiceState;
use Voice;
use types::*;

pub trait Envelope : Sized {
    fn ratio(&self, playhead: Playhead, voice: &Voice, sample_rate: f64) -> f64;
}

/// Simple linear envelope - attack + release only (mainly used to prevent pops)
pub struct SimpleEnvelope {
    pub attack: f64, // attack in ms
    pub release: f64, // release in ms
}

impl Envelope for SimpleEnvelope {

    /// returns a multiply ratio at a given time
    fn ratio(&self, playhead: Playhead, voice: &Voice, sample_rate: f64) -> f64 {

        // guard in case envelope is finished
        // note: convert to std::mem::discriminant when it's stable.
        match voice.state {
            VoiceState::Released(release) => {
                let samples_since_triggered = playhead - release;
                let time_since_triggered = samples_since_triggered as f64 / sample_rate;

                if time_since_triggered > self.release {
                    return 0.0
                }
            },
            _ => ()
        }

        let samples_since_triggered = playhead - voice.started_at;
        let time_since_triggered = samples_since_triggered as f64 / sample_rate;
        let attack_ratio = time_since_triggered / self.attack;
        let release_ratio = match voice.state {
            VoiceState::Released(release_started_at) => {
                let samples_since_triggered = playhead - release_started_at;
                let time_since_triggered = samples_since_triggered as f64 / sample_rate;
                1.0 - time_since_triggered / self.release
            },
            _ => 1.0,
        };

        attack_ratio * release_ratio
    }
}





// Recursive ADSR Envelope.

//use std::ops::Neg;

// Size of VST params. All incoming params are 0.0 - 1.0.
//type VSTParam = f32;

//pub trait Envelope {
//    fn new() -> Self;
//    fn process(&mut self) -> f32;
//}

//
//use time;
//use std;
//
//#[derive(Debug)]
//pub struct ADSR {
//    pub attack_time: f64,
//    pub release_time: f64,
//}
//
//impl ADSR {
//    pub fn gain_ratio(&self, started_at: std::time::Instant) -> f64 {
//        1.0
//    }
//
//    pub fn release_gain_ratio(&self, started_at: std::time::Instant, released_at: std::time::Instant) -> f64 {
//        let time_since_release_in_ms = time::Duration::from_std( std::time::Instant::now() - released_at).unwrap().num_milliseconds() as f64;
//
//        if time_since_release_in_ms < self.attack_time {
////            info!("{:?}", (time_since_release_in_ms, self.attack_time, (self.attack_time - time_since_release_in_ms) / self.attack_time));
//            (self.attack_time - time_since_release_in_ms) / self.attack_time
//        } else {
//            0.0
//        }
//    }
//}
//
//#[test]
//fn it_works() {
//    assert!(true);
//}












//
//#[derive(Clone)]
//pub struct Envelope {
//    // attack_target_ratio: f32, // attack curve
//    attack_coef: f32,
//    attack_base: f32,
//
//    release_coef: f32,
//    release_base: f32,
//
//    pub state: State,
//    pub output: f32,
//}
//
//#[derive(Clone, PartialEq)]
//pub enum State {
//    Idle,
//    Attack,
//    Sustain,
//    Release,
//}
//
//pub fn set_attack_rate(attack_rate: f32, attack_ratio: f32) -> (f32, f32) {
//    let attack_coef = calculate_coefficient(attack_rate, attack_ratio);
//    let attack_base = (1.0 + attack_ratio) * (1.0 - attack_coef);
//    (attack_coef, attack_base)
//}
//
//pub fn set_target_ratio_a(attack_rate: f32, target_ratio: f32) -> (f32, f32) {
//    let attack_coef = calculate_coefficient(attack_rate, target_ratio.max(0.000000001));
//    let attack_base = (1.0 + target_ratio) * (1.0 - attack_coef);
//    (attack_coef, attack_base)
//}
//
//pub fn set_release_rate(release_rate: f32, release_ratio: f32) -> (f32, f32) {
//    let release_coef = calculate_coefficient(release_rate, release_ratio);
//    let release_base = release_ratio.neg() * (1.0 - release_coef);
//    (release_coef, release_base)
//}
//
//fn calculate_coefficient(rate : f32, target_ratio: f32) -> f32 {
//    if rate <= 0.0 {
//        0.0
//    } else {
//        (((1.0 + target_ratio) / target_ratio).ln().neg() / rate).exp()
//    }
//}
//
//impl Envelope {
//
//    /// Construct a new envelope.
//    pub fn new(sample_rate: f32, attack_rate: VSTParam, attack_ratio: VSTParam, release_rate: VSTParam, release_ratio: VSTParam) -> Envelope {
//        let (attack_coef, attack_base) = set_attack_rate(sample_rate * attack_rate, attack_ratio * 100.0);
//        let (release_coef, release_base) = set_release_rate(sample_rate * release_rate, release_ratio * 100.0);
//
//        Envelope {
//            attack_coef: attack_coef,
//            attack_base: attack_base,
//            release_coef: release_coef,
//            release_base: release_base,
//            state: State::Attack,
//            output: 0.0,
//            }
//    }
//
//    pub fn set_attack(&mut self, sample_rate: f32, attack_time: f32) {
//        let (attack_coef, attack_base) = set_attack_rate(sample_rate, attack_time);
//
//        self.attack_coef = attack_coef;
//        self.attack_base = attack_base;
//    }
//
//    /// Return a gain ratio to multiply your input sample against.
//    pub fn process(&mut self) -> f32 {
//        match self.state {
//            State::Attack => self.process_attack(),
//            State::Release => self.process_release(),
//            _ => (),
//        };
//
//        self.output
//    }
//
//    /// Set envelope to release mode.
//    pub fn release(&mut self) {
//        self.state = State::Release;
//    }
//
//    /// Retrigger the envelope (keeping the current output).
//    pub fn retrigger(&mut self) {
//        self.state = State::Attack;
//    }
//
//    fn process_attack(&mut self) {
//        self.output = self.attack_base + self.output * self.attack_coef;
//
//        if self.output >= 1.0 {
//            self.state = State::Sustain;
//            self.output = 1.0
//        }
//    }
//
//    fn process_release(&mut self) {
//        self.output = self.release_base + self.output * self.release_coef;
//        if self.output <= 0.0 {
//            self.output = 0.0;
//            self.state = State::Idle;
//        }
//    }
//}
