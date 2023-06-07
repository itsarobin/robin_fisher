use bevy::prelude::*;
mod setup;

fn main() {
    App::new()
        .add_startup_system(setup::create_actors)
        .add_plugins(DefaultPlugins)
        .run();
}