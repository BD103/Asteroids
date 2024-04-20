mod asteroid;
mod physics;
mod ship;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, (
            // Physics
            (physics::update_velocity, physics::update_position).chain(),
        ))
        .run();
}
