use bevy::prelude::*;

#[derive(Component, Deref, DerefMut, Default)]
pub struct Velocity(Vec2);

#[derive(Component, Deref, DerefMut, Default)]
pub struct Acceleration(Vec2);

pub fn update_position(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }
}

pub fn update_velocity(mut query: Query<(&mut Velocity, &Acceleration)>) {
    for (mut velocity, acceleration) in &mut query {
        **velocity += **acceleration;
    }
}
