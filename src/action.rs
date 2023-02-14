use bevy::prelude::*;
use bevy::ecs::query::QueryIter;
use crate::{*, setup::*, tile::*, player::*};
use bevy_egui::{egui, EguiContext};

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Debounce>()
            .add_system_set(SystemSet::on_update(GameState::Action)
                .with_system(token_landed));
    }
}

#[derive(PartialEq, Debug)]
pub enum Card {
    JailFree, // Holdable, Out of Jail Free
    Jail, // Go to Jail
    GoTile, // Go to Tile, $ GO with Multi, !$ GO without Multi
    Fine, // Single or Multiplying -$
    Collect, // Single or Multiplying +$
    None // Default state
}
impl Card {
    pub fn from_i32(val: i32) -> Card {
        match val {
            0 => { Card::JailFree }
            1 => { Card::Jail }
            2 => { Card::GoTile }
            3 => { Card::Fine }
            4 => { Card::Collect }
            _ => { Card::None }
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Multi {
    Property, // Per property
    Buildings, // Per Building
    Player, // Per Player
    None // Not a multi
}
impl Multi {
    pub fn from_i32(val: i32) -> Multi {
        match val {
            0 => { Multi::Property }
            1 => { Multi::Buildings }
            2 => { Multi::Player }
            _ => { Multi::None }
        }
    }
}

#[derive(Default)]
pub struct Debounce { lock: bool }

fn token_landed(
    mut players: ResMut<Players>,

    mut player_query: Query<(&mut Money, &mut Token, &mut JailFree, Option<&Jailed>, Option<&Computer>)>,
    tile_query: Query<(Entity, &Space, &TileType, Option<&Owner>, Option<&Pair>, &Cost, &Tax, &Tier)>,

    mut ctx: ResMut<EguiContext>,
    mut state: ResMut<State<GameState>>,
    mut debounce: ResMut<Debounce>,
    mut commands: Commands,
    settings: Res<GameSettings>
) {
    let acting_player = players.ids[players.current];

    let (mut money, mut position, mut jail_free, jailed, computer) 
        = player_query.get_mut(acting_player).expect("Can't find active player");
    let (tile, space, building, owner, pair, cost, tax, tier) 
        = tile_query.iter().find(|(_, x, _, _, _, _, _, _)| x.id == position.current).unwrap();

    if position.current < position.previous {
        println!(" Passed Go");
        money.worth += 200;
    }

    match building {
        TileType::Property => {
            println!(" Property index: {}, Player money: {}", position.current, money.worth);
            if owner.is_none() {
                println!("wowowoah");
                start_purchase(acting_player, money, tile, space, cost, ctx, debounce, commands, settings);
            } else if owner.unwrap().id != acting_player { // why not start_fine? because we need to the query multiple times
                println!("wo2");
                let mut monopoly = true;

                tile_query.for_each(|(_, _, _, query_owner, query_pair, _, _, _)| {
                    if query_pair.unwrap().id == pair.unwrap().id && !(query_owner.is_some() && query_owner.unwrap().id == owner.unwrap().id) {
                        monopoly = false;
                    }
                });
            
                let price = match tier {
                    Tier::Base => { if monopoly { tax.pair } else { tax.base } }
                    Tier::House(quantity) => { tax.home[quantity - 1] }
                    Tier::Hotel => { tax.hotel }
                    _ => { 0 }
                };

                money.worth -= price;
                let mut collector: Mut<Money> = player_query.get_component_mut(owner.unwrap().id).expect("Could not find Owner's worth");
                collector.worth += price;
                println!("  Owner money: {}, Rent: {}", collector.worth, price);
            } else {
                println!("  Player property");
            }
        }
        TileType::Tax => {
            money.worth -= tax.base;
            println!(" Tax Index: {}, Taxed: {}", space.id, tax.base);
        }
        TileType::Chance => {
            // Chance
        }
        TileType::CommunityChest => {
            // Chest
        }
        TileType::GoToJail => {
            // Jail
        }
        _ => {
            // Visiting jail or on free space
        }
    }

    loop_players(players, state);
}

#[inline(always)]
fn start_purchase(
    player: Entity,
    mut money: Mut<Money>,

    tile: Entity,
    space: &Space,
    cost: &Cost, 

    mut ctx: ResMut<EguiContext>,
    mut debounce: ResMut<Debounce>,
    mut commands: Commands,
    settings: Res<GameSettings>, // needed later for auctioning
) {
    if cost.initial > money.worth { return; } // auctioning

    egui::Area::new("PurchaseMenu")
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(ctx.ctx_mut(), |ui| {
            ui.heading(format!("Player {} Purchase", player.id())); // replace with a name later
            ui.label(format!("Do you wish to purchase tile {} for ${}?", space.id, cost.initial)); // replace with location later
            ui.horizontal(|horz| {
                if horz.add(egui::Button::new("Yes")).clicked() && !debounce.lock {
                    money.worth -= cost.initial;
                    commands.entity(tile).insert(Owner { id: player });
                    debounce.lock = true;
                    
                    println!("  Purchase index: {}, Cost: {}, Money: {}", space.id, cost.initial, money.worth);
                } else if horz.add(egui::Button::new("No")).clicked() && !debounce.lock {
                    debounce.lock = true; // auctioning
                } else {
                    debounce.lock = false;
                }
            });
        });
}

// ! MAYBE FIGURE THIS OUT
fn start_fine() {}

// ! FINISH THIS
fn start_card() {}

// ! REDO THIS
fn loop_players(
    mut players: ResMut<Players>,
    mut state: ResMut<State<GameState>>,
) {
    if players.current == players.ids.len() - 1 { players.current = 0 } else { players.current += 1; }
    state.set(GameState::Rolling).unwrap();
}
