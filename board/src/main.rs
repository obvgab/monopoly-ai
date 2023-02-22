use bevy::{prelude::*, utils::HashMap};
use naia_bevy_server::{Server, ServerAddrs, Plugin as ServerPlugin, ServerConfig, Stage as ServerStage, events::{AuthEvents, ConnectEvent}, UserKey, RoomKey};
use monai_store::{protocol_builder, Auth};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ServerPlugin::new(ServerConfig::default(), protocol_builder()))
        .add_plugin(WorldInspectorPlugin)

        .add_system_to_stage(ServerStage::Tick, tick)
        .add_system_to_stage(CoreStage::PreUpdate, authorize_player)
        .add_system_to_stage(CoreStage::PreUpdate, connect_player)
        .add_startup_system(initialize_server)
        .run();
}

/*
 * Important data structures
 */
#[derive(Resource)]
struct Code {
    value: String,
    game_room: RoomKey
}

#[derive(Resource)]
pub struct Players {
    pub list: HashMap<UserKey, Entity>,
    pub current: Option<UserKey>,
    pub name: HashMap<UserKey, String>
}

impl Players { // we might not **need** to deref here
    pub fn current_player(&self) -> (UserKey, Entity) {
        let player = self.list.get_key_value(&self.current.unwrap()).unwrap();
        return (*player.0, *player.1);
    }

    pub fn current_player_entity(&self) -> Entity {
        return *self.list.get(&self.current.unwrap()).unwrap();
    }

    pub fn current_player_key(&self) -> UserKey {
        return *self.list.get_key_value(&self.current.unwrap()).unwrap().0;
    }
    
    pub fn next_player(&mut self) {
        let counter: Vec<UserKey> = self.list.keys().cloned().collect();

        let mut current_position = counter.iter().position(|&key| key == self.current.unwrap()).unwrap();
        let total_size = counter.len();

        if current_position == total_size - 1 {
            current_position = 0;
        } else {
            current_position += 1;
        }

        self.current = Some(counter[current_position]);
    }

    pub fn initial_player(&mut self) { // Assumes 'list' has been populated
        let counter: Vec<UserKey> = self.list.keys().cloned().collect();

        self.current = Some(counter[0]);
    }
}

/*
 * Server Handling
 */
fn initialize_server(
    mut commands: Commands,
    mut server: Server
) {
    let address = ServerAddrs::new( // Σ ascii("monai") = 1096
        "127.0.0.1:1095".parse().expect("Could not parse signal local address"),
        "127.0.0.1:1096".parse().expect("Could not parse WebRTC local address"),
        "http://127.0.0.1:1096"
    );

    server.listen(&address);
    
    // Make this random later
    commands.insert_resource(Players { list: HashMap::new(), current: None, name: HashMap::new() });
    commands.insert_resource(Code { value: "MONAI".to_string(), game_room: server.make_room().key() });
    
    info!("Naia server initialized");
}

fn tick(
    mut server: Server
) {
    server.send_all_updates();
}

/*
 * Server Events
 */
fn authorize_player(
    mut event_reader: EventReader<AuthEvents>,

    code: Res<Code>,
    mut players: ResMut<Players>,

    mut server: Server
) {
    for event in event_reader.iter() {
        for (key, auth) in event.read::<Auth>() {
            if players.name.contains_key(&key) {
                server.reject_connection(&key);
                info!("Declined connection from player {}, key already authorized", auth.name);
                continue;
            }

            if auth.code != code.value  {
                server.reject_connection(&key);
                info!("Declined connection from player {}, code invalid", auth.name);
                continue;
            }

            server.accept_connection(&key);
            info!("Authorized connection from player {}", auth.name);
            players.name.insert(key, auth.name);
        }
    }
}

fn connect_player(
    mut event_reader: EventReader<ConnectEvent>,

    code: Res<Code>,
    mut players: ResMut<Players>,

    mut server: Server
) {
    for ConnectEvent(key) in event_reader.iter() { // needs player components
        let user = server.user_mut(key).enter_room(&code.game_room).address();
        let entity = server.spawn().enter_room(&code.game_room).id();
        players.list.insert(*key, entity);

        info!("Connected and spawned entity for {}, {}", players.name[key], user);
    }
}