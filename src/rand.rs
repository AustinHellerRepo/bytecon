use rand::SeedableRng;
use rand_chacha::{ChaCha12Rng, ChaCha20Rng, ChaCha8Rng};
use crate::{get_multiple_bytes, ByteConverter};

impl ByteConverter for ChaCha8Rng {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let seed: [u8; 32] = self.get_seed();
        bytes.extend_from_slice(&seed);
        Ok(())
    }
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        let seed: [u8; 32] = get_multiple_bytes(bytes, index, 32)?.try_into()?;
        Ok(Self::from_seed(seed))
    }
}

impl ByteConverter for ChaCha12Rng {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let seed: [u8; 32] = self.get_seed();
        bytes.extend_from_slice(&seed);
        Ok(())
    }
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        let seed: [u8; 32] = get_multiple_bytes(bytes, index, 32)?.try_into()?;
        Ok(Self::from_seed(seed))
    }
}

impl ByteConverter for ChaCha20Rng {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let seed: [u8; 32] = self.get_seed();
        bytes.extend_from_slice(&seed);
        Ok(())
    }
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        let seed: [u8; 32] = get_multiple_bytes(bytes, index, 32)?.try_into()?;
        Ok(Self::from_seed(seed))
    }
}