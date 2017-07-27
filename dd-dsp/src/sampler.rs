//! Sample playback.

// use simplelog::*;

use std::fs::File;
use std::io::BufReader;
use std::i16;

use hound;

#[derive(Clone)]
pub struct Sampler {
    sample_rate: f64,
    samples: Vec<i16>,
    position: usize,
}

impl Sampler {
    pub fn new(sample_rate: f64) -> Result<Sampler, String> {

        // let _ = CombinedLogger::init(
        //     vec![
        //         WriteLogger::new(LogLevelFilter::Info, Config::default(), File::create("/tmp/simplesynth.log").unwrap()),
        //     ]
        // );

        match hound::WavReader::open("/Users/rob/snare.wav") {
            Err(why) => {
                Result::Err(why.to_string())
            },
            Ok(mut reader) => {
                let samples: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();
                Ok(Sampler { sample_rate: sample_rate, samples: samples, position: 0 })
            }
        }
    }

    pub fn process(&mut self) -> f32 {
        if self.position < self.samples.len() {
            let sample = self.samples[self.position];
            self.position = self.position + 1;

            let amplitude = i16::MAX as f32;
            return sample as f32 / amplitude
        } else {
            0.0
        }
    }
}
