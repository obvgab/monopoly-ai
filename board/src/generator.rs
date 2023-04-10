use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;
use crate::{menu::BoardConfiguration, state::{Tiles, Players}};
use monai_store::tile::{Probability, Group, Chance};

pub fn generate_board(
    configuration: Res<BoardConfiguration>,
    mut tiles: ResMut<Tiles>,

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());

    let mut reference_transform = Transform::from_xyz(0.0, 0.0, 0.0);
    let radians = (360.0 / configuration.corners as f32).to_radians();
    for _rotation  in 0..configuration.corners {
        for tile in 0..(configuration.squares / configuration.corners) { 
            let entity = commands.spawn(MaterialMesh2dBundle {
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
            }).id();

            tiles.list.push(entity);

            reference_transform.translation += reference_transform.rotation * Vec3::X * 10.0;
        }

        reference_transform.rotate_z(radians);
    }
}

pub fn initialize_players(
    configuration: Res<BoardConfiguration>,

    mut tiles: ResMut<Tiles>,
    mut players: ResMut<Players>,

    mut commands: Commands
) {
    players.initial_player();

    tiles.tested_probability = vec![0; tiles.list.len()];

    let runs = tiles.list.len() as i32 * 300;
    let mut last_tile = 0;
    let mut random = rand::thread_rng();

    for turns in 0..runs {
        if turns % 30 == 0 { last_tile = 0; } 

        last_tile += random.gen_range(2..=12);
        last_tile %= tiles.list.len() - 1;

        tiles.tested_probability[last_tile] += 1;
    }

    let mut current_group = -1;
    let mut current_group_fill = 0;

    for tile in 0..tiles.list.len() {
        let mut entity_commands = commands.get_entity(tiles.list[tile]).expect("Ghost tile found");
        entity_commands.insert(Probability::new(tiles.tested_probability[tile] as f32 / runs as f32));

        let relative_tile = tile % (configuration.squares / configuration.corners) as usize;
        if relative_tile == 0 { continue; }
        if relative_tile % 3 == 1 {
            entity_commands.insert(Chance);
            continue;
        }

        if current_group_fill % 3 == 0 {
            current_group += 1;
            tiles.groups.push(vec![]);
        }

        entity_commands.insert(Group::new(current_group as usize));
        tiles.groups[current_group as usize].push(entity_commands.id());
        current_group_fill += 1;
    }
}

pub fn reset_game() {}