#[cfg(test)]
mod bevy_tests {
    #[cfg(feature = "bevy")]
    #[test]
    fn bevy_serialize_entity() {
        use std::error::Error;

        use bevy::{ecs::{component::Component, system::Commands, world::{CommandQueue, World}}, transform::components::Transform};
        use bytecon::{ByteConverter, ByteConverterFactory};

        struct ByteConverterFactoryContext<'a, 'w, 's> {
            commands: &'a mut Commands<'w, 's>,
        }

        let mut queue = CommandQueue::default();
        let mut world = World::default();
        world.register_component::<Transform>();
        let mut commands = Commands::new(&mut queue, &mut world);

        fn apply_component<TByteConverter>(context: &mut ByteConverterFactoryContext, byte_converter: TByteConverter) -> Result<(), Box<dyn Error + Send + Sync + 'static>>
        where
            TByteConverter: ByteConverter + Component,
        {
            context.commands.spawn_empty().insert(byte_converter);
            Ok(())
        }

        let mut byte_converter_factory = ByteConverterFactory::<ByteConverterFactoryContext, ()>::new();
        byte_converter_factory
            .register::<Transform>(apply_component);

        let transform = Transform::from_xyz(1.0, 2.0, 3.0);
        let transform_bytes = transform.to_vec_bytes().unwrap();

        let mut context = ByteConverterFactoryContext {
            commands: &mut commands,
        };
        let type_id = std::any::TypeId::of::<Transform>();
        let mut index = 0;
        let output = byte_converter_factory.extract_from_bytes_and_apply(&mut context, type_id, &transform_bytes, &mut index).unwrap();

    }
}