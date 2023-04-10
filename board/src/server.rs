use bevy::prelude::*;
use std::collections::HashMap;
use naia_bevy_server::{Server, events::{AuthEvents, ConnectEvent, TickEvent, DisconnectEvent}, transport::webrtc, CommandsExt};
use monai_store::{Auth, player::Money};
use crate::{state::{Players, Code, Tiles}, menu::BoardConfiguration};

pub fn initialize_server(
    mut commands: Commands,
    mut server: Server
) {
    let address = webrtc::ServerAddrs::new( // Σ ascii("monai") = 1096
        "127.0.0.1:1095".parse().expect("Could not parse signal local address"),
        "127.0.0.1:1096".parse().expect("Could not parse WebRTC local address"),
        "http://127.0.0.1:1096"
    );

    let socket = webrtc::Socket::new(&address, server.socket_config());
    server.listen(socket);
    
    // Make this random later
    commands.insert_resource(Players { list: HashMap::new(), current: None, name: HashMap::new() });
    commands.insert_resource(Code { value: "MONAI".to_string(), game_room: server.make_room().key() });
    commands.insert_resource(BoardConfiguration { polygonal_board: false, corners: 4, squares: 40 });
    commands.insert_resource(Tiles { list: vec![], tested_probability: vec![], groups: vec![] });
    
    info!("Naia server initialized");
}

pub fn tick(
    mut event_reader: EventReader<TickEvent>,

    mut server: Server
) {
    let mut ticked = false;

    // Placeholder tick code
    for TickEvent(_event) in event_reader.iter() {
        ticked = true;
        break; // temp
    }

    if ticked {
        for (_, key, entity) in server.scope_checks() { // Ignore room key, we only have one
            server.user_scope(&key).include(&entity); // Include all entities, no checking
        }
    }
}

pub fn authorize_player(
    mut event_reader: EventReader<AuthEvents>,

    code: Res<Code>,
    mut players: ResMut<Players>,

    mut server: Server
) {
    for event in event_reader.iter() {
        for (key, auth) in event.read::<Auth>() {
            if players.name.contains_key(&key) {
                server.reject_connection(&key);
                info!("Declined connection from player {}, key already authorized", auth.name);
                continue;
            }

            if auth.code != code.value  {
                server.reject_connection(&key);
                info!("Declined connection from player {}, code invalid", auth.name);
                continue;
            }

            server.accept_connection(&key);
            info!("Authorized connection from player {}", auth.name);
            players.name.insert(key, auth.name);
        }
    }
}

pub fn connect_player(
    mut event_reader: EventReader<ConnectEvent>,

    code: Res<Code>,
    mut players: ResMut<Players>,

    mut commands: Commands,
    mut server: Server
) {
    for ConnectEvent(key) in event_reader.iter() { // needs player components
        let user = server.user_mut(key).enter_room(&code.game_room).address();
        let entity = commands
            .spawn_empty()
            .enable_replication(&mut server)
            .insert(Money::new(100))
            .id();

        server.room_mut(&code.game_room).add_entity(&entity);
        players.list.insert(*key, entity);

        info!("Connected and spawned entity for {}, {}", players.name[key], user);
    }
}

pub fn disconnect_player(
    mut event_reader: EventReader<DisconnectEvent>,

    mut players: ResMut<Players>,

    mut commands: Commands
) {
    for DisconnectEvent(key, _user) in event_reader.iter() {
        let entity = players.list.remove(key).expect("User not registered");
        let name = players.name.remove(key).expect("User has no name");
        commands.get_entity(entity).expect("User entity already removed").despawn_recursive();

        info!("Disconnected and removed entity for {}", name);
    }
}