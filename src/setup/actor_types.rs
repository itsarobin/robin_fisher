use bevy::prelude::{Entity, Resource};

#[derive(Resource)]
pub struct Player(pub Entity);

#[derive(Resource)]
pub struct Camera(pub Entity);
