use std::error::Error;
use crate::ByteConverter;

impl ByteConverter for rustls::Certificate {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.0.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync>> where Self: Sized {
        Ok(Self(Vec::<u8>::extract_from_bytes(bytes, index)?))
    }
}

impl ByteConverter for rustls::PrivateKey {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.0.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync>> where Self: Sized {
        Ok(Self(Vec::<u8>::extract_from_bytes(bytes, index)?))
    }
}