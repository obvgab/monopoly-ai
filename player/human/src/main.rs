use bevy::prelude::*;
use naia_bevy_client::{Client, ClientConfig, Plugin as ClientPlugin, events::{SpawnEntityEvent}, transport::webrtc};
use monai_store::{protocol_builder, Auth};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ClientPlugin::new(ClientConfig::default(), protocol_builder()))
        .add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin::new())

        .add_startup_system(initialize_client)
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