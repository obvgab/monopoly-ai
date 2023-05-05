use bevy::prelude::*;
use model::StatefulInformation;
use naia_bevy_client::{ClientConfig, Plugin as ClientPlugin, Client, transport::webrtc};
use monai_store::{protocol_builder, Auth, tile::Tile, transfer::{PlayerActionChannel, Ready, Finish}};
use std::env;

mod model;

pub const SQUARES: usize = 40;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("-- {{ip:port}} {{auth}} {{name}} Option<{{model}}>");
        return;
    }
    let model_path = if args.len() < 5 {
        None
    } else {
        Some(args[4].clone())
    };

    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(ClientPlugin::new(ClientConfig::default(), protocol_builder()))
        .add_startup_system(model::add_stateful)

        .insert_resource(ClientResources { url: args[1].clone(), code: args[2].clone(), name: args[3].clone(), model_path: model_path })
        .add_startup_system(connect_client)

        .add_state::<GameState>()
        .add_system(model::message_event.in_set(OnUpdate(GameState::InGame)))
        .add_systems(
            (
                await_board,
                model::read_entity
            )
            .in_set(OnUpdate(GameState::Awaiting))
        )
        .add_systems(
            (
                despawn_board,
                model::read_entity
            )
            .in_set(OnUpdate(GameState::Despawning))
        )
        .add_system(
            notify_server
            .in_schedule(OnEnter(GameState::Despawning))
        )

        .run();
}

#[derive(Resource)]
pub struct ClientResources {
    pub url: String,
    pub code: String,
    pub name: String,
    pub model_path: Option<String>
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Awaiting, // make sure we have entities so we can be player 1
    Despawning, // make sure we release entities so we can have a restart
    InGame
}

pub fn connect_client(
    mut info: ResMut<ClientResources>,

    mut client: Client
) {
    client.auth(Auth::new(&info.name, &info.code));
    if !info.url.starts_with("http://") {
        info.url = "http://".to_string() + info.url.as_str();
    }
    println!("Connecting to {} with {} as {}", info.url, info.code, info.name);

    let socket = webrtc::Socket::new(&info.url, client.socket_config());
    client.connect(socket);
}

pub fn await_board(
    tiles: Query<&Tile>,

    mut game_state: ResMut<NextState<GameState>>,
    mut client: Client
) {
    if tiles.iter().count() == SQUARES {
        println!("Tiles populated to {}", SQUARES);
        game_state.set(GameState::InGame);
        client.send_message::<PlayerActionChannel, Ready>(&Ready);
    }
}

pub fn despawn_board(
    stateful: NonSend<StatefulInformation>,

    mut game_state: ResMut<NextState<GameState>>,
) {
    if stateful.entity != 0 { // should never be possible, since 0th entity is usually the window
        println!("Moving to await");
        game_state.set(GameState::Awaiting);
    }
}

pub fn notify_server(
    mut client: Client
) {
    client.send_message::<PlayerActionChannel, Finish>(&Finish); // Let the server know that we are open to messages after saving model
}