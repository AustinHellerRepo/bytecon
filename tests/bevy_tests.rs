#[cfg(test)]
mod bevy_tests {
    use std::error::Error;

    use bevy::{ecs::{component::Component, entity::Entity, world::World}, transform::components::Transform};
    use bytecon::{ByteConverter, SerializationByteConverterFactory};

    #[cfg(feature = "bevy")]
    #[test]
    fn bevy_deserialize_entity() {
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
    fn bevy_serialize_entity() {

        struct ByteConverterFactoryContext<'a> {
            world: &'a mut World,
            entity: Entity,
        }

        let mut world = World::default();
        world.register_component::<Transform>();

        fn extract_bytes_from_context<TByteConverter>(context: &mut ByteConverterFactoryContext, type_name: &str) -> Result<Vec<u8>, Box<dyn Error + Send + Sync + 'static>>
        where
            TByteConverter: ByteConverter + Component,
        {
            let type_name_and_component_id_tuples = context.world.inspect_entity(context.entity)
                .map(|component_info| {
                    (component_info.type_id(), component_info.name(), component_info.id())
                })
                .filter(|(type_id_option, _, _)| {
                    type_id_option.is_some()
                })
                .map(|(_, type_name, component_id)| {
                    (type_name, component_id)
                })
                .collect::<Vec<_>>();
            for (component_type_name, component_id) in type_name_and_component_id_tuples {
                if component_type_name == type_name {
                    if let Ok(entity_ref) = context.world
                        .get_entity(context.entity) {

                        let ptr = entity_ref.get_by_id(component_id)
                            .unwrap()
                            .as_ptr() as *const TByteConverter;
                        let component_ref = unsafe { &*ptr };
                        return component_ref.to_vec_bytes();
                    }
                }
            }
            Err("Failed to find component on entity.".into())
        }

        let mut byte_converter_factory = SerializationByteConverterFactory::<ByteConverterFactoryContext>::default();
        byte_converter_factory
            .register::<Transform>(extract_bytes_from_context::<Transform>);

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