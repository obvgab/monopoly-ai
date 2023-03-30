use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::state::{Players, GameState};

#[derive(Resource)]
pub struct BoardConfiguration {
    pub polygonal_board: bool,
    pub corners: i32,
    pub squares: i32,
}

pub fn gui(
    players: Res<Players>,
    mut configuration: ResMut<BoardConfiguration>,
    mut game_state: ResMut<NextState<GameState>>,

    mut contexts: EguiContexts,
) { 
    

    egui::Area::new("Main Menu").show(contexts.ctx_mut(), |ui| {
        ui.label("Board");
        ui.separator();

        ui.checkbox(&mut configuration.polygonal_board, "Polygon");
        if configuration.polygonal_board {
            ui.add(egui::Slider::new(&mut configuration.corners, 4..=360).text("Corners"));
        }
        let step = configuration.corners as f64; // Arbitrary numbers for now, just making sure division is easy
        let minimum = configuration.corners * 1;
        let maximum = configuration.corners * 500;
        ui.add(egui::Slider::new(&mut configuration.squares, minimum..=maximum).text("Squares").step_by(step));

        ui.label("Players");
        ui.separator();

        for (_player, name) in players.name.iter() {
            ui.horizontal(|row| {
                row.label(name);
            });
        }

        ui.separator();

        if ui.button("Start").clicked() {
            game_state.set(GameState::InGame);
        }
    });
}