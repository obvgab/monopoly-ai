use bevy::prelude::*;
use crate::{GameState, setup::CurrentPlayer};

pub struct RollingPlugin;

impl Plugin for RollingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Rolling)
            .with_system(start_roll));
    }
}

fn start_roll(
    mut _commands: Commands,
    current_player: Res<CurrentPlayer>
) {
    info!("Player index: {}, Total players: {}", current_player.0, current_player.1);
}