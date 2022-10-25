use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use crate::{GameState, setup::{CurrentPlayer, GameSettings}, player::{PlayerBundle, Money, TokenPosition, PlayerId, HeldJailFree, IsComputer, IsJailed}};

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
            .init_resource::<CurrentPlayer>()
            .add_plugin(EguiPlugin)
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(main_menu));
    }
}

// ! Testing with egui right now, might be a bit easier to read and re-use
fn main_menu(
    mut ctx: ResMut<EguiContext>,
    mut player_count: ResMut<CurrentPlayer>,
    mut settings: ResMut<GameSettings>,
    mut state: ResMut<State<GameState>>,
    mut commands: Commands
) {
    egui::Area::new("MainMenu")
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(ctx.ctx_mut(), |ui| {
            ui.horizontal(|horz| {
                // * Eventually change this for larger board sizes
                if horz.add(egui::Button::new("-")).clicked() {
                    if player_count.1 != 2 { player_count.1 -= 1; }
                }
                horz.label(player_count.1.to_string());
                if horz.add(egui::Button::new("+")).clicked() {
                    if player_count.1 != 6 { player_count.1 += 1; }
                }
            });
            ui.horizontal(|horz| {
                horz.checkbox(&mut settings.1, "Debt");
                horz.checkbox(&mut settings.2, "Sell");
                horz.checkbox(&mut settings.6, "Tax");
            });
            ui.horizontal(|horz| {
                horz.checkbox(&mut settings.3, "Homes");
                horz.checkbox(&mut settings.7, "Jail");
                horz.checkbox(&mut settings.8, "Auciton");
            });
            ui.horizontal(|horz| {
                horz.checkbox(&mut settings.4, "Chance");
                horz.checkbox(&mut settings.5, "Chest");
                horz.checkbox(&mut settings.0, "Visual");
            });
            if ui.add(egui::Button::new("Start")).clicked() {
                for i in 0..player_count.1 {
                    commands.spawn_bundle(PlayerBundle {
                        money: Money(1500),
                        token_position: TokenPosition(0, 0),
                        player_id: PlayerId(i),
                        held_jail_free: HeldJailFree(0),
                        is_computer: IsComputer(false),
                        is_jailed: IsJailed(false)
                    }).insert(Name::new(format!("Player {}", i)));
                }
                state.set(GameState::Rolling).unwrap();
            }
        });
}
