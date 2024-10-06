pub mod timestamp_compression;
pub mod value_compression;
pub trait Compressor {
    type Output;
    type Error;


    fn compress(&mut self, data: Self::Output) -> Result<(), Self::Error>;


    fn finalize(&mut self) -> Result<(), Self::Error>;
}
