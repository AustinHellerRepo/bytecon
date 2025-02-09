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
        }

        struct SerializeComponentContext {
            static_bytes: Vec<u8>,
        }

        let mut queue = CommandQueue::default();
        let mut world = World::default();
        world.register_component::<Transform>();
        let commands = Mutex::new(Commands::new(&mut queue, &mut world));

        fn apply_component<TByteConverter>(context: &ByteConverterFactoryContext, byte_converter: TByteConverter) -> Result<bool, Box<dyn Error + Send + Sync + 'static>>
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

        fn serialize_component_from_context(context: &SerializeComponentContext) -> Result<Vec<u8>, Box<dyn Error + Send + Sync + 'static>>
        {
            Ok(context.static_bytes.clone())
        }

        let mut byte_converter_factory = ByteConverterFactory::<ByteConverterFactoryContext, SerializeComponentContext, bool>::default();
        byte_converter_factory
            .register::<Transform>(apply_component, serialize_component_from_context);

        let transform = Transform::from_xyz(1.0, 2.0, 3.0);
        let transform_bytes = transform.to_vec_bytes().unwrap();

        let mut context = ByteConverterFactoryContext {
            commands,
        };
        let type_id = std::any::TypeId::of::<Transform>();
        let mut index = 0;
        let output = byte_converter_factory.extract_from_bytes_and_apply(&mut context, type_id, &transform_bytes, &mut index).unwrap();
        assert!(output);
    }
}