use std::i16;
use std;

// use basic_dsp;
// use basic_dsp::{ ToComplexVector, SingleBuffer, InterpolationOps };
// use basic_dsp::conv_types::{ SincFunction };

// #[derive(Clone)]
// pub struct Mapping {}

use types::*;
use envelope;

use SampleFile;
use VoiceManager;
use VoiceState;

pub struct Sampler {
    sample_file: SampleFile,
    voice_manager: VoiceManager,
    output_channels: u16,
    output_sample_rate: f64,
    envelope: envelope::ADSR,
}

// fn change_speed(samples: Vec<i16>) -> Vec<i16> {
//     // let sample_cache: Vec<f32> = samples.iter().map(|x| *x as f32).collect();

//     info!("samples: {:?}", samples);
//     // let mut repitched_samples = samples.to_complex_time_vec();
//     info!("repitched_samples: {:?}", repitched_samples);
//     let mut buffer = SingleBuffer::new();
//     let function = SincFunction::new();
//     repitched_samples.interpolatef(&mut buffer, &function, 1.5, 0.0, 10);

//     let mut converted_samples: Vec<i16> = Vec::new();
//     for sample in &repitched_samples[..] {
//         let amplitude = i16::MAX as f32;
//         let sample = (sample * amplitude) as i16;
//         info!("{:?}", sample);
//         converted_samples.push(sample as i16);
//     }

//     info!("converted_samples: {:?}", converted_samples);
//     converted_samples
// }

impl Sampler {
    pub fn new(sample_rate: f64) -> Result<Sampler, String> {
        let sample = SampleFile::from_static_file(include_bytes!("../../dd-sampler/assets/bass.wav")).unwrap();

        Ok(Sampler {
            output_sample_rate: sample_rate,
            sample_file: sample,
            output_channels: 2,
            voice_manager: VoiceManager::new(),
            envelope: envelope::ADSR{
                attack_time: 90.0,
                release_time: 90.0,
            }
        })
    }

    pub fn note_on(&mut self, note: MidiNote) { self.voice_manager.note_on(note) }
    pub fn note_off(&mut self, note: MidiNote) { self.voice_manager.note_off(note, self.envelope.release_time as u64) }

    pub fn process(&mut self) -> f32 {
        let mut output_sample: f64 = 0.0;
        let voices = self.voice_manager.next();
        for playing_sample in voices {
            let pos = playing_sample.samples_since_start as usize;

//            let envelope_gain = match playing_sample.state {
//                VoiceState::Playing =>  {
//                    self.envelope.gain_ratio(std::time::Instant::now())
//                },
//                VoiceState::Released(release_time) => {
//                    self.envelope.release_gain_ratio(std::time::Instant::now(), release_time)
//                }
//            };
//            info!("{:?}", envelope_gain);

            if self.sample_file.samples.len() > pos {
                output_sample += self.sample_file.sample_at(pos, playing_sample.pitch); // * envelope_gain;
            }
        }
        let amplitude = i16::MAX as f32;
        return output_sample as f32 / amplitude;
    }
}
