use bevy::prelude::*;
use setup::actor_types::Player;
mod setup;
mod systems;

fn get_entity(player: Res<Player>) {
    println!("Found Player! {}", player.0.to_bits());
}

fn main() {
    App::new()
        .add_startup_system(setup::create_actors)
        .add_plugins(DefaultPlugins)
        .add_system(get_entity)
        .add_system(systems::camera::rotate_camera.before(systems::camera::move_camera))
        .add_system(systems::camera::move_camera)
        .add_system(systems::movement::keyboard_input)
        .run();
}
