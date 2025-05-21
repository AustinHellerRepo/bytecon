use avian3d::{collision::collider::ColliderConstructorHierarchyConfig, math::Scalar, prelude::*};
use bevy::ecs::entity::Entity;
use glam::{Quat, Vec3};
use crate::ByteConverter;

impl ByteConverter for AngularVelocity {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.0.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self(Vec3::extract_from_bytes(bytes, index)?))
    }
}

impl ByteConverter for CoefficientCombine {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        match self {
            Self::Average => 0u8.append_to_bytes(bytes)?,
            Self::GeometricMean => 1u8.append_to_bytes(bytes)?,
            Self::Min => 2u8.append_to_bytes(bytes)?,
            Self::Multiply => 3u8.append_to_bytes(bytes)?,
            Self::Max => 4u8.append_to_bytes(bytes)?,
        }
        Ok(())
    }
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        let enum_variant_byte = u8::extract_from_bytes(bytes, index)?;
        match enum_variant_byte {
            0u8 => Ok(Self::Average),
            1u8 => Ok(Self::GeometricMean),
            2u8 => Ok(Self::Min),
            3u8 => Ok(Self::Multiply),
            4u8 => Ok(Self::Max),
            _ => Err("Unexpected enum variant byte.".into()),
        }
    }
}

impl ByteConverter for Collider {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let serialized_bytes = bincode::serialize(self)?;
        serialized_bytes.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        let serialized_bytes = Vec::<u8>::extract_from_bytes(bytes, index)?;
        Ok(bincode::deserialize::<Self>(&serialized_bytes)?)
    }
}

impl ByteConverter for ColliderAabb {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.min.append_to_bytes(bytes)?;
        self.max.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            min: Vec3::extract_from_bytes(bytes, index)?,
            max: Vec3::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for ColliderConstructor {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let serialized_bytes = bincode::serialize(self)?;
        serialized_bytes.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        let serialized_bytes = Vec::<u8>::extract_from_bytes(bytes, index)?;
        Ok(bincode::deserialize::<Self>(&serialized_bytes)?)
    }
}

impl ByteConverter for ColliderConstructorHierarchy {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.default_constructor.append_to_bytes(bytes)?;
        self.default_layers.append_to_bytes(bytes)?;
        self.default_density.append_to_bytes(bytes)?;
        self.config.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            default_constructor: Option::<ColliderConstructor>::extract_from_bytes(bytes, index)?,
            default_layers: CollisionLayers::extract_from_bytes(bytes, index)?,
            default_density: ColliderDensity::extract_from_bytes(bytes, index)?,
            config: bevy::platform::collections::HashMap::<String, Option<ColliderConstructorHierarchyConfig>>::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for ColliderConstructorHierarchyConfig {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.constructor.append_to_bytes(bytes)?;
        self.layers.append_to_bytes(bytes)?;
        self.density.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            constructor: Option::<ColliderConstructor>::extract_from_bytes(bytes, index)?,
            layers: Option::<CollisionLayers>::extract_from_bytes(bytes, index)?,
            density: Option::<ColliderDensity>::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for ColliderDensity {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.0.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self(f32::extract_from_bytes(bytes, index)?))
    }
}

impl ByteConverter for ColliderDisabled {
    fn append_to_bytes(&self, _bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        // nothing to serialize
        Ok(())
    }
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(_bytes: &'a TBytes, _index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        // nothing to deserialize
        Ok(Self)
    }
}

impl ByteConverter for ColliderMarker {
    fn append_to_bytes(&self, _bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        // nothing to serialize
        Ok(())
    }
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(_bytes: &'a TBytes, _index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        // nothing to deserialize
        Ok(Self)
    }
}

impl ByteConverter for ColliderMassProperties {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let serialized_bytes = bincode::serialize(self)?;
        serialized_bytes.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        let serialized_bytes = Vec::<u8>::extract_from_bytes(bytes, index)?;
        Ok(bincode::deserialize::<Self>(&serialized_bytes)?)
    }
}

impl ByteConverter for ColliderOf {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.body.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            body: Entity::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for ColliderTransform {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.translation.append_to_bytes(bytes)?;
        self.rotation.append_to_bytes(bytes)?;
        self.scale.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            translation: Vec3::extract_from_bytes(bytes, index)?,
            rotation: Rotation::extract_from_bytes(bytes, index)?,
            scale: Vec3::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for CollisionEnded {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.0.append_to_bytes(bytes)?;
        self.1.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self(Entity::extract_from_bytes(bytes, index)?, Entity::extract_from_bytes(bytes, index)?))
    }
}

impl ByteConverter for CollisionLayers {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.memberships.append_to_bytes(bytes)?;
        self.filters.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            memberships: LayerMask::extract_from_bytes(bytes, index)?,
            filters: LayerMask::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for CollisionMargin {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.0.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self(Scalar::extract_from_bytes(bytes, index)?))
    }
}

impl ByteConverter for CollisionStarted {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.0.append_to_bytes(bytes)?;
        self.1.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self(Entity::extract_from_bytes(bytes, index)?, Entity::extract_from_bytes(bytes, index)?))
    }
}

impl ByteConverter for DefaultFriction {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.0.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self(Friction::extract_from_bytes(bytes, index)?))
    }
}

impl ByteConverter for DefaultRestitution {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.0.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self(Restitution::extract_from_bytes(bytes, index)?))
    }
}

impl ByteConverter for ExternalForce {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let serialized_bytes = bincode::serialize(self)?;
        serialized_bytes.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        let serialized_bytes = Vec::<u8>::extract_from_bytes(bytes, index)?;
        Ok(bincode::deserialize::<Self>(&serialized_bytes)?)
    }
}

impl ByteConverter for ExternalImpulse {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let serialized_bytes = bincode::serialize(self)?;
        serialized_bytes.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        let serialized_bytes = Vec::<u8>::extract_from_bytes(bytes, index)?;
        Ok(bincode::deserialize::<Self>(&serialized_bytes)?)
    }
}

impl ByteConverter for ExternalTorque {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let serialized_bytes = bincode::serialize(self)?;
        serialized_bytes.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        let serialized_bytes = Vec::<u8>::extract_from_bytes(bytes, index)?;
        Ok(bincode::deserialize::<Self>(&serialized_bytes)?)
    }
}

impl ByteConverter for Friction {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.dynamic_coefficient.append_to_bytes(bytes)?;
        self.static_coefficient.append_to_bytes(bytes)?;
        self.combine_rule.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            dynamic_coefficient: Scalar::extract_from_bytes(bytes, index)?,
            static_coefficient: Scalar::extract_from_bytes(bytes, index)?,
            combine_rule: CoefficientCombine::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for Gravity {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.0.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self(Vec3::extract_from_bytes(bytes, index)?))
    }
}

impl ByteConverter for GravityScale {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.0.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self(f32::extract_from_bytes(bytes, index)?))
    }
}

impl ByteConverter for LayerMask {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.0.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self(u32::extract_from_bytes(bytes, index)?))
    }
}

impl ByteConverter for LinearVelocity {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.0.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self(Vec3::extract_from_bytes(bytes, index)?))
    }
}

impl ByteConverter for Mass {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.0.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self(f32::extract_from_bytes(bytes, index)?))
    }
}

impl ByteConverter for MassProperties3d {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.mass.append_to_bytes(bytes)?;
        self.principal_angular_inertia.append_to_bytes(bytes)?;
        self.local_inertial_frame.append_to_bytes(bytes)?;
        self.center_of_mass.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            mass: f32::extract_from_bytes(bytes, index)?,
            principal_angular_inertia: Vec3::extract_from_bytes(bytes, index)?,
            local_inertial_frame: Quat::extract_from_bytes(bytes, index)?,
            center_of_mass: Vec3::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for Restitution {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.coefficient.append_to_bytes(bytes)?;
        self.combine_rule.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            coefficient: Scalar::extract_from_bytes(bytes, index)?,
            combine_rule: CoefficientCombine::extract_from_bytes(bytes, index)?,
        })
    }
}

impl ByteConverter for Sensor {
    fn append_to_bytes(&self, _bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        // nothing to serialize
        Ok(())
    }
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(_bytes: &'a TBytes, _index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        // nothing to deserialize
        Ok(Self)
    }
}

impl ByteConverter for Sleeping {
    fn append_to_bytes(&self, _bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        // nothing to serialize
        Ok(())
    }
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(_bytes: &'a TBytes, _index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        // nothing to deserialize
        Ok(Self)
    }
}

impl ByteConverter for SpeculativeMargin {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.0.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self(f32::extract_from_bytes(bytes, index)?))
    }
}

impl ByteConverter for Position {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.0.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self(Vec3::extract_from_bytes(bytes, index)?))
    }
}

impl ByteConverter for RigidBody {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        match self {
            Self::Dynamic => {
                0u8.append_to_bytes(bytes)?;
            },
            Self::Static => {
                1u8.append_to_bytes(bytes)?;
            },
            Self::Kinematic => {
                2u8.append_to_bytes(bytes)?;
            },
        }
        Ok(())
    }
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        let enum_variant_byte = u8::extract_from_bytes(bytes, index)?;
        match enum_variant_byte {
            0u8 => Ok(Self::Dynamic),
            1u8 => Ok(Self::Static),
            2u8 => Ok(Self::Kinematic),
            _ => Err("Unexpected enum variant byte.".into()),
        }
    }
}

impl ByteConverter for Rotation {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.0.append_to_bytes(bytes)?;
        Ok(())
    }
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self(Quat::extract_from_bytes(bytes, index)?))
    }
}