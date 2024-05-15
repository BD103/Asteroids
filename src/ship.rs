use bevy::{
    math::bounding::{BoundingCircle, IntersectsVolume},
    prelude::*,
};

use crate::{asteroid, color, physics, VIEWPORT};

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
    let Ok(mut transform) = query.get_single_mut() else {
        return;
    };

    if input.pressed(KeyCode::ArrowLeft) {
        transform.rotate_z(time.delta_seconds() * 8.0);
    }

    if input.pressed(KeyCode::ArrowRight) {
        transform.rotate_z(-time.delta_seconds() * 8.0);
    }
}

pub fn accelerate_ship(
    mut query: Query<(&mut physics::Acceleration, &Transform), With<Ship>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let Ok((mut acceleration, transform)) = query.get_single_mut() else {
        return;
    };

    if input.pressed(KeyCode::ArrowUp) {
        **acceleration =
            transform.local_x().xy() * 4096.0 * time.delta_seconds();
    } else {
        **acceleration = Vec2::ZERO;
    }
}

pub fn wrap_ships(mut query: Query<&mut Transform, With<Ship>>) {
    let half_size = VIEWPORT.half_size();
    let size = VIEWPORT.size();

    for mut transform in &mut query {
        let mut pos = transform.translation.xy();

        pos += half_size;
        // Euclidian remainder, similar to Java, to discard the sign.
        pos = pos.rem_euclid(size);
        pos -= half_size;

        transform.translation = pos.extend(transform.translation.z);
    }
}

pub fn ship_asteroid_collision(
    ship_query: Query<(Entity, &Transform), With<Ship>>,
    asteroid_query: Query<&Transform, With<asteroid::Asteroid>>,
    mut commands: Commands,
) {
    let Ok((ship_entity, ship_transform)) = ship_query.get_single() else {
        return;
    };

    let ship_bounds = BoundingCircle::new(ship_transform.translation.xy(), 4.0);

    for asteroid_transform in &asteroid_query {
        let asteroid_bounds =
            BoundingCircle::new(asteroid_transform.translation.xy(), 8.0);

        if ship_bounds.intersects(&asteroid_bounds) {
            commands.entity(ship_entity).despawn();

            commands.spawn(Text2dBundle {
                text: Text::from_section(
                    "Game Over",
                    TextStyle {
                        font_size: 8.0,
                        color: color::WHITE,
                        ..default()
                    },
                )
                .with_no_wrap(),
                ..default()
            });

            break;
        }
    }
}

pub fn draw_ships(query: Query<&Transform, With<Ship>>, mut gizmos: Gizmos) {
    for transform in &query {
        let start = transform.translation.xy();
        let end = (transform.translation + *transform.local_x()).xy() * 10.0;

        gizmos.arrow_2d(start, end, color::BRIGHT_WHITE);
    }
}
