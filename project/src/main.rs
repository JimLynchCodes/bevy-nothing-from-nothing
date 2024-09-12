//! ULTRA simple example of a bevy project

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}

#[derive(Component)]
struct Thing;

