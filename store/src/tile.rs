use bevy_ecs::{component::Component};
use naia_bevy_shared::{Property, Replicate, Serde};

#[derive(Component, Replicate)]
pub struct Group {
    pub id: Property<usize>
}

impl Group {
    pub fn new(id: usize) -> Self {
        Group::new_complete(id)
    }
}

#[derive(Component, Replicate)]
pub struct ServerSide {
    pub probability: Property<f32>,
    pub id: Property<u64>,
    pub index: Property<usize>
}

impl ServerSide {
    pub fn new(probability: f32, id: u64, index: usize) -> Self {
        ServerSide::new_complete(probability, id, index)
    }
}


#[derive(Component, Replicate)]
pub struct Chance;

#[derive(Component, Replicate)]
pub struct Corner;

#[derive(Component, Replicate)]
pub struct Tile {
    pub tier: Property<Tier>,
    pub owner: Property<Option<u64>>, // u64 is the owner's entity
    pub cost: Property<i32>
    // add more tile information later
}

#[derive(Default, Clone, PartialEq, Serde)]
pub enum Tier {
    #[default]
    None,
    Owned,
    House,
    Hotel
}

impl Tile {
    pub fn new(tier: Tier, owner: Option<u64>, cost: i32) -> Self {
        Tile::new_complete(tier, owner, cost)
    }
}