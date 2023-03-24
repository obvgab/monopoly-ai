use bevy_ecs::component::Component;
use bevy_reflect::Reflect;
use naia_bevy_shared::{Property, Replicate};

#[derive(Component, Replicate, Reflect)]
pub struct Money {
    pub worth: Property<i32>
}

impl Money {
    pub fn new(worth: i32) -> Self {
        Money::new_complete(worth)
    }
}