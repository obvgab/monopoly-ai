use bevy::prelude::*;
use naia_bevy_server::{Plugin as ServerPlugin, ReceiveEvents, ServerConfig};
use monai_store::protocol_builder;

mod server;
mod state;
mod menu;
mod generator;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ServerPlugin::new(ServerConfig::default(), protocol_builder()))
        .add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin::new()) // Eventually add raw egui after debugging

        .add_state::<state::GameState>()
        .add_system(menu::gui.in_set(OnUpdate(state::GameState::Menu)))
        .add_systems(
            (
                generator::generate_board,
                generator::initialize_board
            )
            .chain()
            .in_schedule(OnEnter(state::GameState::InGame))
        )
        .add_system(generator::reset_board.in_schedule(OnExit(state::GameState::InGame)))

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