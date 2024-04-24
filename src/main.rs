mod asteroid;
mod bullet;
mod color;
mod physics;
mod score;
mod ship;
mod utils;

use bevy::prelude::*;
use bevy_turborand::prelude::*;

const VIEWPORT: Rect = {
    const BOUNDS: f32 = 64.0;

    Rect {
        min: Vec2::splat(-BOUNDS),
        max: Vec2::splat(BOUNDS),
    }
};

fn main() {
    use bevy::window::WindowResolution;

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
        .add_systems(
            Startup,
            (spawn_camera, ship::spawn_ship, score::spawn_score_display),
        )
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
                    ship::ship_asteroid_collision,
                    ship::draw_ships,
                ),
                // Asteroids
                (
                    asteroid::spawn_asteroids,
                    utils::despawn_off_screen::<asteroid::Asteroid, 8>,
                    asteroid::asteroid_bullet_collision,
                    asteroid::draw_asteroids,
                ),
                // Bullets
                (
                    bullet::spawn_bullets,
                    utils::despawn_off_screen::<bullet::Bullet, 4>,
                    bullet::draw_bullets,
                ),
                // Score
                score::update_score_display,
            ),
        )
        .init_resource::<score::Score>()
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}

pub fn spawn_camera(mut commands: Commands) {
    use bevy::{
        core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
        render::camera::ScalingMode,
    };

    let size = VIEWPORT.size();

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            projection: OrthographicProjection {
                // World goes from -64 to +64 on X and Y axis.
                scaling_mode: ScalingMode::Fixed {
                    width: size.x,
                    height: size.y,
                },
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            ..default()
        },
        BloomSettings::NATURAL,
    ));
}
