use bevy::prelude::*;
use monai_store::transfer::{PlayerActionChannel, Finish};
use std::collections::HashMap;
use naia_bevy_server::{UserKey, RoomKey, events::MessageEvents, Server};

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
    pub bankrupt: Vec<UserKey>,
    pub ready: usize,
    pub finish: usize,
}

#[derive(Resource)]
pub struct Tiles {
    pub list: Vec<Entity>,
    pub tested_probability: Vec<i32>,
    pub groups: Vec<Vec<Entity>>,
    pub total_turns: usize
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    InGame,
    AutoReset
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

pub fn auto_reset( // just funnel into next game
    mut game_state: ResMut<NextState<GameState>>,
    mut players: ResMut<Players>,
    mut spaces: ResMut<Tiles>,
    code: Res<Code>,

    mut event_reader: EventReader<MessageEvents>,

    mut commands: Commands,
    mut server: Server,
) {
    while let Some(last_player) = players.list.keys().last() { // wont run after first iteration
        let last_player = *last_player;
        commands.get_entity(players.list.remove(&last_player)
            .expect("Last player is present without entity")).expect("Last player entity is not found").despawn_recursive();

        players.bankrupt.push(last_player);
    }

    for events in event_reader.iter() {
        for _ in events.read::<PlayerActionChannel, Finish>() {
            players.finish += 1;
            info!("Player has finished despawning {}/{}", players.finish, players.bankrupt.len());
            if players.finish == players.bankrupt.len() {
                players.finish = 0;
                info!("Resuming game, tile count {}", spaces.list.len());
                crate::generator::reset_game(&mut spaces, &mut players, &code, &mut commands, &mut server);
                info!("Generator finished, tile count {}", spaces.list.len());
                game_state.set(GameState::InGame);
            }
        }
    }
}