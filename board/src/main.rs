use bevy::prelude::*;
use std::collections::HashMap;
use naia_bevy_server::{Plugin as ServerPlugin, ReceiveEvents, UserKey, RoomKey, ServerConfig};
use monai_store::protocol_builder;

mod server;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ServerPlugin::new(ServerConfig::default(), protocol_builder()))
        .add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin::new())

        .add_systems(
            (
                server::authorize_player,
                server::connect_player,
                server::tick
            )
            .chain()
            .in_set(ReceiveEvents)
        )
        .add_startup_system(server::initialize_server)

        .run();
}

/*
 * Important data structures
 */
#[derive(Resource)]
pub struct Code {
    value: String,
    game_room: RoomKey
}

#[derive(Resource)]
pub struct Players {
    pub list: HashMap<UserKey, Entity>,
    pub current: Option<UserKey>,
    pub name: HashMap<UserKey, String>
}

impl Players { // we might not **need** to deref here
    pub fn current_player(&self) -> (&UserKey, &Entity) {
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

        if current_position == total_size - 1 {
            current_position = 0;
        } else {
            current_position += 1;
        }

        self.current = Some(counter[current_position]);
    }

    pub fn initial_player(&mut self) { // Assumes 'list' has been populated
        let counter: Vec<UserKey> = self.list.keys().cloned().collect();

        self.current = Some(counter[0]);
    }
}