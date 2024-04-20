use bevy::prelude::*;

use crate::{math, physics};

#[derive(Component, Default)]
pub struct Ship;

pub fn spawn_ship(mut commands: Commands) {
    commands.spawn((
        Ship,
        Transform::default(),
        physics::Velocity::default(),
        physics::Acceleration::default(),
    ));
}

pub fn rotate_ship(
    mut query: Query<&mut Transform, With<Ship>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut transform = query.single_mut();

    if input.pressed(KeyCode::ArrowLeft) {
        transform.rotate_z(time.delta_seconds() * 4.0);
    }

    if input.pressed(KeyCode::ArrowRight) {
        transform.rotate_z(-time.delta_seconds() * 4.0);
    }
}

pub fn accelerate_ship(
    mut query: Query<(&mut physics::Acceleration, &Transform), With<Ship>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if input.pressed(KeyCode::ArrowUp) {
        let (mut acceleration, transform) = query.single_mut();

        **acceleration = math::decompose_vec3(*transform.local_x()) * 4096.0 * time.delta_seconds();
    } else {
        let (mut acceleration, _) = query.single_mut();
        **acceleration = Vec2::ZERO;
    }
}

pub fn wrap_ships(mut query: Query<&mut Transform, With<Ship>>) {
    for mut transform in &mut query {
        let mut pos = math::decompose_vec3(transform.translation);

        pos += 64.0;
        // Euclidian remainder, similar to Java, to discard the sign.
        pos = pos.rem_euclid(Vec2::splat(128.0));
        pos -= 64.0;

        transform.translation = math::compose_vec3(pos, transform.translation.z);
    }
}

pub fn draw_ships(query: Query<&Transform, With<Ship>>, mut gizmos: Gizmos) {
    for transform in &query {
        let start = Vec2::new(transform.translation.x, transform.translation.y);

        let end = transform.translation + transform.local_x() * 10.0;
        let end = Vec2::new(end.x, end.y);

        gizmos.arrow_2d(start, end, Color::WHITE);
    }
}
