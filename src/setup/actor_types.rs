use bevy::prelude::{Resource, Entity};

#[derive(Resource)]
pub struct Player(pub Entity);

#[derive(Resource)]
pub struct Camera(pub Entity);