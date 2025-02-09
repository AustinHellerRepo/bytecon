#[cfg(test)]
mod bevy_tests {
    #[cfg(feature = "bevy")]
    #[test]
    fn bevy_serialize_entity() {
        use std::{error::Error, sync::Mutex};

        use bevy::{ecs::{component::Component, system::Commands, world::{CommandQueue, World}}, transform::components::Transform};
        use bytecon::{ByteConverter, ByteConverterFactory};

        struct ByteConverterFactoryContext<'w, 's> {
            commands: Mutex<Commands<'w, 's>>,
            component_bytes: Vec<u8>,
        }

        let mut queue = CommandQueue::default();
        let mut world = World::default();
        world.register_component::<Transform>();
        let commands = Mutex::new(Commands::new(&mut queue, &mut world));

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
                .lock()
                .map_err(|_| {
                    let error: Box<dyn Error + Send + Sync + 'static> = "Failed to lock commands.".into();
                    error
                })?
                .spawn_empty()
                .insert(byte_converter);
            Ok(true)
        }

        let mut byte_converter_factory = ByteConverterFactory::<ByteConverterFactoryContext, bool>::default();
        byte_converter_factory
            .register::<Transform>(extract_byte_converter_from_context, apply_component);

        let transform = Transform::from_xyz(1.0, 2.0, 3.0);

        let mut context = ByteConverterFactoryContext {
            commands,
            component_bytes: transform.to_vec_bytes().unwrap(),
        };
        let type_id = std::any::TypeId::of::<Transform>();
        let output = byte_converter_factory.apply(&mut context, type_id).unwrap();
        assert!(output);
    }
}