use bevy::prelude::*;
use naia_bevy_client::{ClientConfig, Plugin as ClientPlugin, events::{SpawnEntityEvent, InsertComponentEvents, UpdateComponentEvents}, ReceiveEvents};
use monai_store::{protocol_builder};

mod control;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ClientPlugin::new(ClientConfig::default(), protocol_builder()))
        .add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin::default()) // Eventually replace when inspector is unnecessary

        .add_systems(
            (
                on_spawn_entity,
                on_insert_component,
                on_update_component
            )
            .chain()
            .in_set(ReceiveEvents)
        )
        .insert_resource(control::StatefulInformation {
            is_connected: false,
            name: "".into(),
            code: "".into(),
            url: "".into(),
            can_buy: false,
            can_sell: false,
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

fn on_spawn_entity(
    mut event_reader: EventReader<SpawnEntityEvent>
) {
    for _event in event_reader.iter() {
        info!("Heard entity spawn");
    }
}

fn on_insert_component(
    mut event_reader: EventReader<InsertComponentEvents>
) {
    for _event in event_reader.iter() {
        info!("Heard insert component");
    }
}

fn on_update_component(
    mut event_reader: EventReader<UpdateComponentEvents>
) {
    for _event in event_reader.iter() {
        info!("Heard update component");
    }
}