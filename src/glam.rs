use crate::ByteConverter;
use glam::{Affine2, BVec2, BVec3, BVec3A, BVec4, DAffine2, DAffine3, DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4, I16Vec2, I16Vec3, I16Vec4, I64Vec2, I64Vec3, I64Vec4, I8Vec2, I8Vec3, I8Vec4, IVec2, IVec3, IVec4, Mat2, Mat3, Mat3A, Mat4, Quat, U16Vec2, U16Vec3, U16Vec4, U64Vec2, U64Vec3, U64Vec4, U8Vec2, U8Vec3, U8Vec4, UVec2, UVec3, UVec4, Vec2, Vec3, Vec3A, Vec4};

impl ByteConverter for Quat {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        self.z.append_to_bytes(bytes)?;
        self.w.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self::from_xyzw(
            f32::extract_from_bytes(bytes, index)?,
            f32::extract_from_bytes(bytes, index)?,
            f32::extract_from_bytes(bytes, index)?,
            f32::extract_from_bytes(bytes, index)?,
        ))
    }
}

impl ByteConverter for Vec3 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        self.z.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self::new(
            f32::extract_from_bytes(bytes, index)?,
            f32::extract_from_bytes(bytes, index)?,
            f32::extract_from_bytes(bytes, index)?,
        ))
    }
}

impl ByteConverter for Vec2 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self::new(
            f32::extract_from_bytes(bytes, index)?,
            f32::extract_from_bytes(bytes, index)?,
        ))
    }
}

impl ByteConverter for Mat2 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x_axis.append_to_bytes(bytes)?;
        self.y_axis.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self::from_cols(
            Vec2::extract_from_bytes(bytes, index)?,
            Vec2::extract_from_bytes(bytes, index)?,
        ))
    }
}

impl ByteConverter for Mat3 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x_axis.append_to_bytes(bytes)?;
        self.y_axis.append_to_bytes(bytes)?;
        self.z_axis.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self::from_cols(
            Vec3::extract_from_bytes(bytes, index)?,
            Vec3::extract_from_bytes(bytes, index)?,
            Vec3::extract_from_bytes(bytes, index)?,
        ))
    }
}

impl ByteConverter for Affine2 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.matrix2.append_to_bytes(bytes)?;
        self.translation.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            matrix2: Mat2::extract_from_bytes(bytes, index)?,
            translation: Vec2::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for BVec2 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            x: bool::extract_from_bytes(bytes, index)?,
            y: bool::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for BVec3 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        self.z.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            x: bool::extract_from_bytes(bytes, index)?,
            y: bool::extract_from_bytes(bytes, index)?,
            z: bool::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for BVec3A {
    #[inline(always)]
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
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self::new(
            bool::extract_from_bytes(bytes, index)?,
            bool::extract_from_bytes(bytes, index)?,
            bool::extract_from_bytes(bytes, index)?,
        ))
    }
}

impl ByteConverter for BVec4 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        self.z.append_to_bytes(bytes)?;
        self.w.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            x: bool::extract_from_bytes(bytes, index)?,
            y: bool::extract_from_bytes(bytes, index)?,
            z: bool::extract_from_bytes(bytes, index)?,
            w: bool::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for DVec2 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            x: f64::extract_from_bytes(bytes, index)?,
            y: f64::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for DMat2 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x_axis.append_to_bytes(bytes)?;
        self.y_axis.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            x_axis: DVec2::extract_from_bytes(bytes, index)?,
            y_axis: DVec2::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for DAffine2 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.matrix2.append_to_bytes(bytes)?;
        self.translation.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            matrix2: DMat2::extract_from_bytes(bytes, index)?,
            translation: DVec2::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for DVec3 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        self.z.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            x: f64::extract_from_bytes(bytes, index)?,
            y: f64::extract_from_bytes(bytes, index)?,
            z: f64::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for DMat3 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x_axis.append_to_bytes(bytes)?;
        self.y_axis.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            x_axis: DVec3::extract_from_bytes(bytes, index)?,
            y_axis: DVec3::extract_from_bytes(bytes, index)?,
            z_axis: DVec3::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for DAffine3 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.matrix3.append_to_bytes(bytes)?;
        self.translation.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            matrix3: DMat3::extract_from_bytes(bytes, index)?,
            translation: DVec3::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for DVec4 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        self.z.append_to_bytes(bytes)?;
        self.w.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            x: f64::extract_from_bytes(bytes, index)?,
            y: f64::extract_from_bytes(bytes, index)?,
            z: f64::extract_from_bytes(bytes, index)?,
            w: f64::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for DMat4 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x_axis.append_to_bytes(bytes)?;
        self.y_axis.append_to_bytes(bytes)?;
        self.z_axis.append_to_bytes(bytes)?;
        self.w_axis.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            x_axis: DVec4::extract_from_bytes(bytes, index)?,
            y_axis: DVec4::extract_from_bytes(bytes, index)?,
            z_axis: DVec4::extract_from_bytes(bytes, index)?,
            w_axis: DVec4::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for DQuat {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        self.z.append_to_bytes(bytes)?;
        self.w.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            x: f64::extract_from_bytes(bytes, index)?,
            y: f64::extract_from_bytes(bytes, index)?,
            z: f64::extract_from_bytes(bytes, index)?,
            w: f64::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for I16Vec2 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            x: i16::extract_from_bytes(bytes, index)?,
            y: i16::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for I16Vec3 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        self.z.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            x: i16::extract_from_bytes(bytes, index)?,
            y: i16::extract_from_bytes(bytes, index)?,
            z: i16::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for I16Vec4 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        self.z.append_to_bytes(bytes)?;
        self.w.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            x: i16::extract_from_bytes(bytes, index)?,
            y: i16::extract_from_bytes(bytes, index)?,
            z: i16::extract_from_bytes(bytes, index)?,
            w: i16::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for I64Vec2 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            x: i64::extract_from_bytes(bytes, index)?,
            y: i64::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for I64Vec3 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        self.z.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            x: i64::extract_from_bytes(bytes, index)?,
            y: i64::extract_from_bytes(bytes, index)?,
            z: i64::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for I64Vec4 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        self.z.append_to_bytes(bytes)?;
        self.w.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            x: i64::extract_from_bytes(bytes, index)?,
            y: i64::extract_from_bytes(bytes, index)?,
            z: i64::extract_from_bytes(bytes, index)?,
            w: i64::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for I8Vec2 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            x: i8::extract_from_bytes(bytes, index)?,
            y: i8::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for I8Vec3 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        self.z.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            x: i8::extract_from_bytes(bytes, index)?,
            y: i8::extract_from_bytes(bytes, index)?,
            z: i8::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for I8Vec4 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        self.z.append_to_bytes(bytes)?;
        self.w.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            x: i8::extract_from_bytes(bytes, index)?,
            y: i8::extract_from_bytes(bytes, index)?,
            z: i8::extract_from_bytes(bytes, index)?,
            w: i8::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for IVec2 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            x: i32::extract_from_bytes(bytes, index)?,
            y: i32::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for IVec3 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        self.z.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            x: i32::extract_from_bytes(bytes, index)?,
            y: i32::extract_from_bytes(bytes, index)?,
            z: i32::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for IVec4 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        self.z.append_to_bytes(bytes)?;
        self.w.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            x: i32::extract_from_bytes(bytes, index)?,
            y: i32::extract_from_bytes(bytes, index)?,
            z: i32::extract_from_bytes(bytes, index)?,
            w: i32::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for Vec3A {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        self.z.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self::new(
            f32::extract_from_bytes(bytes, index)?,
            f32::extract_from_bytes(bytes, index)?,
            f32::extract_from_bytes(bytes, index)?,
        ))
    }
}

impl ByteConverter for Mat3A {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x_axis.append_to_bytes(bytes)?;
        self.y_axis.append_to_bytes(bytes)?;
        self.z_axis.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            x_axis: Vec3A::extract_from_bytes(bytes, index)?,
            y_axis: Vec3A::extract_from_bytes(bytes, index)?,
            z_axis: Vec3A::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for Vec4 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        self.z.append_to_bytes(bytes)?;
        self.w.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self::new(
            f32::extract_from_bytes(bytes, index)?,
            f32::extract_from_bytes(bytes, index)?,
            f32::extract_from_bytes(bytes, index)?,
            f32::extract_from_bytes(bytes, index)?,
        ))
    }
}

impl ByteConverter for Mat4 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x_axis.append_to_bytes(bytes)?;
        self.y_axis.append_to_bytes(bytes)?;
        self.z_axis.append_to_bytes(bytes)?;
        self.w_axis.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            x_axis: Vec4::extract_from_bytes(bytes, index)?,
            y_axis: Vec4::extract_from_bytes(bytes, index)?,
            z_axis: Vec4::extract_from_bytes(bytes, index)?,
            w_axis: Vec4::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for U16Vec2 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            x: u16::extract_from_bytes(bytes, index)?,
            y: u16::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for U16Vec3 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        self.z.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            x: u16::extract_from_bytes(bytes, index)?,
            y: u16::extract_from_bytes(bytes, index)?,
            z: u16::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for U16Vec4 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        self.z.append_to_bytes(bytes)?;
        self.w.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            x: u16::extract_from_bytes(bytes, index)?,
            y: u16::extract_from_bytes(bytes, index)?,
            z: u16::extract_from_bytes(bytes, index)?,
            w: u16::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for U64Vec2 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            x: u64::extract_from_bytes(bytes, index)?,
            y: u64::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for U64Vec3 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        self.z.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            x: u64::extract_from_bytes(bytes, index)?,
            y: u64::extract_from_bytes(bytes, index)?,
            z: u64::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for U64Vec4 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        self.z.append_to_bytes(bytes)?;
        self.w.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            x: u64::extract_from_bytes(bytes, index)?,
            y: u64::extract_from_bytes(bytes, index)?,
            z: u64::extract_from_bytes(bytes, index)?,
            w: u64::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for U8Vec2 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            x: u8::extract_from_bytes(bytes, index)?,
            y: u8::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for U8Vec3 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        self.z.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            x: u8::extract_from_bytes(bytes, index)?,
            y: u8::extract_from_bytes(bytes, index)?,
            z: u8::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for U8Vec4 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        self.z.append_to_bytes(bytes)?;
        self.w.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            x: u8::extract_from_bytes(bytes, index)?,
            y: u8::extract_from_bytes(bytes, index)?,
            z: u8::extract_from_bytes(bytes, index)?,
            w: u8::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for UVec2 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            x: u32::extract_from_bytes(bytes, index)?,
            y: u32::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for UVec3 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        self.z.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            x: u32::extract_from_bytes(bytes, index)?,
            y: u32::extract_from_bytes(bytes, index)?,
            z: u32::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for UVec4 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.x.append_to_bytes(bytes)?;
        self.y.append_to_bytes(bytes)?;
        self.z.append_to_bytes(bytes)?;
        self.w.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            x: u32::extract_from_bytes(bytes, index)?,
            y: u32::extract_from_bytes(bytes, index)?,
            z: u32::extract_from_bytes(bytes, index)?,
            w: u32::extract_from_bytes(bytes, index)?,
        })
    }
}