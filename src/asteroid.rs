use bevy::prelude::*;

use crate::physics;

#[derive(Component, Default)]
pub struct Asteroid;

pub fn spawn_asteroids(mut commands: Commands, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        commands.spawn((
            Asteroid,
            Transform::default(),
            physics::Velocity(Vec2::ONE),
        ));
    }
}

pub fn draw_asteroids(query: Query<&Transform, With<Asteroid>>, mut gizmos: Gizmos) {
    for transform in &query {
        let position = Vec2::new(transform.translation.x, transform.translation.y);
        gizmos.circle_2d(position, 8.0, Color::WHITE);
    }
}
