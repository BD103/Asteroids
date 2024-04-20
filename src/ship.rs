use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Ship;

pub fn spawn_ship(mut commands: Commands) {
    commands.spawn((
        Ship,
        Transform::default(),
    ));
}

pub fn draw_ship(query: Query<&Transform, With<Ship>>, mut gizmos: Gizmos) {
    for transform in &query {
        let start = Vec2::new(transform.translation.x, transform.translation.y);

        let end = transform.translation + transform.local_x() * 10.0;
        let end = Vec2::new(end.x, end.y);

        gizmos.arrow_2d(start, end, Color::WHITE);
    }
}
