use bevy::prelude::*;
mod setup;
mod systems;

fn get_entity(
    mut query: Query<(Entity, With<setup::actor_types::Player>)>,
    mut commands: Commands
) {
    for player in &mut query {
        let player_entity = commands.entity(player.0).id();
        println!("Found Player! {}", player_entity.to_bits());
    }
}

fn main() {
    App::new()
        .add_startup_system(setup::create_actors)
        .add_plugins(DefaultPlugins)
        .add_system(get_entity)
        .add_system(systems::movement::keyboard_input)
        .run();
}