use std::error::Error;
use crate::ByteConverter;

impl ByteConverter for rustls::Certificate {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        self.0.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self(Vec::<u8>::extract_from_bytes(bytes, index)?))
    }
}

impl ByteConverter for rustls::PrivateKey {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        self.0.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self(Vec::<u8>::extract_from_bytes(bytes, index)?))
    }
}