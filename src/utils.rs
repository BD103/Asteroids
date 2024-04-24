use bevy::prelude::*;

use crate::VIEWPORT;

/// Creates a new [`Vec3`] from a [`Vec2`] and a specified `z` value.
pub fn compose_vec3(vec2: Vec2, z: f32) -> Vec3 {
    Vec3::new(vec2.x, vec2.y, z)
}

/// Creates a new [`Vec2`] by discarding the `z` in a [`Vec3`].
pub fn decompose_vec3(vec3: Vec3) -> Vec2 {
    Vec2::new(vec3.x, vec3.y)
}

/// Despawns all asteroids outside of the boundary.
pub fn despawn_off_screen<T: Component, const MARGIN: usize>(
    query: Query<(Entity, &Transform), With<T>>,
    mut commands: Commands,
) {
    let half_size = VIEWPORT.half_size();

    // Boundary is distance from (0, 0) on either axis to despawn entities from.
    let boundary = half_size + MARGIN as f32;

    for (entity, transform) in &query {
        let pos = transform.translation;

        if pos.x > boundary.x || pos.x < -boundary.x || pos.y > boundary.y || pos.y < -boundary.y {
            commands.entity(entity).despawn();
        }
    }
}
