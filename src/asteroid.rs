use crate::{physics, utils};
use bevy::prelude::*;
use bevy_turborand::prelude::*;

#[derive(Component, Default)]
pub struct Asteroid;

pub fn spawn_asteroids(mut commands: Commands, mut rng: ResMut<GlobalRng>) {
    if rng.chance(0.05) {
        // Calculate starting position, along 1 of 4 possible edges.
        let position = match rng.u8(0..4) {
            0 => Vec2::new(rng.f32_normalized() * 64.0, 64.0),
            1 => Vec2::new(rng.f32_normalized() * 64.0, -64.0),
            2 => Vec2::new(64.0, rng.f32_normalized() * 64.0),
            3 => Vec2::new(-64.0, rng.f32_normalized() * 64.0),
            _ => unreachable!(),
        };

        // Move towards the center (0, 0), with a random offset.
        let velocity_angle = (-position).to_angle() + rng.f32_normalized();

        commands.spawn((
            Asteroid,
            Transform::from_translation(utils::compose_vec3(position, 0.0)),
            physics::Velocity(Vec2::from_angle(velocity_angle) * 32.0),
        ));
    }
}

/// Draws all asteroids using gizmos.
pub fn draw_asteroids(query: Query<&Transform, With<Asteroid>>, mut gizmos: Gizmos) {
    for transform in &query {
        gizmos.circle_2d(
            utils::decompose_vec3(transform.translation),
            8.0,
            Color::WHITE,
        );
    }
}
