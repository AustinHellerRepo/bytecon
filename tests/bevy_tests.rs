#[cfg(test)]
mod bevy_tests {
    use std::error::Error;

    use bevy::{asset::{uuid::Uuid, AssetId, Assets}, color::Color, ecs::{component::Component, entity::Entity, resource::Resource, world::World}, pbr::wireframe::WireframeMaterial, transform::components::Transform};
    use bytecon::{bevy::{BevyWorldMutSingleton, BevyWorldRefSingleton}, ByteConverter, Context, DeserializationByteConverterFactory, SerializationByteConverterFactory};

    #[cfg(feature = "bevy")]
    #[test]
    fn test_y7v9_bevy_deserialize_entity() {
        use std::error::Error;

        use bevy::{ecs::{component::Component, system::Commands, world::{CommandQueue, World}}, transform::components::Transform};
        use bytecon::{ByteConverter, Context, DeserializationByteConverterFactory};

        struct ByteConverterFactoryContext<'w, 's, 'a> {
            commands: &'a mut Commands<'w, 's>,
            component_bytes: Vec<u8>,
        }

        impl<'w, 's, 'a> Context for ByteConverterFactoryContext<'w, 's, 'a> {

        }

        let mut queue = CommandQueue::default();
        let mut world = World::default();
        world.register_component::<Transform>();
        let mut commands = Commands::new(&mut queue, &mut world);

        fn extract_byte_converter_from_context<TByteConverter>(context: &mut ByteConverterFactoryContext) -> Result<TByteConverter, Box<dyn Error + Send + Sync + 'static>>
        where
            TByteConverter: ByteConverter + Component,
        {
            let byte_converter = TByteConverter::deserialize_from_bytes(
                &context.component_bytes,
            )?;
            Ok(byte_converter)
        }

        fn apply_component<TByteConverter>(context: &mut ByteConverterFactoryContext) -> Result<bool, Box<dyn Error + Send + Sync + 'static>>
        where
            TByteConverter: ByteConverter + Component,
        {
            let byte_converter = TByteConverter::deserialize_from_bytes(&context.component_bytes)?;
            context.commands
                .spawn_empty()
                .insert(byte_converter);
            Ok(true)
        }

        let mut byte_converter_factory = DeserializationByteConverterFactory::<bool>::default();
        byte_converter_factory
            .register::<Transform, ByteConverterFactoryContext>(apply_component::<Transform>);

        let transform = Transform::from_xyz(1.0, 2.0, 3.0);

        let mut context = ByteConverterFactoryContext {
            commands: &mut commands,
            component_bytes: transform.to_vec_bytes().unwrap(),
        };
        let type_name = std::any::type_name::<Transform>();
        let output = byte_converter_factory.deserialize(&mut context, type_name).unwrap();
        assert!(output);
    }

    #[test]
    fn test_o2s9_bevy_serialize_entity() {

        struct ByteConverterFactoryContext<'a> {
            world: &'a mut World,
            entity: Entity,
        }

        impl<'a> Context for ByteConverterFactoryContext<'a> {

        }

        let mut world = World::default();
        world.register_component::<Transform>();

        fn extract_bytes_from_context<TByteConverter>(context: &mut ByteConverterFactoryContext) -> Result<Vec<u8>, Box<dyn Error + Send + Sync + 'static>>
        where
            TByteConverter: ByteConverter + Component,
        {
            if let Some(component) = context.world.get::<TByteConverter>(context.entity) {
                return component.to_vec_bytes();
            }
            Err("Failed to find component on entity.".into())
        }

        let mut byte_converter_factory = SerializationByteConverterFactory::default();
        byte_converter_factory
            .register::<Transform, ByteConverterFactoryContext>(extract_bytes_from_context::<Transform>);

        let transform = Transform::from_xyz(1.0, 2.0, 3.0);
        let expected_bytes = transform.to_vec_bytes().unwrap();
        let entity = world
            .spawn_empty()
            .insert(transform)
            .id();

        let mut context = ByteConverterFactoryContext {
            world: &mut world,
            entity,
        };

        let type_name = std::any::type_name::<Transform>();
        println!("looking for {}", type_name);
        let actual_bytes = byte_converter_factory.serialize(&mut context, type_name).unwrap();
        assert_eq!(expected_bytes, actual_bytes);
    }

    #[test]
    fn test_z7v1_bevy_deserialize_resource() {
        #[derive(Resource)]
        struct ReplicatedResource {
            value: String,
        }

        impl ByteConverter for ReplicatedResource {
            fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
                self.value.append_to_bytes(bytes)?;
                Ok(())
            }
            fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
                Ok(Self {
                    value: String::extract_from_bytes(bytes, index)?,
                })
            }
        }

        struct UpdateResourceByteConverterFactoryContext<'a> {
            world: &'a mut World,
            byte_converter_bytes: &'a Vec<u8>,
        }

        impl<'a> Context for UpdateResourceByteConverterFactoryContext<'a> {}

        let mut world = World::default();
        //world.register_resource::<ReplicatedResource>();

        fn insert_resource_apply_function<TByteConverter>(
            context: &mut UpdateResourceByteConverterFactoryContext,
        ) -> Result<(), Box<dyn Error + Send + Sync + 'static>>
        where
            TByteConverter: ByteConverter + Resource,
        {
            let resource = TByteConverter::deserialize_from_bytes(context.byte_converter_bytes)?;
            context.world.insert_resource(resource);
            Ok(())
        }

        let mut byte_converter_factory: DeserializationByteConverterFactory<()> = DeserializationByteConverterFactory::default();
        byte_converter_factory.register::<ReplicatedResource, UpdateResourceByteConverterFactoryContext>(insert_resource_apply_function::<ReplicatedResource>);

        let expected_resource = ReplicatedResource {
            value: String::from("Hello world"),
        };

        let mut context = UpdateResourceByteConverterFactoryContext {
            world: &mut world,
            byte_converter_bytes: &expected_resource.to_vec_bytes().unwrap(),
        };
        let type_name = std::any::type_name::<ReplicatedResource>();
        byte_converter_factory.deserialize(&mut context, type_name).unwrap();
    }

    #[test]
    fn test_a8e9_serialize_and_deserialize_asset_handle() {
        let mut world = World::default();
        world.init_resource::<Assets<WireframeMaterial>>();
        
        let mut wireframe_assets = world.get_resource_mut::<Assets<WireframeMaterial>>().expect("Failed to find WireframeMaterial assets.");

        let wireframe_material = WireframeMaterial {
            color: Color::WHITE,
        };
        let asset_id = AssetId::Uuid {
            uuid: Uuid::parse_str("ba2e24ab-7460-4647-87f2-0581bac4360a").unwrap(),
        };
        wireframe_assets.insert(asset_id, wireframe_material);
        let expected_handle = wireframe_assets.get_strong_handle(asset_id).expect("Failed to find handle for AssetId.");

        BevyWorldMutSingleton::set(&mut world, || {
            let actual_handle = expected_handle.clone_via_bytes().expect("Failed to clone via bytes.");
            assert_eq!(expected_handle, actual_handle);
            Ok(())
        })
            .expect("Failed to set world.");
    }
}