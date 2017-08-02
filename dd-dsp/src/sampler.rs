use std::i16;
use hound;
// use basic_dsp;
// use basic_dsp::{ ToComplexVector, SingleBuffer, InterpolationOps };
// use basic_dsp::conv_types::{ SincFunction };

// use log;

#[derive(Clone)]
pub struct Sampler {
    sample_rate: f64,
    unity_pitch: f64,

    sample_count: usize,
    samples: Vec<i16>,
    position: usize,
    scale_factor: f64,
    sample_inc: f64,

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
    pub fn new(sample_rate: f64, playback_pitch: f64) -> Result<Sampler, String> {
        info!("loading new sampler.. playback_pitch: {:?}", playback_pitch);
        const SOUNDFILE: &'static [u8] = include_bytes!("../../dd-sampler/assets/bass.wav");
        let mut wr = match hound::WavReader::new(SOUNDFILE) {
            Err(why) => { return Result::Err(why.to_string()) },
            Ok(reader) => { reader }
        };

        info!("loaded sample. {:?}", wr.spec());
        let channels = wr.spec().channels;
        // let sample_rate = wr.spec().sample_rate;

        let samples: Vec<i16> = wr.samples::<i16>().map(|x|x.expect("Failed to read sample")).collect();

        let unity_pitch = 440.0;
        let scale_factor = playback_pitch / unity_pitch;
        let sample_inc = sample_rate / 10000.0 * scale_factor;
        let sample_count = samples.len();

        // info!("using base sample frequency A4(69): 440.0");
        // info!("A5(81): {:?}", );
        // info!("scale_factor: {:?} sample_inc {:?}", scale_factor, sample_inc);
        
        Ok(Sampler {
            sample_rate: sample_rate,
            unity_pitch: unity_pitch,

            samples: samples,
            position: 0,
            scale_factor: scale_factor,
            sample_inc: sample_inc,
            sample_count: sample_count,
            read_index: 0.0,
        })
    }

    pub fn process(&mut self) -> f32 {
        // let channels = 1.0; // mono

        // get unity note frequency
        // let unity_frequency: f64 = self.unity_pitch / (self.sample_count as f64 / channels); //self.sample_pitch

        // equivalent length
        // let unity_length: f64 = self.sample_rate / unity_frequency;

        if (self.read_index as usize) < self.samples.len() {
            // let next_sample: usize = self.position + self.sample_offset as usize;
            let sample = self.samples[self.read_index as usize];
            self.read_index += self.scale_factor;

            let amplitude = i16::MAX as f32;
            return sample as f32 / amplitude
        } else {
            0.0
        }
    }
}
