use naia_bevy_shared::{Channel, Message};
use crate::player::Action;

#[derive(Channel)]
pub struct PlayerActionChannel;

#[derive(Message)]
pub struct Forfeit; // Not going to let the AI forfeit, less data

#[derive(Message)]
pub struct AlterOwnable {
    pub id: u64,
    pub house: bool,
    pub hotel: bool
}

#[derive(Message)]
pub struct SellOwnable {
    pub id: u64
}

#[derive(Message)]
pub struct BuyOwnable;

#[derive(Message)]
pub struct EndTurn;

#[derive(Channel)]
pub struct BoardUpdateChannel;

#[derive(Message)]
pub struct BeginTurn { 
    // in theory we can make the client simpler by not having replication, just using begin turn to transfer necessary data
    // however, its nice to have access to a lot of information for the AI paramters
    pub sellable: Vec<u64>,
    pub position: u64,
    pub available_actions: Vec<Action>
}