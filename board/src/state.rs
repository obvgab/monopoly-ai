use bevy::prelude::*;
use std::collections::HashMap;
use naia_bevy_server::{UserKey, RoomKey};

#[derive(Resource)]
pub struct Code {
    pub value: String,
    pub game_room: RoomKey
}

// These structs are an abomination of ECS, but it works for now
#[derive(Resource)]
pub struct Players {
    pub list: HashMap<UserKey, Entity>,
    pub current: Option<UserKey>,
    pub name: HashMap<UserKey, String>,
    // connected by not alive players
    pub ready: usize
}

#[derive(Resource)]
pub struct Tiles {
    pub list: Vec<Entity>,
    pub tested_probability: Vec<i32>,
    pub groups: Vec<Vec<Entity>>,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    InGame
}

impl Players { // we might not **need** to deref here
    pub fn _current_player(&self) -> (&UserKey, &Entity) {
        let player = self.list.get_key_value(&self.current.unwrap()).unwrap();
        return (player.0, player.1);
    }

    pub fn current_player_entity(&self) -> &Entity {
        return self.list.get(&self.current.unwrap()).unwrap();
    }

    pub fn current_player_key(&self) -> &UserKey {
        return self.list.get_key_value(&self.current.unwrap()).unwrap().0;
    }
    
    pub fn next_player(&mut self) {
        let counter: Vec<UserKey> = self.list.keys().cloned().collect();

        let mut current_position = counter.iter().position(|&key| key == self.current.unwrap()).unwrap();
        let total_size = counter.len();

        current_position += 1;
        current_position %= total_size;

        self.current = Some(counter[current_position]);
    }

    pub fn initial_player(&mut self) { // Assumes 'list' has been populated
        let counter: Vec<UserKey> = self.list.keys().cloned().collect();

        self.current = Some(counter[0]);
    }
}