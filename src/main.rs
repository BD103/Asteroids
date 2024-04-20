mod asteroid;
mod physics;
mod ship;

use bevy::{prelude::*, window::WindowResolution};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(512., 512.),
                title: "Asteroids".into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (spawn_camera, ship::spawn_ship))
        .add_systems(
            Update,
            (
                // Physics
                (physics::update_velocity, physics::update_position).chain(),
                ship::draw_ship,
                (asteroid::spawn_asteroids, asteroid::draw_asteroids),
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
