use hound;
use std::io::{BufReader, Read};

#[derive(Clone)]
pub struct SampleFile {
    pub sample_rate: f64,
    pub unity_pitch: f64,
    pub samples: Vec<i16>,
}

impl SampleFile {
    pub fn from_static_file(file: &'static [u8]) -> Result<Self, String> {
        // let samples: Vec<i16> = reader.samples::<i16>().map(|x|x.expect("Failed to read sample")).collect();
        match hound::WavReader::new(BufReader::new(&file[..])) {
            Ok(samplefile) => SampleFile::from_wavreader(samplefile),
            Err(why) => Err(format!("{:?}", why)),
        }
    }

    fn from_wavreader<R:Read>(mut reader: hound::WavReader<BufReader<R>>) -> Result<Self, String> {
            let samples: Vec<i16> = reader.samples::<i16>().map(|x|x.expect("Failed to read sample")).collect();

            Ok(SampleFile {
                sample_rate: reader.spec().sample_rate as f64,
                unity_pitch: 440.0,
                samples: samples,
            })
    }

    /// Gives a length of a resized sample at a specific pitch.
    pub fn len_for_freq(&self, freq: f64) -> usize {
        let unity_freq = self.unity_pitch;
        let scale_factor = freq / unity_freq;
        (self.samples.len() as f64 * scale_factor) as usize
    }

    /// Returns the relative sample for pos at a specific pitch.
    pub fn sample_at(&mut self, pos: usize, freq: f64) -> f64 {
        let unity_freq = self.unity_pitch;
        let scale_factor = freq / unity_freq;

        let new_samplerate_ratio = self.sample_rate / 10000.0 * scale_factor;
        let new_pos = ((pos as f64) * new_samplerate_ratio) as usize;

        if self.samples.len() > new_pos {
            self.samples[new_pos] as f64
        } else { 0.0 }
    }
}
