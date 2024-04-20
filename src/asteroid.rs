use crate::{math, physics};
use bevy::prelude::*;
use bevy_turborand::prelude::*;

#[derive(Component, Default)]
pub struct Asteroid;

pub fn spawn_asteroids(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut rng: ResMut<GlobalRng>,
) {
    if input.pressed(KeyCode::Space) {
        // Calculate starting position, along 1 of 4 possible edges.
        let position = match rng.u8(0..4) {
            0 => Vec2::new(rng.f32_normalized() * 64., 64.),
            1 => Vec2::new(rng.f32_normalized() * 64., -64.),
            2 => Vec2::new(64., rng.f32_normalized() * 64.),
            3 => Vec2::new(-64., rng.f32_normalized() * 64.),
            _ => unreachable!(),
        };

        // Move towards the center (0, 0), with a random offset.
        let velocity_angle = (-position).to_angle() + rng.f32_normalized();

        commands.spawn((
            Asteroid,
            Transform::from_translation(math::compose_vec3(position, 1.0)),
            physics::Velocity(Vec2::from_angle(velocity_angle) * 32.0),
        ));
    }
}

/// Despawns all asteroids outside of the boundary.
pub fn despawn_asteroids(query: Query<(Entity, &Transform), With<Asteroid>>, mut commands: Commands) {
    for (entity, transform) in &query {
        // Edge of window (64) + asteroid radius (8)
        const BOUNDARY: f32 = 72.0;

        let pos = transform.translation;

        if pos.x > BOUNDARY || pos.x < -BOUNDARY || pos.y > BOUNDARY || pos.y < -BOUNDARY {
            commands.entity(entity).despawn();
        }
    }
}

/// Draws all asteroids using gizmos.
pub fn draw_asteroids(query: Query<&Transform, With<Asteroid>>, mut gizmos: Gizmos) {
    for transform in &query {
        gizmos.circle_2d(
            math::decompose_vec3(transform.translation),
            8.0,
            Color::WHITE,
        );
    }
}
