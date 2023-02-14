use crate::{ *, action::*, player::*, setup::* };
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use rand::Rng;


pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_tiles);
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
    House(usize),
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
pub struct Owner {
    pub id: Entity
}
#[derive(Component, Inspectable)]
pub struct Tax {
    pub base: i32,
    pub pair: i32,
    pub home: [i32; 4],
    pub hotel: i32
} // 50% sell value
#[derive(Component, Inspectable, PartialEq)]
pub struct Pair { pub id: i32 }

#[derive(Component, Inspectable)]
pub struct Space { pub id: i32 } // Not using Entity so we can have numerical position


#[derive(Bundle)]
pub struct TileBundle {
    pub space: Space,
    pub tile: TileType,
    pub cost: Cost,
    pub tax: Tax,
    pub tier: Tier,
}

impl Default for TileBundle {
    fn default() -> TileBundle {
        TileBundle {
            space: Space { id: 0 },
            tile: TileType::Property,
            cost: Cost { initial: 0, house: 0, hotel: 0, refund: 0 },
            tax: Tax { base: 0, pair: 0, home: [0, 0, 0, 0], hotel: 0 },
            tier: Tier::Base,
        }
    }
}

// * Eventually replace this with code that will allow us to have dynamic game board sizes
fn spawn_tiles(mut commands: Commands) {
    commands
        .spawn()
        .with_children(|tile_container| {
            tile_container
                .spawn_bundle(TileBundle {
                    space: Space { id: 0 },
                    tile: TileType::GoOrFree,
                    ..default()
                })
                .insert(Name::new("Tile GO"));
            for i in 1..=28 {
                // ! Spawn 40 basic tiles for now
                tile_container
                    .spawn_bundle(TileBundle {
                        space: Space { id: i },
                        tax: Tax { base: 100, pair: 0, home: [0, 0, 0, 0], hotel: 0 },
                        cost: Cost { initial: 100, house: 0, hotel: 0, refund: 0 },
                        ..default()
                    })
                    .insert(Name::new(format!("Tile {}", i)));
            }
            for i in 29..=39 {
                tile_container
                    .spawn_bundle(TileBundle {
                        space: Space { id: i },
                        tile: TileType::Chance,
                        ..default()
                    })
                    .insert(Name::new(format!("Tile CARD {}", i)));
            }
        })
        .insert(Name::new("TileContainer"));
}
