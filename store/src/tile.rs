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
pub struct Probability {
    pub id: Property<f32>
}

impl Probability {
    pub fn new(value: f32) -> Self {
        Probability::new_complete(value)
    }
}


#[derive(Component, Replicate)]
pub struct Chance;

#[derive(Component, Replicate)]
pub struct Corner;

#[derive(Component, Replicate)]
pub struct Ownable {
    pub tier: Property<Tier>,
    pub owner: Property<Option<usize>>
}

#[derive(Default, Clone, PartialEq, Serde)]
pub enum Tier {
    #[default]
    None,
    Owned,
    House,
    Hotel
}