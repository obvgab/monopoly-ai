use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use crate::menu::BoardConfiguration;

pub fn generate_board(
    configuration: Res<BoardConfiguration>,

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());

    let mut reference_transform = Transform::from_xyz(0.0, 0.0, 0.0);
    let radians = (360.0 / configuration.corners as f32).to_radians();
    for _rotation  in 0..configuration.corners {
        for _tile in 0..(configuration.squares / configuration.corners) { 
            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(10.0).into()).into(),
                material: materials.add(ColorMaterial::from(Color::BLACK)),
                transform: reference_transform,
                ..default()
            });

            reference_transform.translation += reference_transform.rotation * Vec3::X * 10.0;
        }

        reference_transform.rotate_z(radians);
    }
}

pub fn initialize_board(
    mut _commands: Commands
) {

}

pub fn reset_board() {}