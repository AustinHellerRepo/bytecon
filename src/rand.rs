use std::error::Error;
use rand::SeedableRng;
use rand_chacha::{ChaCha12Rng, ChaCha20Rng, ChaCha8Rng};
use crate::{get_multiple_bytes, ByteConverter};

impl ByteConverter for ChaCha8Rng {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let seed: [u8; 32] = self.get_seed();
        bytes.extend_from_slice(&seed);
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        let seed: [u8; 32] = get_multiple_bytes(bytes, index, 32)?.try_into()?;
        Ok(Self::from_seed(seed))
    }
}

impl ByteConverter for ChaCha12Rng {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let seed: [u8; 32] = self.get_seed();
        bytes.extend_from_slice(&seed);
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        let seed: [u8; 32] = get_multiple_bytes(bytes, index, 32)?.try_into()?;
        Ok(Self::from_seed(seed))
    }
}

impl ByteConverter for ChaCha20Rng {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let seed: [u8; 32] = self.get_seed();
        bytes.extend_from_slice(&seed);
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        let seed: [u8; 32] = get_multiple_bytes(bytes, index, 32)?.try_into()?;
        Ok(Self::from_seed(seed))
    }
}