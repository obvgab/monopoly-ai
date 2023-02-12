use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

/*
    All the tags/components for Player entities
    Everything is considered public, as most
    systems will eventually acess one of these
    values for operation
    (1) The amount of money a player has
    (2) The tile index that the player is on (tiles are considered 1d array)
    (3) The corresponding index for CurrentPlayer(i32, i32)
    (4) An enumeration of Card::JailFree a player has
    Properties are different entities, as we might need to index them separately
    for when another player lands on them
*/
#[derive(Component, Inspectable)]
pub struct Money { pub worth: i32 } // 1
#[derive(Component, Inspectable)]
pub struct TokenPosition { pub current: i32, pub previous: i32 } // 2
#[derive(Component, Inspectable)]
pub struct HeldJailFree(pub i32); // 4

#[derive(Component, Inspectable)]
#[component(storage="SparseSet")]
pub struct Computer;
#[derive(Component, Inspectable)]
#[component(storage="SparseSet")]
pub struct Jailed;

/*
    The actual player bundle that contains
    the above components. This allows for
    easy insertions via .insert_bundle()
*/
#[derive(Bundle)]
pub struct PlayerBundle {
    pub money: Money,
    pub token_position: TokenPosition,
    pub held_jail_free: HeldJailFree
}
