use crate::{
    action::{Card, FallThroughAction, Multi},
    player::{Money, PlayerId, TokenPosition},
    setup::CurrentPlayer,
    GameState,
};
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_tiles)
            .add_system_set(SystemSet::on_update(GameState::TileAction).with_system(tile_action));
    }
}

#[derive(Inspectable, PartialEq)]
pub enum TileAttribute {
    Chance,
    CommunityChest,
    GoToJail,
    Tax,
    Go,
    Property,
}

#[derive(Component, Inspectable)]
pub struct TilePosition(pub i32); // Effectively TileId too, as we can't have duplicate tiles
#[derive(Component, Inspectable)]
pub struct TileType(pub TileAttribute);
#[derive(Component, Inspectable)]
pub struct Owner(pub PlayerId); // Who owns the property, -1 for none/unownable
#[derive(Component, Inspectable)]
pub struct PairId(pub i32); // What properties are in the same set, -1 for none
#[derive(Component, Inspectable)]
pub struct Cost(pub i32, pub i32, pub i32, pub i32); // Cost of a property, house multiple, hotel cost, mortgage refund (10% to buy back)
#[derive(Component, Inspectable)]
pub struct Tax(pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32); // Base tax, Pair tax, 1 home, 2 homes, 3 homes, 4 homes, hotel (50% sell value)
#[derive(Component, Inspectable)]
pub struct Tier(pub i32); // How many houses/hotels, 0 default, -1 for mortgage

pub struct TotalTiles(pub i32);
pub struct FallThroughState(pub FallThroughAction, pub i32, pub Card, pub Multi);

#[derive(Bundle)]
pub struct TileBundle {
    pub position: TilePosition,
    pub tile: TileType,
    pub owner: PlayerId,
    pub pair: PairId,
    pub cost: Cost,
    pub tax: Tax,
    pub tier: Tier,
}

impl Default for TileBundle {
    fn default() -> TileBundle {
        TileBundle {
            position: TilePosition(0),
            tile: TileType(TileAttribute::Property),
            owner: PlayerId(-1),
            pair: PairId(-1),
            cost: Cost(0, 0, 0, 0),
            tax: Tax(0, 0, 0, 0, 0, 0, 0),
            tier: Tier(0),
        }
    }
}

fn tile_action(
    mut current_player: ResMut<CurrentPlayer>,
    mut state: ResMut<State<GameState>>,
    mut fall_through: ResMut<FallThroughState>,
    mut player_tile: Query<
        (&mut Money, &TokenPosition, &PlayerId),
        (With<PlayerId>, Without<TilePosition>),
    >, // Make sure we aren't getting a tile
    active_tile: Query<
        (&TilePosition, &TileType, &PlayerId, &PairId, &Tax, &Tier),
        With<TilePosition>,
    >,
) {
    // Get the current player attributes
    let (mut money, token_position, player_id) = player_tile
        .iter_mut()
        .filter(|x| x.2 .0 == current_player.0)
        .nth(0)
        .unwrap();
    let (position, tile, owner, pair, tax, tier) = active_tile
        .iter()
        .filter(|x| x.0 .0 == token_position.0)
        .nth(0)
        .unwrap();

    if token_position.0 < token_position.1 {
        money.0 += 200;
    } // Check if we passed Go or are on Go
    if tile.0 == TileAttribute::Property {
        if owner.0 == -1 {
            fall_through.0 = FallThroughAction::Purchase;
        } else if owner.0 != player_id.0 {
            let tax_price;
            match tier.0 {
                0 => {
                    let mut own_all = true;
                    for pair_property in active_tile.iter().filter(|x| x.3 .0 == pair.0) {
                        if pair_property.0 .0 != owner.0 {
                            own_all = false;
                            break;
                        }
                    }
                    if own_all { tax_price = tax.0; } else { tax_price = tax.1; }
                }
                1 => { tax_price = tax.2; }
                2 => { tax_price = tax.3; }
                3 => { tax_price = tax.4; }
                4 => { tax_price = tax.5; }
                5 => { tax_price = tax.6; }
                _ => { tax_price = 0; }
            }

            if money.0 - tax_price < 0 {
                fall_through.0 = FallThroughAction::Debt;
                fall_through.1 = tax_price;
            } else {
                money.0 -= tax_price;
            }
        }
    } else if tile.0 == TileAttribute::Tax {
        if money.0 - tax.0 < 0 {
            fall_through.0 = FallThroughAction::Debt;
            fall_through.1 = tax.0;
        } else {
            money.0 -= tax.0;
        }
    }

    state.set(GameState::TileAction).unwrap();
}

// * Eventually replace this with code that will allow us to have dynamic game board sizes
fn spawn_tiles(mut commands: Commands) {
    commands
        .spawn()
        .with_children(|tile_container| {
            tile_container
                .spawn_bundle(TileBundle {
                    position: TilePosition(0),
                    tile: TileType(TileAttribute::Go),
                    ..default()
                })
                .insert(Name::new("Tile GO"));
            for i in 1..40 {
                // ! Spawn 40 basic tiles for now
                tile_container
                    .spawn_bundle(TileBundle {
                        position: TilePosition(i),
                        tax: Tax(i * 100, 0, 0, 0, 0, 0, 0),
                        ..default()
                    })
                    .insert(Name::new(format!("Tile {}", i)));
            }
        })
        .insert(Name::new("TileContainer"));

    commands.insert_resource(TotalTiles(40));
}
