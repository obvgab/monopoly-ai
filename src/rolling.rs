use bevy::prelude::*;
use crate::{GameState, setup::CurrentPlayer, player::{TokenPosition, PlayerId}};
use rand::Rng;

pub struct RollingPlugin;

impl Plugin for RollingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Rolling) // ! Changed to on_update for now
            .with_system(start_roll));
    }
}

fn start_roll(
    mut _commands: Commands,
    current_player: Res<CurrentPlayer>,
    mut players: Query<(&PlayerId, &mut TokenPosition), With<PlayerId>>,
    mut state: ResMut<State<GameState>>
) {
    let dice = (rand::thread_rng().gen_range(1..7), rand::thread_rng().gen_range(1..7));
    info!("Player index: {}, Total players: {}, Random Roll: {:?}", current_player.0, current_player.1, dice);
    let mut player_current_tile: Option<Mut<TokenPosition>> = None;
    for (player, player_tile) in players.iter_mut() { if player.0 == current_player.0 { player_current_tile = Some(player_tile); } }
    if player_current_tile.is_some() { player_current_tile.unwrap().0 += dice.0 + dice.1; }
    state.set(GameState::TileAction).unwrap();
}