use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use crate::{*, setup::*, player::*};

pub struct MainMenuPlugin;

/*
    Implementing the Plugin trait for MainMenuPlugin
    When loaded, MainMenuPlugin will:
    (1) Craft the UI
    (2) Load common Resources
    (3) Initiate the UI buttons
*/
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameSettings>()
            .init_resource::<Players>()
            .add_plugin(EguiPlugin)
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(main_menu));
    }
}

// ! Testing with egui right now, might be a bit easier to read and re-use
fn main_menu(
    mut ctx: ResMut<EguiContext>,
    mut players: ResMut<Players>,
    mut settings: ResMut<GameSettings>,
    mut state: ResMut<State<GameState>>,
    mut commands: Commands
) {
    let mut player_count = 2;

    egui::Area::new("MainMenu")
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(ctx.ctx_mut(), |ui| {
            ui.horizontal(|horz| {
                // * Eventually change this for larger board sizes
                if horz.add(egui::Button::new("-")).clicked() {
                    if player_count != 2 { player_count -= 1; }
                }
                horz.label(player_count.to_string());
                if horz.add(egui::Button::new("+")).clicked() {
                    if player_count != 6 { player_count += 1; }
                }
            });
            ui.horizontal(|horz| {
                horz.checkbox(&mut settings.debt, "Debt");
                horz.checkbox(&mut settings.sell, "Sell");
                horz.checkbox(&mut settings.tax, "Tax");
            });
            ui.horizontal(|horz| {
                horz.checkbox(&mut settings.homes, "Homes");
                horz.checkbox(&mut settings.jail, "Jail");
                horz.checkbox(&mut settings.auction, "Auciton");
            });
            ui.horizontal(|horz| {
                horz.checkbox(&mut settings.chance, "Chance");
                horz.checkbox(&mut settings.chest, "Chest");
                horz.checkbox(&mut settings.visual, "Visual");
            });
            if ui.add(egui::Button::new("Start")).clicked() {
                for i in 0..player_count {
                    let entity = commands.spawn_bundle(PlayerBundle {
                        money: Money { worth: 1500 },
                        token_position: Token { current: 0, previous: 0 },
                        held_jail_free: JailFree { held: 0 },
                    })
                    .insert(Name::new(format!("Player {}", i)))
                    .id();

                    players.ids.push(entity);
                }

                state.set(GameState::Rolling).unwrap();
            }
        });
}
