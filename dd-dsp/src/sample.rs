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
}