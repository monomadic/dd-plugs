use std::i16;
// use hound;
// use basic_dsp;
// use basic_dsp::{ ToComplexVector, SingleBuffer, InterpolationOps };
// use basic_dsp::conv_types::{ SincFunction };

// use log;

// #[derive(Clone)]
// pub struct Voice {
//     read_index: f64, // current read location, double as we will have fractional locations.
//     scale_factor: f64,
//     sample_inc: f64,
// }

// #[derive(Clone)]
// pub struct Mapping {}

use types::*;

use SampleFile;
use VoiceManager;

pub struct Sampler {
    unity_pitch: f64,

    sample_file: SampleFile,
    voice_manager: VoiceManager,

    scale_factor: f64,
    sample_inc: f64,

    output_channels: u16,
    output_sample_rate: f64,

    read_index: f64, // current read location, double as we will have fractional locations.
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
        // info!("loading new sampler.. playback_pitch: {:?}", playback_pitch);
        // const SOUNDFILE: &'static [u8] = include_bytes!("../../dd-sampler/assets/bass.wav");
        // let mut wr = match hound::WavReader::new(SOUNDFILE) {
        //     Err(why) => { return Result::Err(why.to_string()) },
        //     Ok(reader) => { reader }
        // };

        let sample = SampleFile::from_static_file(include_bytes!("../../dd-sampler/assets/bass.wav")).unwrap();

        // info!("loaded sample. {:?}, len: {:?}", wr.spec(), sample.frames.len());
        // let sample_rate = wr.spec().sample_rate;

        // let samples: Vec<i16> = wr.samples::<i16>().map(|x|x.expect("Failed to read sample")).collect();
        let playback_pitch = 100.0;
        let unity_pitch = 440.0;
        let scale_factor = playback_pitch / unity_pitch;
        let sample_inc = sample_rate / 10000.0 * scale_factor;

        // info!("using base sample frequency A4(69): 440.0");
        // info!("A5(81): {:?}", );
        // info!("scale_factor: {:?} sample_inc {:?}", scale_factor, sample_inc);

        info!("loaded single sample.");
        
        Ok(Sampler {
            output_sample_rate: sample_rate,
            unity_pitch: unity_pitch,
            // samples: samples,
            sample_file: sample,

            // samples: vec!(sample),
            scale_factor: scale_factor,
            sample_inc: sample_inc,
            read_index: 0.0,
            output_channels: 2,
            voice_manager: VoiceManager::new(),
        })
    }

    pub fn note_on(&mut self, note: MidiNote) { self.voice_manager.note_on(note) }

    pub fn process(&mut self) -> f32 {
        for voice in self.voice_manager.next() {
            info!("{:?}", voice);
        }

        0.0

        // for voice in voices {
        //     let sample = self.sample_file.samples[voice.next_position()];
        //     // voice.1.playhead_position += 1;
        //     let amplitude = i16::MAX as f32;
        //     output_samples.push(sample as f32 / amplitude);
        // }

        // *output_samples.first().unwrap()

        // if (self.read_index as usize) < self.sample.frames.len() {
        //     let sample = self.sample.frames[self.read_index as usize];
        //     self.read_index += self.scale_factor;

        //     let amplitude = i16::MAX as f32;
        //     info!("sample");
        //     return sample as f32 / amplitude
        // } else {
        //     0.0
        // }
    }
}
