use bevy::prelude::*;
use bevy_inspector_egui::{WorldInspectorPlugin, RegisterInspectable};
mod setup; mod menu; mod rolling; mod player; mod tile; mod action;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
// We can probably swap this out for Events
pub enum GameState {
    Menu,
    Rolling,
    Action,
    Results
}

#[derive(Default)]
pub struct Players {
    ids: Vec<Entity>,
    current: usize
}

#[derive(Default)]
pub struct GameSettings {
    pub visual: bool,
    pub debt: bool,
    pub sell: bool,
    pub homes: bool,
    pub chance: bool,
    pub chest: bool,
    pub tax: bool,
    pub jail: bool,
    pub auction: bool
}

fn main() {
    App::new()
        .add_state(GameState::Menu)
        .add_plugin(setup::SetupPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(menu::MainMenuPlugin)
        .add_plugin(rolling::RollingPlugin)
        .add_plugin(tile::TilePlugin)
        .add_plugin(action::ActionPlugin)

        // * Debug instructions
        .add_plugin(WorldInspectorPlugin::new())

        // Player Debug
        .register_inspectable::<player::Money>()
        .register_inspectable::<player::Token>()
        .register_inspectable::<player::JailFree>()
        .register_inspectable::<player::Computer>()

        // Tile Debug
        .register_inspectable::<tile::Space>()
        .register_inspectable::<tile::TileType>()
        .register_inspectable::<tile::Owner>()
        .register_inspectable::<tile::Pair>()
        .register_inspectable::<tile::Cost>()
        .register_inspectable::<tile::Tax>()
        .register_inspectable::<tile::Tier>()
        
        .run();
}

// ! Redo the println!() statements--they're currently hard to read
// ! Have bankruptcy handled by a different system, looking at changes in money