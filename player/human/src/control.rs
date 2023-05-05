use bevy::{prelude::*};
use bevy_egui::{egui, EguiContexts};
use monai_store::{Auth, transfer::{BoardUpdateChannel, BeginTurn, SendPlayer, StartGame, PlayerActionChannel, BuyOwnable, SellOwnable, Forfeit, EndTurn, EndGame, Ready}, player::{Action, Money, Position, ServerPlayer}, tile::{Tile, Chance, Corner, ServerSide}};
use naia_bevy_client::{Client, transport::webrtc, events::MessageEvents};

#[derive(Resource)]
pub struct StatefulInformation {
    pub is_connected: bool,
    pub name: String,
    pub code: String,
    pub url: String,
    pub can_buy: bool,
    pub can_sell: bool,
    pub can_end: bool,
    pub entity: u64,
    pub started: bool,
    pub ready: bool,
}

pub fn gui( // separate this into multiple functions later
    mut stateful: ResMut<StatefulInformation>,

    tiles: Query<(Entity, &mut Tile, Option<&Corner>, Option<&Chance>, &ServerSide), (Without<Money>, Without<Position>)>,
    tokens: Query<(Entity, &mut Money, &Position, &ServerPlayer), (Without<Tile>, Without<Corner>, Without<Chance>)>,

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
        } else if !stateful.started || stateful.entity == 0 || !stateful.ready {
            ui.label("Waiting for game...");
            if ui.button("Ready").clicked() {
                if !stateful.ready {
                    client.send_message::<PlayerActionChannel, Ready>(&Ready);
                }
                stateful.ready = true;
            }
        } else {
            let (_entity, money, position, _server_entity) = {
                let mut last: Option<(Entity, &Money, &Position, &ServerPlayer)> = None;
                
                for x in &tokens {
                    if *x.3.id == stateful.entity {
                        last = Some(x);
                    }
                }

                last.expect("Couldn't find current player from server reference")
            };

            ui.label(format!("Player ID: {:#?}", stateful.entity));
            ui.label(format!("Money: {}", *money.worth));

            ui.horizontal(|row| {
                row.label(format!("Space {:#?}", *position.tile)); // replace with names later
                if stateful.can_buy && row.button("Buy").clicked() {
                    client.send_message::<PlayerActionChannel, BuyOwnable>(&BuyOwnable);
                }
            });

            ui.label("Properties");
            ui.separator();
            tiles.for_each(|(_, tile, _, _, server_side)| { // add homes/houses later
                if *tile.owner == Some(stateful.entity) {
                    ui.horizontal(|row| {
                        row.label(format!("{:#?}", *server_side.id));
                        if stateful.can_sell && row.button("Sell").clicked() {
                            client.send_message::<PlayerActionChannel, SellOwnable>(&SellOwnable { id: *server_side.id });
                        }
                    });
                }
            });

            ui.separator();
            ui.horizontal(|row| {
                if row.button("Forfeit").clicked() { // fix lose event
                    client.send_message::<PlayerActionChannel, Forfeit>(&Forfeit);
                }
                if stateful.can_end && row.button("End Turn").clicked() {
                    client.send_message::<PlayerActionChannel, EndTurn>(&EndTurn);
                    stateful.can_buy = false;
                    stateful.can_sell = false;
                    stateful.can_end = false;
                }
            });
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

                stateful.can_end = true;
            }
        }

        for entity in events.read::<BoardUpdateChannel, SendPlayer>() {
            stateful.entity = entity.id;
        }

        for _ in events.read::<BoardUpdateChannel, StartGame>() {
            stateful.started = true;
        }

        for _ in events.read::<BoardUpdateChannel, EndGame>() {
            stateful.started = false;
            stateful.ready = false;
            stateful.entity = 0;
        }
    }
}