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
    pub tile: Property<usize>
}

impl Position {
    pub fn new(tile: usize) -> Self {
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