use bevy::{prelude::*, render::render_resource::{TextureDimension, TextureFormat, Extent3d}};

use self::actor_types::Player;
pub mod actor_types;

pub fn create_actors(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player_entity = commands.spawn(
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.0, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 1.0),
            ..default()
        }
    )
        .id();

    commands.insert_resource(
        Player(player_entity)
    );

    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(10.0).into()),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(images.add(floor_texture())),
            ..default()
        }),
        ..default()
    }).add_child(player_entity);

    // light
    commands.spawn(
        PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        }
    );
    // camera
    commands.spawn(
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        }
    );
}

const IMAGE_PIXELS: [u8; 16] = [255, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 255, 255];

fn floor_texture() -> Image {
    Image::new_fill(
        Extent3d {
            width: 2 as u32,
            height: 2 as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &IMAGE_PIXELS,
        TextureFormat::Rgba8UnormSrgb,
    )
}