use bevy::prelude::*;
use naia_bevy_client::{ClientConfig, Plugin as ClientPlugin, Client, transport::webrtc};
use monai_store::{protocol_builder, Auth};
use std::env;

mod model;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("-- {{ip:port}} {{auth}} {{name}}");
        return;
    }

    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(ClientPlugin::new(ClientConfig::default(), protocol_builder()))
        .add_startup_system(model::add_stateful)

        .insert_resource(ClientResources { url: args[1].clone(), code: args[2].clone(), name: args[3].clone() })
        .add_startup_system(connect_client)

        .add_system(model::message_event)

        .run();
}

#[derive(Resource)]
pub struct ClientResources {
    pub url: String,
    pub code: String,
    pub name: String
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