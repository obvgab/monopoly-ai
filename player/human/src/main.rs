use bevy::prelude::*;
use naia_bevy_client::{Client, ClientConfig, Plugin as ClientPlugin, events::{SpawnEntityEvent, InsertComponentEvents}, transport::webrtc, ReceiveEvents};
use monai_store::{protocol_builder, Auth};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ClientPlugin::new(ClientConfig::default(), protocol_builder()))
        .add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin::default())

        .add_systems(
            (
                on_spawn_entity,
                on_insert_component
            )
            .chain()
            .in_set(ReceiveEvents)
        )
        .add_startup_system(initialize_client)
        .register_type::<monai_store::player::Money>()

        .run();
}

fn initialize_client(
    mut _commands: Commands,
    mut client: Client
) {
    client.auth(Auth::new("GUH", "MONAI"));

    let socket = webrtc::Socket::new("http://127.0.0.1:1095", client.socket_config());
    client.connect(socket);
}

fn on_spawn_entity(
    mut event_reader: EventReader<SpawnEntityEvent>
) {
    for event in event_reader.iter() {
        info!("Heard entity spawn");
    }
}

fn on_insert_component(
    mut event_reader: EventReader<InsertComponentEvents>
) {
    for event in event_reader.iter() {
        info!("Heard insert component");
    }
}