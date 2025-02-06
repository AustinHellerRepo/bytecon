use crate::ByteConverter;
use bevy::math::Affine3;
use glam::{Affine2, BVec2, BVec3, BVec3A, Mat2, Mat3, Quat, Vec2, Vec3};

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

impl ByteConverter for Vec2 {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self::new(
            f32::extract_from_bytes(bytes, index)?,
            f32::extract_from_bytes(bytes, index)?,
        ))
    }
}

impl ByteConverter for Mat2 {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x_axis.append_to_bytes(bytes)?;
        self.y_axis.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self::from_cols(
            Vec2::extract_from_bytes(bytes, index)?,
            Vec2::extract_from_bytes(bytes, index)?,
        ))
    }
}

impl ByteConverter for Mat3 {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x_axis.append_to_bytes(bytes)?;
        self.y_axis.append_to_bytes(bytes)?;
        self.z_axis.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self::from_cols(
            Vec3::extract_from_bytes(bytes, index)?,
            Vec3::extract_from_bytes(bytes, index)?,
            Vec3::extract_from_bytes(bytes, index)?,
        ))
    }
}

impl ByteConverter for Affine2 {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.matrix2.append_to_bytes(bytes)?;
        self.translation.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            matrix2: Mat2::extract_from_bytes(bytes, index)?,
            translation: Vec2::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for Affine3 {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.matrix3.append_to_bytes(bytes)?;
        self.translation.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            matrix3: Mat3::extract_from_bytes(bytes, index)?,
            translation: Vec3::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for BVec2 {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            x: bool::extract_from_bytes(bytes, index)?,
            y: bool::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for BVec3 {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        self.z.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            x: bool::extract_from_bytes(bytes, index)?,
            y: bool::extract_from_bytes(bytes, index)?,
            z: bool::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for BVec3A {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let bitmask = self.bitmask();
        let x = (bitmask & (1 << 0)) != 0;
        let y = (bitmask & (1 << 1)) != 0;
        let z = (bitmask & (1 << 2)) != 0;
        x.append_to_bytes(bytes)?;
        y.append_to_bytes(bytes)?;
        z.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self::new(
            bool::extract_from_bytes(bytes, index)?,
            bool::extract_from_bytes(bytes, index)?,
            bool::extract_from_bytes(bytes, index)?,
        ))
    }
}