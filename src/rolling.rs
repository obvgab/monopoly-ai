use bevy::prelude::*;
use crate::{GameState, setup::CurrentPlayer, player::{TokenPosition, PlayerId}, tile::TotalTiles};
use rand::Rng;

pub struct RollingPlugin;

impl Plugin for RollingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Rolling)
            .with_system(start_roll));
    }
}

fn start_roll(
    mut _commands: Commands,
    current_player: Res<CurrentPlayer>,
    total_tiles: Res<TotalTiles>,
    mut players: Query<(&PlayerId, &mut TokenPosition), With<PlayerId>>,
    mut state: ResMut<State<GameState>>
) {
    let dice = (rand::thread_rng().gen_range(1..7), rand::thread_rng().gen_range(1..7));
    info!("Player index: {}, Total players: {}, Random Roll: {:?}", current_player.0, current_player.1, dice);
    let mut value = players.iter_mut().filter(|x| x.0.0 == current_player.0).nth(0).unwrap().1;
    value.1 = value.0;
    value.0 += dice.0 + dice.1;
    if value.0 >= total_tiles.0 - 1 { value.0 = value.0 - 39; }
    state.set(GameState::TileAction).unwrap();
}