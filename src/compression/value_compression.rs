
use crate::utils::BitWriter;
use std::io::{self, Write};

#[derive(Debug)]
pub enum ValueCompressionError {
    IoError(io::Error),
}

impl From<io::Error> for ValueCompressionError {
    fn from(err: io::Error) -> Self {
        ValueCompressionError::IoError(err)
    }
}


pub struct ValueCompressor<W: Write> {
    bit_writer: BitWriter<W>,
    previous_value: u64,
    previous_xor: u32, 
}

impl<W: Write> ValueCompressor<W> {
   
    pub fn new(initial_value: f64, writer: W) -> Self {
        let previous_value = initial_value.to_bits();
        let previous_xor = 0;
        ValueCompressor {
            bit_writer: BitWriter::new(writer),
            previous_value,
            previous_xor,
        }
    }

    pub fn compress(&mut self, value: f64) -> Result<(), ValueCompressionError> {
        let current_bits = value.to_bits();
        let xor = self.previous_value ^ current_bits;

        if xor == 0 {
            self.bit_writer.write_bit(false)?;
        } else {
            self.bit_writer.write_bit(true)?;

            let leading_zeros = xor.leading_zeros() as u8;
            let trailing_zeros = xor.trailing_zeros() as u8;

            if leading_zeros >= (self.previous_xor >> 16) as u8
                && trailing_zeros >= (self.previous_xor & 0xFFFF) as u8
            {
                self.bit_writer.write_bit(false)?;

                let meaningful_bits = 64 - leading_zeros - trailing_zeros;
                self.bit_writer
                    .write_bits(xor as u64, meaningful_bits as usize)?;
            } else {
                self.bit_writer.write_bit(true)?;

                self.bit_writer.write_bits(leading_zeros as u64, 5)?;
                let meaningful_bits = 64 - leading_zeros - trailing_zeros;
                self.bit_writer.write_bits(meaningful_bits as u64, 6)?;

                self.bit_writer
                    .write_bits(xor as u64, meaningful_bits as usize)?;
            }

            self.previous_xor = ((leading_zeros as u32) << 16) | (trailing_zeros as u32);
        }

        self.previous_value = current_bits;
        Ok(())
    }

    pub fn get_writer(&self) -> &W {
        &self.bit_writer.writer
    }


    pub fn flush(&mut self) -> Result<(), ValueCompressionError> {
        self.bit_writer.flush()?;
        Ok(())
    }

    pub fn into_inner(self) -> W {
        self.bit_writer.into_inner()
    }
}
