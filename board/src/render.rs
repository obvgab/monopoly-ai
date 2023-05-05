use bevy::prelude::*;
use monai_store::{player::Position, tile::Tile};


pub fn render_position(
    mut player_positions: Query<(&mut Transform, &Position)>,
    tile_positions: Query<&Transform, Without<Position>>
) {
    for (mut player, point) in player_positions.iter_mut() {
        if let Ok(tile) = tile_positions.get(Entity::from_bits(*point.tile)) {
            if tile.translation + Vec3::Z != player.translation {
                println!("{:?} : {:?}", player.translation.to_array(), tile.translation.to_array());
                player.translation = tile.translation + Vec3::Z;
            }
        }
    }
}

pub fn render_owner(
    players: Query<&Handle<ColorMaterial>, Without<Tile>>,
    tiles: Query<(&Handle<ColorMaterial>, &Tile), Without<Position>>,

    mut materials: ResMut<Assets<ColorMaterial>>
) {
    for (sprite, tile)  in tiles.iter() {
        if let Some(owner) = *tile.owner {
            if let Ok(player) = players.get(Entity::from_bits(owner)) {
                let player_color = materials.get(player).expect("Handle dangling").color;
                materials.get_mut(sprite).expect("Handle dangling").color = player_color;
            }
        } else {
            materials.get_mut(sprite).expect("Handle dangling").color = Color::hex("#1e1e2e").expect("Hex should be valid");
        }
    }
}