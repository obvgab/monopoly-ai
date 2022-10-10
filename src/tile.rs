use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use crate::GameState;
use crate::setup::CurrentPlayer;
use crate::player::{PlayerId, TokenPosition};

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_tiles)
            .add_system_set(SystemSet::on_update(GameState::TileAction) // ! Change to on_update for now
                .with_system(tile_action_startup));
    }
}

#[derive(Inspectable)]
pub enum TileAttribute {
    Chance,
    CommunityChest,
    GoToJail,
    Tax,
    Go,
    Property
}

#[derive(Component, Inspectable)]
pub struct TilePosition(i32);
#[derive(Component, Inspectable)]
pub struct TileType(TileAttribute);
#[derive(Component, Inspectable)]
pub struct Owner(PlayerId); // Who owns the property, -1 for none/unownable
#[derive(Component, Inspectable)]
pub struct PairId(i32); // What properties are in the same set, -1 for none
#[derive(Component, Inspectable)]
pub struct Cost(i32, i32, i32); // Cost of a property, house multiple, hotel cost
#[derive(Component, Inspectable)]
pub struct Tax(i32, i32, i32, i32, i32, i32, i32); // Base tax, Pair tax, 1 home, 2 homes, 3 homes, 4 homes, hotel
#[derive(Component, Inspectable)]
pub struct Tier(i32); // How many houses/hotels, 0 default

#[derive(Bundle)]
pub struct TileBundle {
    pub position: TilePosition,
    pub tile: TileType,
    pub owner: PlayerId,
    pub pair: PairId,
    pub cost: Cost,
    pub tax: Tax,
    pub tier: Tier
}

impl Default for TileBundle {
    fn default() -> TileBundle {
        TileBundle {
            position: TilePosition(0),
            tile: TileType(TileAttribute::Go),
            owner: PlayerId(-1),
            pair: PairId(-1),
            cost: Cost(0, 0, 0),
            tax: Tax(0, 0, 0, 0, 0, 0, 0),
            tier: Tier(0)
        }
    }
}

fn tile_action_startup(
    mut current_player: ResMut<CurrentPlayer>,
    mut state: ResMut<State<GameState>>,
    player_tile: Query<(&PlayerId, &TokenPosition), With<PlayerId>>
) {
    // * Test code for now, just going to cycle through the players
    if current_player.0 != current_player.1 - 1 {
        current_player.0 += 1; state.set(GameState::Rolling).unwrap();
    } else {
        state.set(GameState::Results).unwrap();
    }
}

// * Eventually replace this with code that will allow us to have dynamic game board sizes
fn spawn_tiles(mut commands: Commands) {
    commands.spawn().with_children(|tile_container| {
        for i in 0..40 {
            // ! Spawn 40 basic tiles for now
            tile_container.spawn_bundle(TileBundle {
                position: TilePosition(i),
                tax: Tax(-200, 0, 0, 0, 0, 0, 0),
                ..default()
            }).insert(Name::new(format!("Tile {}", i)));
        }
    }).insert(Name::new("TileContainer"));
}