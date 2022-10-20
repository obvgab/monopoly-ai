use bevy::prelude::*;

use crate::{GameState, tile::FallThroughState};

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(instantiate_actions)
            .add_system_set(SystemSet::on_update(GameState::PlayerAction));
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

pub enum Multi {
    Property, // Per property
    Buildings, // Per Building
    Player, // Per Player
    None // Not a multi
}

fn instantiate_actions(mut commands: Commands) {
    commands.insert_resource(FallThroughState(FallThroughAction::None, 0, Card::None, Multi::None));
}

fn player_action(
    fall_through: ResMut<FallThroughState>
) {
    
}
