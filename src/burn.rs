use std::error::Error;
use burn::tensor::{DType, TensorData};
use crate::ByteConverter;

impl ByteConverter for TensorData {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {

        // slice
        self.bytes.append_to_bytes(bytes)?;

        // slice
        self.shape.append_to_bytes(bytes)?;

        // dtype
        self.dtype.append_to_bytes(bytes)?;

        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        
        // slice
        let tensor_data_bytes = Vec::<u8>::extract_from_bytes(bytes, index)?;

        // slice
        let shape = Vec::<usize>::extract_from_bytes(bytes, index)?;

        // dtype
        let dtype = DType::extract_from_bytes(bytes, index)?;

        Ok(TensorData {
            bytes: tensor_data_bytes,
            shape,
            dtype,
        })
    }
}