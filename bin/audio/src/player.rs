use audiopus::{coder::Decoder, Channels, SampleRate, TryFrom};
use sfml::audio::SoundStream;
use std::{
    convert::TryInto,
    io::{ErrorKind, Read},
    mem::size_of,
};

use crate::{BUFFER_FRAME_SIZE, CHANNEL_COUNT, SAMPLE_RATE};

pub struct AudioStream<R> {
    reader: R,
    n_read: usize,
    expect_len: Option<usize>,
    inp_buffer: Vec<u8>,
    out_buffer: Vec<i16>,
    decoder: Decoder,
}

impl<R: Read> AudioStream<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            n_read: 0,
            expect_len: None,
            inp_buffer: vec![0; BUFFER_FRAME_SIZE],
            out_buffer: vec![0; BUFFER_FRAME_SIZE],
            decoder: Decoder::new(
                SampleRate::try_from(SAMPLE_RATE as i32).expect("Unsupported sample rate"),
                Channels::try_from(CHANNEL_COUNT as i32).expect("Unsupported channel count"),
            )
            .expect("Failed to create opus decoder"),
        }
    }
}

impl<R: Read> SoundStream for AudioStream<R> {
    fn get_data(&mut self) -> (&mut [i16], bool) {
        const U32_SIZE: usize = size_of::<u32>();

        loop {
            match self.reader.read(&mut self.inp_buffer[self.n_read..]) {
                Ok(n_read) => {
                    self.n_read += n_read;
                    // println!("Read buffer: {}, n read: {}", self.n_read, n_read);

                    if self.n_read < U32_SIZE {
                        continue;
                    }

                    let len = match self.expect_len.as_ref() {
                        Some(len) => *len,
                        None => {
                            let len = u32::from_be_bytes(
                                self.inp_buffer[0..U32_SIZE].try_into().unwrap(),
                            ) as usize;

                            // println!("Read len: {}", len);
                            self.expect_len = Some(len);
                            len
                        }
                    };

                    if self.n_read < len + U32_SIZE {
                        continue;
                    }

                    let n_decoded = match self.decoder.decode(
                        Some(&self.inp_buffer[U32_SIZE..len + U32_SIZE]),
                        &mut self.out_buffer,
                        false,
                    ) {
                        Ok(n_decoded) => {
                            let n_decoded = n_decoded * CHANNEL_COUNT as usize;

                            // println!("Decoded {} bytes to {} samples", n_read, n_decoded);
                            Some(n_decoded)
                        }
                        Err(err) => {
                            println!("Decode Error {:?}", err);
                            None
                        }
                    };

                    self.expect_len.take();
                    self.inp_buffer.rotate_left(len + U32_SIZE);
                    self.n_read -= len + U32_SIZE;

                    // println!("Decoded {} => {:?}", len, n_decoded);

                    match n_decoded {
                        Some(n_decoded) => break (&mut self.out_buffer[..n_decoded], true),
                        None => break (&mut [], false),
                    }
                }
                Err(err) => {
                    println!("Io Error {:?}", err);
                    if matches!(err.kind(), ErrorKind::Other) {
                        continue;
                    }

                    break (&mut [], false);
                }
            }
        }
    }

    fn seek(&mut self, offset: sfml::system::Time) {
        println!("Seek to {} ms", offset.as_milliseconds());
    }

    fn channel_count(&self) -> u32 {
        CHANNEL_COUNT
    }

    fn sample_rate(&self) -> u32 {
        SAMPLE_RATE
    }
}
