use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
mod setup; mod menu; mod rolling;

// ? Do we need all these game states
/*
    All the different stages of the game
    (1) Choosing game settings and preparing the board
    (2) Rolling the dice and moving a piece [Loop Starts]
    (3) Initiating a tile's action, like paying rent
        or buying a property, action falls down to 4
    (4) Individual player's action, like buying houses
        or trading properties, pickup of 3 [Loop Ends, prediction required]
    (5) The end of the game where we can show total
        results and return to the Menu state
*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    Menu, // 1
    Rolling, // 2
    TileAction, // 3
    PlayerAction, // 4
    Results // 5
}

fn main() {
    App::new()
        .add_state(GameState::Menu)
        .add_plugin(setup::SetupPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(menu::MainMenuPlugin)
        .add_plugin(rolling::RollingPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .run();
}