use std::io::{self, Write};

pub struct BitWriter<W: Write> {
    pub writer: W,
    current_byte: u8,
    bits_filled: u8, 
}

impl<W: Write> BitWriter<W> {

    pub fn new(writer: W) -> Self {
        BitWriter {
            writer,
            current_byte: 0,
            bits_filled: 0,
        }
    }

    pub fn into_inner(self) -> W {
        self.writer
    }

    pub fn get_ref(&self) -> &W {
        &self.writer
    }


    pub fn write_bit(&mut self, bit: bool) -> io::Result<()> {
        self.current_byte <<= 1;
        if bit {
            self.current_byte |= 1;
        }
        self.bits_filled += 1;

        if self.bits_filled == 8 {
            self.flush_byte()?;
        }

        Ok(())
    }

    pub fn write_bits(&mut self, bits: u64, count: usize) -> io::Result<()> {
        for i in (0..count).rev() {
            let bit = ((bits >> i) & 1) != 0;
            self.write_bit(bit)?;
        }
        Ok(())
    }

    pub fn write_bits_signed(&mut self, value: i64, count: usize) -> io::Result<()> {
        let bits = if value < 0 {
            (value as u64).wrapping_neg()
        } else {
            value as u64
        };
        self.write_bits(bits, count)
    }


    fn flush_byte(&mut self) -> io::Result<()> {
        self.writer.write_all(&[self.current_byte])?;
        self.current_byte = 0;
        self.bits_filled = 0;
        Ok(())
    }

    pub fn flush(&mut self) -> io::Result<()> {
        if self.bits_filled > 0 {
            self.current_byte <<= 8 - self.bits_filled;
            self.writer.write_all(&[self.current_byte])?;
            self.current_byte = 0;
            self.bits_filled = 0;
        }
        self.writer.flush()
    }
}
