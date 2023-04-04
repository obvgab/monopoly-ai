use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use crate::menu::BoardConfiguration;
use monai_store::tile;

pub fn generate_board(
    configuration: Res<BoardConfiguration>,

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());

    let mut reference_transform = Transform::from_xyz(0.0, 0.0, 0.0);
    let radians = (360.0 / configuration.corners as f32).to_radians();
    for rotation  in 0..configuration.corners {
        for tile in 0..(configuration.squares / configuration.corners) { 
            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(
                    if tile == 0 {
                        shape::Circle::new(5.0).into()
                    } else {
                        shape::Quad::new(Vec2::new(5.0, 10.0)).into()
                    }
                ).into(),
                material: materials.add(ColorMaterial::from(Color::BLACK)),
                transform: reference_transform,
                ..default()
            }).insert(tile::Group::new(tile + ((configuration.squares / configuration.corners) * rotation)));

            reference_transform.translation += reference_transform.rotation * Vec3::X * 10.0;
        }

        reference_transform.rotate_z(radians);
    }
}

pub fn initialize_players(
    mut _commands: Commands
) {

}

pub fn reset_game() {}