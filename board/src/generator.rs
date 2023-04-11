use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;
use crate::{menu::BoardConfiguration, state::{Tiles, Players}, message::NextTurn};
use monai_store::{tile::{Probability, Group, Chance, Corner, Tile, Tier}, player::Position};

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

    mut event_writer: EventWriter<NextTurn>,
    mut spaces: ResMut<Tiles>,
    mut players: ResMut<Players>,

    mut commands: Commands
) {
    players.initial_player();

    spaces.tested_probability = vec![0; spaces.list.len()];

    let runs = spaces.list.len() as i32 * 300;
    let mut last_tile = 0;
    let mut random = rand::thread_rng();

    for turns in 0..runs {
        if turns % 30 == 0 { last_tile = 0; } 

        last_tile += random.gen_range(2..=12);
        last_tile %= spaces.list.len() - 1;

        spaces.tested_probability[last_tile] += 1;
    }

    let mut current_group = -1;
    let mut current_group_fill = 0;

    for tile in 0..spaces.list.len() {
        let mut entity_commands = commands.get_entity(spaces.list[tile]).expect("Ghost tile found");
        entity_commands.insert(Probability::new(spaces.tested_probability[tile] as f32 / runs as f32));
        entity_commands.insert(Tile::new(Tier::None, None, 100)); // This wont run??

        let relative_tile = tile % (configuration.squares / configuration.corners) as usize;
        if relative_tile == 0 { entity_commands.insert(Corner); continue; }
        if relative_tile % 3 == 1 { entity_commands.insert(Chance); continue; }

        if current_group_fill % 3 == 0 {
            current_group += 1;
            spaces.groups.push(vec![]);
        }

        entity_commands.insert(Group::new(current_group as usize));
        spaces.groups[current_group as usize].push(entity_commands.id());
        current_group_fill += 1;
    }

    for entity in players.list.values() {
        commands.get_entity(*entity).expect("Could not find a valid player in initialization")
            .insert(Position::new(spaces.list[0].to_bits()));
    }

    event_writer.send(NextTurn(None));
}

pub fn reset_game() {}