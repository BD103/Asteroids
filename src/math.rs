use bevy::prelude::*;

/// Creates a new [`Vec3`] from a [`Vec2`] and a specified `z` value.
pub fn compose_vec3(vec2: Vec2, z: f32) -> Vec3 {
    Vec3::new(vec2.x, vec2.y, z)
}

/// Creates a new [`Vec2`] by discarding the `z` in a [`Vec3`].
pub fn decompose_vec3(vec3: Vec3) -> Vec2 {
    Vec2::new(vec3.x, vec3.y)
}
