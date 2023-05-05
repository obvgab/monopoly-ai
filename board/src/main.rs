//use std::env;
use bevy::{prelude::*};
use naia_bevy_server::{Plugin as ServerPlugin, ReceiveEvents, ServerConfig};
use monai_store::protocol_builder;

mod server;
mod state;
mod menu;
mod generator;
mod message;

const SQUARE_SIZE: f32 = 720.0;

fn main() {
    //let args: Vec<String> = env::args().collect();

    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    resolution: (SQUARE_SIZE, SQUARE_SIZE).into(),
                    ..default()
                }),
                ..default()
            }
        ))
        .add_plugin(ServerPlugin::new(ServerConfig::default(), protocol_builder()))
        //.add_plugin(bevy_egui::EguiPlugin)
        .add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin::new())

        .add_startup_system(init_camera)
        .add_state::<state::GameState>()
        .add_system(menu::gui.in_set(OnUpdate(state::GameState::Menu)))
        .add_systems(
            (
                generator::generate_board,
                generator::initialize_players
            )
            .chain()
            .in_schedule(OnEnter(state::GameState::InGame))
        )
        //.add_system(generator::reset_game.in_schedule(OnExit(state::GameState::InGame)))
        .add_systems(
            (
                message::message_receive,
                message::next_turn,
                message::reward_player,
                message::bankrupt_player,
            )
            .in_set(OnUpdate(state::GameState::InGame))
        )

        .add_systems(
            (
                server::authorize_player,
                server::connect_player,
                server::disconnect_player,
                server::tick
            )
            .chain()
            .in_set(ReceiveEvents)
        )
        .add_startup_system(server::initialize_server)

        .add_system(
            state::auto_reset
                .in_set(OnUpdate(state::GameState::AutoReset))
        )

        .add_event::<message::AwardPlayer>()
        .add_event::<message::NextTurn>()
        .add_event::<message::BankruptPlayer>();

    // if args.len() != 1 || args[0] != "headless" {
    //     app.add_startup_system(init_camera);
    // }

    app.run();
}

pub fn init_camera(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());
}