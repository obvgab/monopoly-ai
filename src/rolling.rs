use bevy::prelude::*;
use crate::{*, setup::*, player::*, tile::*};
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
    players: Res<Players>,
    tile_query: Query<(&TileType)>,
    mut player_query: Query<(&mut TokenPosition)>,
    mut state: ResMut<State<GameState>>
) {
    // Handle being in Jail
    let dice = (rand::thread_rng().gen_range(1..=6), rand::thread_rng().gen_range(1..=6));
    println!("Player id: {:?}, Total players: {}, Random Roll: {:?}", players.ids[players.current], players.ids.len(), dice);

    let current_player: Entity = players.ids[players.current];
    let mut position = player_query.get_mut(current_player).unwrap();
    let total_tiles = tile_query.iter().len();

    position.previous = position.current;
    position.current += dice.0 + dice.1;
    if position.current > total_tiles as i32 { position.current %= total_tiles as i32 - 1; }
    state.set(GameState::Action).unwrap();
}
