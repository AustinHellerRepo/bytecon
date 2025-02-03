#[cfg(test)]
mod bevy_tests {
    #[cfg(feature = "bevy")]
    #[test]
    fn bevy_serialize_entity() {
        use bevy::{ecs::{system::Commands, world::{CommandQueue, World}}, transform::components::Transform};
        use bytecon::{ByteConverter, DeserializeComponentFromWorld};

        let mut queue = CommandQueue::default();
        let mut world = World::default();
        world.register_component::<Transform>();
        let mut commands = Commands::new(&mut queue, &mut world);

        let transform = Transform::from_xyz(1.0, 2.0, 3.0);
        let transform_bytes = transform.to_vec_bytes().unwrap();

        let type_id = std::any::TypeId::of::<Transform>();
        let mut index = 0;
        let generated_instance = world.deserialize_component(type_id, &transform_bytes, &mut index).unwrap();

        let entity = commands.spawn_empty().id();
        world.entity_mut(entity).insert(generated_instance);
    }
}