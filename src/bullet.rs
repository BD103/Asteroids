use bevy::prelude::*;

use crate::{physics, ship, utils};

#[derive(Component, Default)]
pub struct Bullet;

pub fn spawn_bullets(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    query: Query<&Transform, With<ship::Ship>>,
) {
    if input.just_pressed(KeyCode::Space) {
        let transform = query.single();
        let velocity = physics::Velocity(utils::decompose_vec3(*transform.local_x()) * 80.0);

        commands.spawn((Bullet, *transform, velocity));
    }
}

pub fn draw_bullets(query: Query<&Transform, With<Bullet>>, mut gizmos: Gizmos) {
    for transform in &query {
        let start = utils::decompose_vec3(transform.translation);
        let vector = utils::decompose_vec3(transform.local_x() * 4.0);

        gizmos.ray_2d(start, vector, Color::WHITE);
    }
}
