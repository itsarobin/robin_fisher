use bevy::prelude::*;
use bevy::input::mouse::{ MouseMotion };

use crate::setup::actor_types::{ Camera, Player };

pub fn rotate_camera(
    mouse_input: Res<Input<MouseButton>>,
    camera: Res<Camera>,
    mut motion_event: EventReader<MouseMotion>,
    mut transforms: Query<&mut Transform>
) {
    if mouse_input.any_pressed([MouseButton::Right]) {
        for movement in motion_event.iter() {
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
    mouse_input: Res<Input<MouseButton>>,
    mut motion_event: EventReader<MouseMotion>,
    mut transforms: Query<&mut Transform>
) {
    let player_transform = transforms.get(player.0).unwrap().clone();
    if mouse_input.any_pressed([MouseButton::Left]) {
        for movement in motion_event.iter() {
            let mut new_transform = transforms.get_mut(camera.0).unwrap();
            let xz_rotation = Vec2::new(
                new_transform.translation.x,
                new_transform.translation.z
            ).rotate(Vec2::from_angle(movement.delta.x / 10.0));
            let zy_rotation = Vec2::new(xz_rotation[1], new_transform.translation.y).rotate(
                Vec2::from_angle(
                    (if xz_rotation[1].is_sign_negative() {
                        -movement.delta.y
                    } else {
                        movement.delta.y
                    }) / 10.0
                )
            );

            new_transform.translation.x = xz_rotation[0];
            new_transform.translation.z = xz_rotation[1];

            new_transform.translation.y = zy_rotation[1].clamp(0.0, 10.0);
            new_transform.look_at(player_transform.translation, Vec3::Y);
        }
    }
}
