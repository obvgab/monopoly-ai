use bevy::prelude::*;
use naia_bevy_client::{ClientConfig, Plugin as ClientPlugin};
use monai_store::{protocol_builder};

mod model;

fn main() {
    App::new()
    .add_plugins(MinimalPlugins)
    .add_plugin(ClientPlugin::new(ClientConfig::default(), protocol_builder()))
    .add_startup_system(model::add_stateful)
    .run();
}
