mod asteroid;
mod math;
mod physics;
mod ship;

use bevy::{prelude::*, window::WindowResolution};
use bevy_turborand::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(512., 512.),
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
                (
                    ship::rotate_ship,
                    ship::accelerate_ship,
                    ship::wrap_ships.after(physics::update_position),
                    ship::draw_ships,
                ),
                (
                    asteroid::spawn_asteroids,
                    asteroid::despawn_asteroids,
                    asteroid::draw_asteroids,
                ),
            ),
        )
        .run();
}

pub fn spawn_camera(mut commands: Commands) {
    use bevy::render::camera::ScalingMode;

    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            // World goes from -64 to +64 on X and Y axis.
            scaling_mode: ScalingMode::Fixed {
                width: 128.,
                height: 128.,
            },
            ..default()
        },
        ..default()
    });
}
