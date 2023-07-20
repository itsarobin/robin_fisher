use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion};

use crate::setup::actor_types::{Camera};

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
