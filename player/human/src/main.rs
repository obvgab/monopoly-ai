use bevy::prelude::*;
use naia_bevy_client::{ClientConfig, Plugin as ClientPlugin};
use monai_store::{protocol_builder};

mod control;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ClientPlugin::new(ClientConfig::default(), protocol_builder()))
        .add_plugin(bevy_egui::EguiPlugin)

        .insert_resource(control::StatefulInformation {
            is_connected: false,
            name: "".into(),
            code: "".into(),
            url: "".into(),
            can_buy: false,
            can_sell: false,
            can_end: false,
            entity: 0,
            started: false
        })
        .add_systems(
            (
                control::gui,
                control::begin_turn
            )
        )

        .run();
}