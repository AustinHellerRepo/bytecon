#[cfg(test)]
mod bevy_tests {
    use std::error::Error;

    use bevy::{ecs::{component::Component, entity::Entity, world::World}, transform::components::Transform};
    use bytecon::{ByteConverter, Context, SerializationByteConverterFactory};

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

        fn apply_component<TByteConverter>(context: &mut ByteConverterFactoryContext, byte_converter: TByteConverter) -> Result<bool, Box<dyn Error + Send + Sync + 'static>>
        where
            TByteConverter: ByteConverter + Component,
        {
            context.commands
                .spawn_empty()
                .insert(byte_converter);
            Ok(true)
        }

        let mut byte_converter_factory = DeserializationByteConverterFactory::<bool>::default();
        byte_converter_factory
            .register::<Transform, ByteConverterFactoryContext>(extract_byte_converter_from_context, apply_component);

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
}