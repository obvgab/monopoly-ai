use bevy::prelude::*;
use crate::{*, setup::*, tile::*, player::*};
use bevy_egui::{egui, EguiContext};

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Debounce>()
            .add_system_set(SystemSet::on_update(GameState::Action)
                .with_system(player_interaction));
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

fn player_interaction(
    mut players: ResMut<Players>,

    mut player_query: Query<(&mut Money, &mut TokenPosition, &mut HeldJailFree, Option<&Jailed>, Option<&Computer>)>,
    mut tile_query: Query<(Entity, &TilePosition, &TileType, Option<&mut Owner>, Option<&PairId>, &Cost, &Tax, &mut Tier)>,

    mut ctx: ResMut<EguiContext>,
    mut state: ResMut<State<GameState>>,
    mut debounce: ResMut<Debounce>,
    mut commands: Commands,
    settings: Res<GameSettings>
) {
    let player = players.ids[players.current];
    let (mut money, mut position, mut jail_free, jailed, computer) 
        = player_query.get_mut(player).unwrap();
    let (tile, space, building, owner, pair, cost, tax, tier) 
        = tile_query.iter_mut().find(|(_, x, _, _, _, _, _, _)| x.0 == position.current).unwrap(); // change to .current later

    if position.current < position.previous {
        println!(" Passed Go");
        money.worth += 200;
    }

    match building {
        TileType::Property => {
            println!(" Property index: {}, Owner: {}, Player money: {}", position.current, if owner.is_some() { "NONE" } else { owner.unwrap().id.id().to_string().as_str() }, money.worth);
            if owner.is_none() {
                start_purchase(player, money, tile, space, cost, ctx, state, debounce, commands, settings);
            } else if owner.unwrap().id != players.ids[players.current] {
                // Fine
            } else {
                println!("  Player property");
            }
        }
        TileType::Tax => {
            // Tax
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
}

fn start_purchase(
    player: Entity,
    mut money: Mut<Money>,

    tile: Entity,
    space: &TilePosition,
    cost: &Cost, 

    mut ctx: ResMut<EguiContext>,
    mut state: ResMut<State<GameState>>,
    mut debounce: ResMut<Debounce>,
    mut commands: Commands,
    settings: Res<GameSettings>,
) {
    if cost.initial > money.worth { return; } // auctioning

    egui::Area::new("PurchaseMenu")
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(ctx.ctx_mut(), |ui| {
            ui.heading(format!("Player {} Purchase", player.id())); // replace with a name later
            ui.label(format!("Do you wish to purchase tile {} for ${}?", space.0 + 1, cost.initial));
            ui.horizontal(|horz| {
                if horz.add(egui::Button::new("Yes")).clicked() && !debounce.lock {
                    money.worth -= cost.initial;
                    commands.entity(tile).insert(Owner { id: player });
                    debounce.lock = true;
                    
                    println!("  Purchase index: {}, Cost: {}, Money: {}", space.0, cost.initial, money.worth);
                } else if horz.add(egui::Button::new("No")).clicked() && !debounce.lock {
                    debounce.lock = true; // auctioning
                } else {
                    debounce.lock = false;
                }
            });
        });
}

fn fall_through_card(
    mut player: (Mut<Money>, Mut<TokenPosition>, &PlayerId, Mut<HeldJailFree>),
    
    settings: Res<GameSettings>,
    mut fall_through: ResMut<FallThroughState>,
    // Pass through for loop
    mut state: ResMut<State<GameState>>,
    mut current_player: ResMut<CurrentPlayer>,
    // Resource to handle submit buttons
    mut debounce: ResMut<Debounce> 
) {
    println!("  Card: {:?}, Multi: {:?}, Amount: {}", fall_through.2, fall_through.3, fall_through.1);
    
    match fall_through.2 {
        Card::JailFree => {
            println!("   Added jail free card");
            player.3.0 += 1;
            loop_players(player, settings, current_player, state, fall_through);
        }
        Card::Jail => {
            // JAIL or JAIL FREE
            loop_players(player, settings, current_player, state, fall_through);
        }
        Card::GoTile => {
            println!("   Going to tile {}", fall_through.1);
            // Abusing Multi::Player here to track if we pass Go
            if fall_through.3 == Multi::Player && fall_through.1 < player.1.0 {
                println!("    Passed Go");
                player.0.0 += 200;
            }
            player.1.0 = fall_through.1; player.1.1 = fall_through.1;
            loop_players(player, settings, current_player, state, fall_through);
        }
        Card::Collect | Card::Fine => {
            let mut base = fall_through.1;
            match fall_through.3 {
                Multi::Player => { base *= current_player.1; }
                Multi::Property => {

                }
                Multi::Buildings => {

                }
                _ => {}
            }
            if fall_through.2 == Card::Collect {
                player.0.0 += base;
            } else {
                player.0.0 -= base; // Bankruptcy is handled at loop
            }
            println!("    Card collect/fine: {}", base);
            loop_players(player, settings, current_player, state, fall_through);
        }
        _ => {}
    }

    
}

fn loop_players(
    mut player: (Mut<Money>, Mut<TokenPosition>, &PlayerId, Mut<HeldJailFree>, &IsComputer), 
    settings: Res<GameSettings>,
    mut current_player: ResMut<CurrentPlayer>,
    mut state: ResMut<State<GameState>>,
    mut fall_through: ResMut<FallThroughState>
) {
    if player.0.0 < 0 {
        if settings.1 {
            player.1.0 = -1; player.1.1 = -1; // -1 means bankrupt, handled by roll
        } else {

        }
        
    }
    if current_player.0 == current_player.1 - 1 { current_player.0 = 0; } else { current_player.0 += 1; }
    fall_through.0 = FallThroughAction::None; fall_through.1 = 0; fall_through.2 = Card::None; fall_through.3 = Multi::None;
    state.set(GameState::Rolling).unwrap();
}
