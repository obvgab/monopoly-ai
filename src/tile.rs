use crate::{ *, action::*, player::*, setup::* };
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use rand::Rng;


pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_tiles)
    }
}

#[derive(Component, Inspectable, PartialEq)]
pub enum TileType {
    Chance,
    CommunityChest,
    GoToJail,
    Tax,
    GoOrFree,
    Jail,
    Property,
}
#[derive(Component, Inspectable, PartialEq)]
pub enum Tier {
    Hotel,
    House(u32),
    Base,
    Mortgage
}
#[derive(Component, Inspectable)]
pub struct Cost {
    pub initial: i32,
    pub house: i32,
    pub hotel: i32,
    pub refund: i32
}
#[derive(Component, Inspectable)]
pub struct Owner { pub id: Entity }

#[derive(Component, Inspectable)]
pub struct TilePosition(pub i32); // Not using Entity so we can have numerical position
#[derive(Component, Inspectable)]
pub struct PairId(pub i32);
#[derive(Component, Inspectable)]
pub struct Tax(pub i32, pub i32, pub i32, pub i32, pub i32, pub i32, pub i32); // Base tax, Pair tax, 1 home, 2 homes, 3 homes, 4 homes, hotel (50% sell value)


#[derive(Bundle)]
pub struct TileBundle {
    pub position: TilePosition,
    pub tile: TileType,
    pub cost: Cost,
    pub tax: Tax,
    pub tier: Tier,
}

impl Default for TileBundle {
    fn default() -> TileBundle {
        TileBundle {
            position: TilePosition(0),
            tile: TileType::Property,
            cost: Cost { initial: 0, house: 0, hotel: 0, refund: 0 },
            tax: Tax(0, 0, 0, 0, 0, 0, 0),
            tier: Tier::Base,
        }
    }
}
//チェンソーマン
fn tile_action(
    current_player: Res<CurrentPlayer>,
    mut state: ResMut<State<GameState>>,
    mut fall_through: ResMut<FallThroughState>,
    mut player_tile: Query<
        (&mut Money, &TokenPosition, &PlayerId, &IsJailed),
        (With<PlayerId>, Without<TilePosition>),
    >, // Make sure we aren't getting a tile
    active_tile: Query<
        (&TilePosition, &TileType, &PlayerId, &PairId, &Tax, &Tier),
        With<TilePosition>,
    >,
) {
    // Get the current player attributes
    let (mut money, token_position, player_id, is_jailed) = player_tile.iter_mut().find(|x| x.2.0 == current_player.0).unwrap();
    let (position, tile, owner, pair, tax, tier) = active_tile.iter().find(|x| x.0.0 == token_position.0).unwrap();

    if token_position.0 < token_position.1 {
        println!(" Passed Go");
        money.0 += 200;
    } // Check if we passed Go or are on Go
    if is_jailed.0 { /* Jail FallThroughAction */ }
    if tile.0 == TileAttribute::Property {
        println!(" Property index: {}, Owner: {}, Player money: {}", position.0, owner.0, money.0);
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
                    if own_all { tax_price = tax.1; } else { tax_price = tax.0; }
                }
                1 => { tax_price = tax.2; }
                2 => { tax_price = tax.3; }
                3 => { tax_price = tax.4; }
                4 => { tax_price = tax.5; }
                5 => { tax_price = tax.6; }
                _ => { tax_price = 0; }
            }

            money.0 -= tax_price;
            let mut tax_collector = player_tile.iter_mut().find(|x| x.2.0 == owner.0).unwrap();
            tax_collector.0.0 += tax_price;
            println!("  Owner money: {}, Rent: {}", tax_collector.0.0, tax_price);
        } else {
            println!("  Player property")
        }
    } else if tile.0 == TileAttribute::Tax {
        money.0 -= tax.0;
        println!(" Tax Index: {}, Taxed: {}", position.0, tax.0);
    } else if tile.0 == TileAttribute::Chance {
        let card = Card::from_i32(rand::thread_rng().gen_range(0..=4));
        let mut multi = Multi::None;
        if card == Card::Fine || card == Card::Collect {
            multi = Multi::from_i32(rand::thread_rng().gen_range(0..=2));
        } else if card == Card::GoTile {
            multi = Multi::from_i32(rand::thread_rng().gen_range(2..=3));
        }
        fall_through.0 = FallThroughAction::Card;
        fall_through.1 = 100;
        fall_through.2 = card;
        fall_through.3 = multi;
    } else if tile.0 == TileAttribute::CommunityChest {
        /* Cast random number to enum for all Chest cards */

    } else if tile.0 == TileAttribute::GoToJail {
        /* Send to the jail space, then do FallThroughAction */

    }

    // Else we are on Go or Free Parking, so we can skip
    // ! This will infinite loop right now when all tiles are bought, as we don't handle debt
    state.set(GameState::PlayerAction).unwrap();
}

// * Eventually replace this with code that will allow us to have dynamic game board sizes
fn spawn_tiles(mut commands: Commands) {
    commands
        .spawn()
        .with_children(|tile_container| {
            tile_container
                .spawn_bundle(TileBundle {
                    position: TilePosition(0),
                    tile: TileType::GoOrFree,
                    ..default()
                })
                .insert(Name::new("Tile GO"));
            for i in 1..=28 {
                // ! Spawn 40 basic tiles for now
                tile_container
                    .spawn_bundle(TileBundle {
                        position: TilePosition(i),
                        tax: Tax(100, 0, 0, 0, 0, 0, 0),
                        cost: Cost(100, 0, 0, 0),
                        ..default()
                    })
                    .insert(Name::new(format!("Tile {}", i)));
            }
            for i in 29..=39 {
                tile_container
                    .spawn_bundle(TileBundle {
                        position: TilePosition(i),
                        tile: TileType::Chance,
                        ..default()
                    })
                    .insert(Name::new(format!("Tile CARD {}", i)));
            }
        })
        .insert(Name::new("TileContainer"));
}
