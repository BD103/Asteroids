mod asteroid;
mod bullet;
mod physics;
mod ship;
mod utils;

use bevy::{prelude::*, window::WindowResolution};
use bevy_turborand::prelude::*;

#[derive(Resource, Deref, DerefMut, Default)]
pub struct Score(u32);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(512.0, 512.0),
                    title: "Asteroids".into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
            RngPlugin::new(),
        ))
        .add_systems(Startup, (spawn_camera, ship::spawn_ship))
        .add_systems(
            Update,
            (
                // Physics
                (physics::update_velocity, physics::update_position).chain(),
                // Ships
                (
                    ship::rotate_ship,
                    ship::accelerate_ship,
                    ship::wrap_ships.after(physics::update_position),
                    ship::draw_ships,
                ),
                // Asteroids
                (
                    asteroid::spawn_asteroids,
                    utils::despawn_off_screen::<asteroid::Asteroid, 8>,
                    asteroid::draw_asteroids,
                ),
                // Bullets
                (
                    bullet::spawn_bullets,
                    utils::despawn_off_screen::<bullet::Bullet, 4>,
                    bullet::draw_bullets,
                ),
            ),
        )
        .init_resource::<Score>()
        .run();
}

pub fn spawn_camera(mut commands: Commands) {
    use bevy::render::camera::ScalingMode;

    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            // World goes from -64 to +64 on X and Y axis.
            scaling_mode: ScalingMode::Fixed {
                width: 128.0,
                height: 128.0,
            },
            ..default()
        },
        ..default()
    });
}
