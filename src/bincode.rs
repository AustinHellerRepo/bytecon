use serde::{Deserialize, Serialize};
use crate::ByteConverter;

impl<T: Serialize + for<'de> Deserialize<'de>> ByteConverter for T {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let serialized_bytes = bincode::serialize(self)?;
        serialized_bytes.append_to_bytes(bytes);
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        let serialized_bytes = Vec::<u8>::extract_from_bytes(bytes, index)?;
        let deserialized_self: Self = bincode::deserialize(&serialized_bytes)?;
        Ok(deserialized_self)
    }
}