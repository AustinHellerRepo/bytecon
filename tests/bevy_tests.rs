#[cfg(test)]
mod bevy_tests {
    #[cfg(feature = "bevy")]
    #[test]
    fn bevy_serialize_entity() {
        use bevy::{ecs::{system::Commands, world::{CommandQueue, World}}, transform::components::Transform};
        use bytecon::{ByteConverter, ByteConverterFactory};

        let mut queue = CommandQueue::default();
        let mut world = World::default();
        let mut commands = Commands::new(&mut queue, &mut world);
        let mut byte_converter_factory = ByteConverterFactory::default();
        byte_converter_factory.register::<Transform>();

        let transform = Transform::from_xyz(1.0, 2.0, 3.0);
        let transform_bytes = transform.to_vec_bytes().unwrap();

        let type_id = std::any::TypeId::of::<Transform>();
        let generated_instance = byte_converter_factory.generate(type_id, &transform_bytes).unwrap();

        let entity = commands.spawn_empty()
            .insert(generated_instance);
    }
}