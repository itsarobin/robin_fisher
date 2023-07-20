use bevy::prelude::{EventReader, Res, Query, Transform, Vec3};
use bevy::input::mouse::{MouseMotion};

use crate::setup::actor_types::{Player, Camera};

pub fn rotate_camera(
    player: Res<Player>,
    camera: Res<Camera>,
    mut motion_event: EventReader<MouseMotion>,
    mut transforms: Query<&mut Transform>
) {    
    for movement in motion_event.iter(){
        let mut new_transform = transforms.get_mut(camera.0).unwrap();
        new_transform.rotate_y((-movement.delta.x).to_radians());
        new_transform.rotate_x(movement.delta.y.to_radians());
        let forward = new_transform.forward();

        new_transform.look_to(forward, Vec3::Y);
    }
}
