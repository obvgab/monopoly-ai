use bevy_ecs::component::Component;
use naia_bevy_shared::{Property, Replicate};

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
    pub tile: Property<i32>
}

impl Position {
    pub fn new(tile: i32) -> Self {
        Position::new_complete(tile)
    }
}