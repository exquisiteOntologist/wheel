use bevy::{
    ecs::world::Mut,
    math::{Vec3, Vec3Swizzles},
    transform::components::Transform,
};

pub fn look_at_on_y(t_looking: &mut Mut<Transform>, t_target: &Transform) {
    t_looking.look_at(t_target.translation.xyz(), Vec3::Y);
}
