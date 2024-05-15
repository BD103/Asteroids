use bevy::prelude::*;

use crate::{color, physics, ship};

#[derive(Component, Default)]
pub struct Bullet;

pub fn spawn_bullets(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    query: Query<&Transform, With<ship::Ship>>,
) {
    if input.just_pressed(KeyCode::Space) {
        let Ok(transform) = query.get_single() else {
            return;
        };

        let velocity = physics::Velocity(transform.local_x().xy() * 80.0);

        commands.spawn((Bullet, *transform, velocity));
    }
}

pub fn draw_bullets(query: Query<&Transform, With<Bullet>>, mut gizmos: Gizmos) {
    for transform in &query {
        let start = transform.translation.xy();
        let vector = (transform.local_x() * 4.0).xy();

        gizmos.ray_2d(start, vector, color::BRIGHT_WHITE);
    }
}
