use bevy::prelude::*;

#[derive(Component, Clone, Copy, Deref, DerefMut, Default)]
pub struct Velocity(pub Vec2);

#[derive(Component, Clone, Copy, Deref, DerefMut, Default)]
pub struct Acceleration(pub Vec2);

pub fn update_position(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.extend(0.0) * time.delta_seconds();
    }
}

pub fn update_velocity(mut query: Query<(&mut Velocity, &Acceleration)>, time: Res<Time>) {
    for (mut velocity, acceleration) in &mut query {
        **velocity += **acceleration * time.delta_seconds();
    }
}
