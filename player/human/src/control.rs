use bevy::{prelude::*};
use bevy_egui::{egui, EguiContexts};
use monai_store::{Auth, transfer::{BoardUpdateChannel, BeginTurn, SendPlayer, StartGame, PlayerActionChannel, BuyOwnable, SellOwnable, Forfeit}, player::{Action, Money, Position}, tile::{Tile, Chance, Corner}};
use naia_bevy_client::{Client, transport::webrtc, events::MessageEvents};

#[derive(Resource)]
pub struct StatefulInformation {
    pub is_connected: bool,
    pub name: String,
    pub code: String,
    pub url: String,
    pub can_buy: bool,
    pub can_sell: bool,
    pub entity: u64,
    pub started: bool,
}

pub fn gui( // separate this into multiple functions later
    mut stateful: ResMut<StatefulInformation>,

    tiles: Query<(Entity, &mut Tile, Option<&Corner>, Option<&Chance>), (Without<Money>, Without<Position>)>,
    tokens: Query<(Entity, &mut Money, &Position), (Without<Tile>, Without<Corner>, Without<Chance>)>,

    mut client: Client,
    mut contexts: EguiContexts
) {
    egui::Area::new("Main Console").show(contexts.ctx_mut(), |ui| {
        if !stateful.is_connected {
            ui.label("Connect");
            ui.separator();

            ui.horizontal(|row| {
                row.label("Address: ");
                row.text_edit_singleline(&mut stateful.url);
            });
            
            ui.horizontal(|row| {
                row.label("Name: ");
                row.text_edit_singleline(&mut stateful.name);
                row.label("Code: ");
                row.text_edit_singleline(&mut stateful.code);
            });

            if !stateful.name.is_empty()
                && !stateful.code.is_empty()
                && !stateful.url.is_empty()
                && ui.button("Connect").clicked() {
                stateful.is_connected = true;
                client.auth(Auth::new(&stateful.name, &stateful.code));
                if !stateful.url.starts_with("http://") {
                    stateful.url = "http://".to_string() + stateful.url.as_str();
                }

                let socket = webrtc::Socket::new(&stateful.url, client.socket_config());
                client.connect(socket);
            }
        } else if !stateful.started || stateful.entity == 0 {
            ui.label("Waiting for game...");
        } else {
            // this may cause issues, but the entity bits should be synchronized between client and server in our case
            let (_, money, position) = tokens.get(Entity::from_bits(stateful.entity)).expect("Could not find active player");

            ui.label(format!("Player ID: {:#?}", Entity::from_bits(stateful.entity)));
            ui.label(format!("Money: {}", *money.worth));

            ui.horizontal(|row| {
                row.label(format!("Space {:#?}", Entity::from_bits(*position.tile))); // replace with names later
                if stateful.can_buy && row.button("Buy").clicked() {
                    client.send_message::<PlayerActionChannel, BuyOwnable>(&BuyOwnable);
                }
            });

            ui.label("Properties");
            ui.separator();
            tiles.for_each(|(entity, tile, _, _)| { // add homes/houses later
                if *tile.owner == Some(stateful.entity) {
                    ui.horizontal(|row| {
                        row.label(format!("{:#?}", entity));
                        if row.button("Sell").clicked() {
                            // also should match entities
                            client.send_message::<PlayerActionChannel, SellOwnable>(&SellOwnable { id: entity.to_bits() });
                        }
                    });
                }
            });

            ui.separator();
            if ui.button("Forfeit").clicked() {
                client.send_message::<PlayerActionChannel, Forfeit>(&Forfeit);
            }
        }
    });
}

pub fn begin_turn(
    mut stateful: ResMut<StatefulInformation>,

    mut event_reader: EventReader<MessageEvents>
) {
    for events in event_reader.iter() {
        for turn in events.read::<BoardUpdateChannel, BeginTurn>() {
            for action in turn.available_actions {
                match action {
                    Action::Sell => {
                        stateful.can_sell = true; // eventually evaluate this ourselves for async
                    },
                    Action::Purchase => {
                        stateful.can_buy = true;
                    }
                    _ => {} // add more later
                }
            }
        }

        for entity in events.read::<BoardUpdateChannel, SendPlayer>() {
            stateful.entity = entity.id;
        }

        for _start in events.read::<BoardUpdateChannel, StartGame>() {
            stateful.started = true;
        }
    }
}