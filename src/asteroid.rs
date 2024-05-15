use crate::{bullet, color, physics, score, VIEWPORT};
use bevy::{
    math::bounding::{BoundingCircle, IntersectsVolume},
    prelude::*,
};
use bevy_turborand::prelude::*;

#[derive(Component, Default)]
pub struct Asteroid;

pub fn spawn_asteroids(mut commands: Commands, mut rng: ResMut<GlobalRng>, time: Res<Time>) {
    // Game gets progressively harder, following a sqrt curve.
    let chance = (time.elapsed_seconds_f64() * 0.00005).sqrt();

    if rng.chance(chance) {
        let half_size = VIEWPORT.half_size();

        // Calculate starting position, along 1 of 4 possible edges.
        let position = match rng.u8(0..4) {
            0 => Vec2::new(rng.f32_normalized() * half_size.x, half_size.y),
            1 => Vec2::new(rng.f32_normalized() * half_size.x, -half_size.y),
            2 => Vec2::new(half_size.x, rng.f32_normalized() * half_size.y),
            3 => Vec2::new(-half_size.x, rng.f32_normalized() * half_size.y),
            _ => unreachable!(),
        };

        // Move towards the center (0, 0), with a random offset.
        let velocity_angle = (-position).to_angle() + rng.f32_normalized();

        commands.spawn((
            Asteroid,
            Transform::from_translation(position.extend(0.0)),
            physics::Velocity(Vec2::from_angle(velocity_angle) * 32.0),
        ));
    }
}

pub fn asteroid_bullet_collision(
    asteroids_query: Query<(Entity, &Transform), With<Asteroid>>,
    bullets_query: Query<(Entity, &Transform), With<bullet::Bullet>>,
    mut commands: Commands,
    mut score: ResMut<score::Score>,
) {
    for (asteroid_entity, asteroid_transform) in &asteroids_query {
        let asteroid_bounds =
            BoundingCircle::new(asteroid_transform.translation.xy(), 8.0);

        for (bullet_entity, bullet_transform) in &bullets_query {
            let bullet_bounds =
                BoundingCircle::new(bullet_transform.translation.xy(), 2.0);

            if asteroid_bounds.intersects(&bullet_bounds) {
                commands.entity(asteroid_entity).despawn();
                commands.entity(bullet_entity).despawn();

                **score += 100;

                break;
            }
        }
    }
}

/// Draws all asteroids using gizmos.
pub fn draw_asteroids(query: Query<&Transform, With<Asteroid>>, mut gizmos: Gizmos) {
    for transform in &query {
        gizmos.circle_2d(
            transform.translation.xy(),
            8.0,
            color::BRIGHT_WHITE,
        );
    }
}
