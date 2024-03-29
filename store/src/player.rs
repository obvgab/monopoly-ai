use bevy_ecs::component::Component;
use naia_bevy_shared::{Property, Replicate, Serde};

#[derive(Component, Replicate)]
pub struct Money {
    pub worth: Property<i32>
}

impl Money {
    pub fn new(worth: i32) -> Self {
        Money::new_complete(worth)
    }
}

#[derive(Component, Replicate)]
pub struct Position {
    pub tile: Property<u64>
}

impl Position {
    pub fn new(tile: u64) -> Self {
        Position::new_complete(tile)
    }
}

// Effective what defines the action state
#[derive(Default, Clone, PartialEq, Serde)]
pub enum Action {
    #[default]
    None,
    Sell, // Currently can only sell 1 at a time for the AI
    // SellMany,
    Purchase,
    // More later
}

#[derive(Component, Replicate)]
pub struct ServerPlayer {
    pub id: Property<u64>,
    pub index: Property<usize>
}

impl ServerPlayer {
    pub fn new(server: u64, index: usize) -> Self {
        ServerPlayer::new_complete(server, index)
    }
}