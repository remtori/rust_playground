use std::io::Write;

use audiopus::{coder::Encoder, Application, Channels, SampleRate, TryFrom};
use sfml::audio::SoundRecorder;

use crate::{BUFFER_FRAME_SIZE, CHANNEL_COUNT, SAMPLE_RATE};

pub struct AudioRecorder<W> {
    writer: W,
    inp_buffer: Vec<i16>,
    out_buffer: Vec<u8>,
    encoder: Encoder,
}

impl<W: Write> AudioRecorder<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            inp_buffer: Vec::new(),
            out_buffer: vec![0; BUFFER_FRAME_SIZE * 4],
            encoder: Encoder::new(
                SampleRate::try_from(SAMPLE_RATE as i32).expect("Unsupported sample rate"),
                Channels::try_from(CHANNEL_COUNT as i32).expect("Unsupported channel count"),
                Application::Audio,
            )
            .expect("Failed to create opus encoder"),
        }
    }
}

impl<W: Write> SoundRecorder for AudioRecorder<W> {
    fn on_process_samples(&mut self, samples: &[i16]) -> bool {
        self.inp_buffer.extend_from_slice(samples);

        if self.inp_buffer.len() >= BUFFER_FRAME_SIZE {
            match self
                .encoder
                .encode(&self.inp_buffer[..BUFFER_FRAME_SIZE], &mut self.out_buffer)
            {
                Ok(n_encoded) => {
                    // println!(
                    //     "Encoded {} samples to {} bytes",
                    //     BUFFER_FRAME_SIZE, n_encoded
                    // );

                    self.inp_buffer.drain(0..BUFFER_FRAME_SIZE);

                    self.writer
                        .write_all(&(n_encoded as u32).to_be_bytes())
                        .expect("Io Error");

                    self.writer
                        .write_all(&self.out_buffer[..n_encoded])
                        .expect("Io Error");
                }
                Err(err) => {
                    panic!("Encode Error {:?}", err);
                }
            }
        }

        true
    }
}
