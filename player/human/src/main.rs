use bevy::prelude::*;
use naia_bevy_client::{Client, ClientConfig, Plugin as ClientPlugin, Stage, events::{SpawnEntityEvent}};
use monai_store::{protocol_builder, Auth};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ClientPlugin::new(ClientConfig::default(), protocol_builder()))
        .add_plugin(WorldInspectorPlugin)

        .add_startup_system(initialize_client)
        .run();
}

fn initialize_client(
    mut _commands: Commands,
    mut client: Client
) {
    client.auth(Auth::new("GUH", "MONAI"));
    client.connect("http://127.0.0.1:1095");
}

fn on_spawn_entity(
    mut event_reader: EventReader<SpawnEntityEvent>
) {
    for SpawnEntityEvent(entity) in event_reader.iter() {
        info!("Heard entity spawn");
    }
}