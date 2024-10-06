use crate::utils::BitWriter;
use std::io::{self, Write};


#[derive(Debug)]
pub enum TimestampCompressionError {
    IoError(io::Error),
}

impl From<io::Error> for TimestampCompressionError {
    fn from(err: io::Error) -> Self {
        TimestampCompressionError::IoError(err)
    }
}

pub struct TimestampCompressor<W: Write> {
    bit_writer: BitWriter<W>,
    last_timestamp: u64,
    last_delta: i64,
}

impl<W: Write> TimestampCompressor<W> {
    pub fn new(initial_timestamp: u64, writer: W) -> Self {
        TimestampCompressor {
            bit_writer: BitWriter::new(writer),
            last_timestamp: initial_timestamp,
            last_delta: 0,
        }
    }
    pub fn get_writer(&self) -> &W {
        &self.bit_writer.writer
    }

    pub fn compress(&mut self, timestamp: u64) -> Result<(), TimestampCompressionError> {
        let delta = (timestamp as i64) - (self.last_timestamp as i64);
        let delta_of_delta = delta - self.last_delta;
        self.last_timestamp = timestamp;
        self.last_delta = delta;

        match delta_of_delta {
            0 => {
                self.bit_writer.write_bit(false)?;
            }
            -63..=64 => {
                self.bit_writer.write_bits(0b10, 2)?;
                let value = (delta_of_delta as i64) as i8;
                self.bit_writer.write_bits_signed(value.into(), 7)?;
            }
            -255..=256 => {
                self.bit_writer.write_bits(0b110, 3)?;
                let value = (delta_of_delta as i64) as i16;
                self.bit_writer.write_bits_signed(value.into(), 9)?;
            }
            -2047..=2048 => {
                self.bit_writer.write_bits(0b1110, 4)?;
                let value = (delta_of_delta as i64) as i16;
                self.bit_writer.write_bits_signed(value.into(), 12)?;
            }
            _ => {
                self.bit_writer.write_bits(0b1111, 4)?;
                self.bit_writer.write_bits_signed(delta_of_delta, 32)?;
            }
        }

        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), TimestampCompressionError> {
        self.bit_writer.flush()?;
        Ok(())
    }

    pub fn into_inner(self) -> W {
        self.bit_writer.into_inner()
    }
}
