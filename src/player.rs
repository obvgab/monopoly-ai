use bevy::prelude::*;

// ! Move this later when we define the other states
pub enum Card {
    JailFree, // Holdable, Out of Jail Free
    Jail, // Go to Jail
    GoTile, // Go to Tile, $ GO
    Tile, // Go to Tile, !$ GO
    MultiFine, // Multiplying -$
    Fine, // Single -$
    MultiCollect, // Multiplying +$
    Collect, // Single +$
}

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
#[derive(Component)]
pub struct Money(pub i32); // 1
#[derive(Component)]
pub struct Tile(pub i32); // 2
#[derive(Component)]
// * We might be able to transition to using raw Entity IDs
pub struct PlayerId(pub i32); // 3
#[derive(Component)]
pub struct HeldJailFree(pub i32); // 4