use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion};

use crate::setup::actor_types::{Camera, Player};

pub fn rotate_camera(
    mouse_input: Res<Input<MouseButton>>,
    camera: Res<Camera>,
    mut motion_event: EventReader<MouseMotion>,
    mut transforms: Query<&mut Transform>
) { 
    if mouse_input.any_pressed([MouseButton::Right]) {
        for movement in motion_event.iter(){
            let mut new_transform = transforms.get_mut(camera.0).unwrap();
            new_transform.rotate_y((-movement.delta.x).to_radians());
            new_transform.rotate_x(movement.delta.y.to_radians());
            let forward = new_transform.forward();
    
            new_transform.look_to(forward, Vec3::Y);
        }
    }
}


pub fn move_camera(
    player: Res<Player>,
    camera: Res<Camera>,
    mut motion_event: EventReader<MouseMotion>,
    mut transforms: Query<&mut Transform>
) { 
    let player_transform = transforms.get(player.0).unwrap().clone();

    for movement in motion_event.iter(){
        let mut new_transform = transforms.get_mut(camera.0).unwrap();
        let xz = Vec2::new(new_transform.translation.x, new_transform.translation.z).rotate(Vec2::from_angle(movement.delta.x / 10.00));
        let xy = Vec2::new(xz[0], new_transform.translation.y).rotate(Vec2::from_angle(-movement.delta.y / 10.00));

        new_transform.translation.x = xy[0];
        new_transform.translation.y = xy[1];
        new_transform.translation.z = xz[1];
        
        new_transform.look_at(player_transform.translation, Vec3::Y);
    }
}
