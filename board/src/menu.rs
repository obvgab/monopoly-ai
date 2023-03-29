use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub fn gui(
    mut contexts: EguiContexts
) {
    egui::Window::new("Hello, world").show(contexts.ctx_mut(), |ui| {
        ui.label("Guh");
    });
}