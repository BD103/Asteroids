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

        commands.spawn((
            Asteroid,
            Transform::from_translation(math::compose_vec3(position, 1.0)),
            physics::Velocity(Vec2::ZERO),
        ));
    }
}

pub fn draw_asteroids(query: Query<&Transform, With<Asteroid>>, mut gizmos: Gizmos) {
    for transform in &query {
        gizmos.circle_2d(
            math::decompose_vec3(transform.translation),
            8.0,
            Color::WHITE,
        );
    }
}
