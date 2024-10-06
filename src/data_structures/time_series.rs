use crate::compression::timestamp_compression::TimestampCompressor;
use crate::compression::value_compression::ValueCompressor;
use std::cell::RefCell;
use std::io::Cursor;

pub struct TimeSeries {
    key: String,
    timestamp_compressor: RefCell<TimestampCompressor<Cursor<Vec<u8>>>>,
    value_compressor: RefCell<ValueCompressor<Cursor<Vec<u8>>>>,
    last_timestamp: u64,
    last_value: f64,
}

impl TimeSeries {
    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn get_last_timestamp(&self) -> u64 {
        self.last_timestamp
    }

    pub fn get_last_value(&self) -> f64 {
        self.last_value
    }
    pub fn new(key: String, initial_timestamp: u64, initial_value: f64) -> Self {
        let compressed_timestamps = Vec::new();
        let timestamp_cursor = Cursor::new(compressed_timestamps);
        let timestamp_compressor = TimestampCompressor::new(initial_timestamp, timestamp_cursor);

        let compressed_values = Vec::new();
        let value_cursor = Cursor::new(compressed_values);
        let value_compressor = ValueCompressor::new(initial_value, value_cursor);

        TimeSeries {
            key,
            timestamp_compressor: RefCell::new(timestamp_compressor),
            value_compressor: RefCell::new(value_compressor),
            last_timestamp: initial_timestamp,
            last_value: initial_value,
        }
    }

    pub fn append(&self, timestamp: u64, value: f64) -> Result<(), Box<dyn std::error::Error>> {
        self.timestamp_compressor
            .borrow_mut()
            .compress(timestamp)
            .unwrap();

        self.value_compressor.borrow_mut().compress(value).unwrap();

        Ok(())
    }

    pub fn finalize(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.timestamp_compressor.borrow_mut().flush().unwrap();
        self.value_compressor.borrow_mut().flush().unwrap();
        Ok(())
    }

    pub fn get_compressed_timestamps(&self) -> Vec<u8> {
        let timestamp_compressor = self.timestamp_compressor.borrow(); 
        let inner_cursor = timestamp_compressor.get_writer(); 
        inner_cursor.get_ref().clone()
    }
    pub fn get_compressed_values(&self) -> Vec<u8> {
        let value_compressor = self.value_compressor.borrow(); 
        let inner_cursor = value_compressor.get_writer(); 
        inner_cursor.get_ref().clone() 
    }
}
