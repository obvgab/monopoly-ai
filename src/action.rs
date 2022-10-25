use bevy::prelude::*;
use crate::{
    GameState,
    setup::{CurrentPlayer, GameSettings},
    tile::{FallThroughState, TilePosition, TileType, PairId, Cost, Tax, Tier}, 
    player::{Money, PlayerId, TokenPosition, HeldJailFree, IsComputer}
};
use bevy_egui::{egui, EguiContext};

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<FallThroughState>()
            .init_resource::<Debounce>()
            .add_system_set(SystemSet::on_update(GameState::PlayerAction)
                .with_system(player_interaction));
    }
}

/*
    Enums for actions/cards from fall-through
    (1) Actions that can fall through TileAction
    (2) Cards that can be picked up during TileAction
    (3) The Multi* card conditions
*/
pub enum FallThroughAction { // 1
    Purchase, // Possibly purchase a property
    Debt, // Required tax/rent incurred debt, must mortgage/sell/trade
    Card, // Check a card provided
    Restricted, // In jail, must choose to roll/pay/card
    None
}

pub enum Card { // 2
    JailFree, // Holdable, Out of Jail Free
    Jail, // Go to Jail
    GoTile, // Go to Tile, $ GO
    Tile, // Go to Tile, !$ GO
    Fine, // Single or Multiplying -$
    Collect, // Single or Multiplying +$
    None // Default state
}

pub enum Multi { // 3
    Property, // Per property
    Buildings, // Per Building
    Player, // Per Player
    None // Not a multi
}

#[derive(Default)]
pub struct Debounce(bool);

impl Default for FallThroughState { fn default() -> Self { FallThroughState(FallThroughAction::None, 0, Card::None, Multi::None) } }

fn player_interaction(
    mut current_player: ResMut<CurrentPlayer>,
    mut player_tiles: Query<
        (&mut Money, &mut TokenPosition, &PlayerId, &mut HeldJailFree, &IsComputer),
        (With<PlayerId>, Without<TilePosition>),
    >,
    mut active_tiles: Query<
        (&TilePosition, &TileType, &mut PlayerId, &PairId, &Cost, &Tax, &mut Tier),
        With<TilePosition>,
    >,
    mut ctx: ResMut<EguiContext>,
    mut fall_through: ResMut<FallThroughState>,
    mut state: ResMut<State<GameState>>,
    mut debounce: ResMut<Debounce>,
    settings: Res<GameSettings>
) {
    let mut player = player_tiles.iter_mut().find(|x| x.2.0 == current_player.0).unwrap();
    let mut tile = active_tiles.iter_mut().find(|x| x.0.0 == player.1.0).unwrap();

    if player.4.0 == false {
        match fall_through.0 {
            FallThroughAction::Purchase => { fall_through_purchase(player, tile, ctx, fall_through, state, current_player, debounce); }
            _ => { loop_players(current_player, state, fall_through); }
        }
    } else { /* AI does not need UI, invoke specific commands */ }
}

fn fall_through_purchase( // Note that &mut becomes Mut<> in bevy queries
    mut player: (Mut<Money>, Mut<TokenPosition>, &PlayerId, Mut<HeldJailFree>, &IsComputer),
    mut tile: (&TilePosition, &TileType, Mut<PlayerId>, &PairId, &Cost, &Tax, Mut<Tier>), 
    mut ctx: ResMut<EguiContext>,
    // Pass through for loop
    mut fall_through: ResMut<FallThroughState>,
    mut state: ResMut<State<GameState>>,
    mut current_player: ResMut<CurrentPlayer>,
    // Resource to handle submit buttons
    mut debounce: ResMut<Debounce>
) {
    if tile.4.0 > player.0.0 { /* Handle properties that can't be purchased / auctioning? */ return; }
    egui::Area::new("PurchaseMenu")
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(ctx.ctx_mut(), |ui| {
            ui.heading(format!("Player {} Purchase", current_player.0 + 1));
            ui.label(format!("Do you wish to purchase tile {} for ${}?", tile.0.0 + 1, tile.4.0));
            ui.horizontal(|horz| {
                if horz.add(egui::Button::new("Yes")).clicked() && !debounce.0 {
                    player.0.0 -= tile.4.0;
                    tile.2.0 = player.2.0;
                    println!("  Purchase index: {}, Cost: {}, Money: {}", tile.0.0, tile.4.0, player.0.0);
                    debounce.0 = true; // Apply a debounce so that the .clicked() doesn't infinite
                    loop_players(current_player, state, fall_through);
                } else if horz.add(egui::Button::new("No")).clicked() && !debounce.0 {
                    /* auctioning? */
                    debounce.0 = true;
                    loop_players(current_player, state, fall_through);
                } else {
                    // GUH
                    debounce.0 = false;
                }
            });
        });
}



fn loop_players(
    mut current_player: ResMut<CurrentPlayer>,
    mut state: ResMut<State<GameState>>,
    mut fall_through: ResMut<FallThroughState>
) {
    if current_player.0 == current_player.1 - 1 { current_player.0 = 0; } else { current_player.0 += 1; }
    fall_through.0 = FallThroughAction::None; fall_through.1 = 0; fall_through.2 = Card::None; fall_through.3 = Multi::None;
    state.set(GameState::Rolling).unwrap();
    // This can also be where we check if someone has a monopoly / is bankrupt to skip them
}
