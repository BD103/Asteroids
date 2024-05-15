use bevy::prelude::*;

use crate::VIEWPORT;

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
