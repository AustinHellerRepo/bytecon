use crate::ByteConverter;
use glam::{Quat, Vec3};

impl ByteConverter for Quat {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        self.z.append_to_bytes(bytes)?;
        self.w.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self::from_xyzw(
            f32::extract_from_bytes(bytes, index)?,
            f32::extract_from_bytes(bytes, index)?,
            f32::extract_from_bytes(bytes, index)?,
            f32::extract_from_bytes(bytes, index)?,
        ))
    }
}

impl ByteConverter for Vec3 {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        self.z.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self::new(
            f32::extract_from_bytes(bytes, index)?,
            f32::extract_from_bytes(bytes, index)?,
            f32::extract_from_bytes(bytes, index)?,
        ))
    }
}