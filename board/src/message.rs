use bevy::prelude::*;
use monai_store::{transfer::{Forfeit, PlayerActionChannel, BuyOwnable}, tile::{Chance, Tile, Corner, Tier}, player::{Money, Position}};
use naia_bevy_server::{Server, events::MessageEvents};
use crate::state::Players;

pub fn message_receive(
    mut players: ResMut<Players>,

    mut event_reader: EventReader<MessageEvents>,
    mut tiles: Query<(Entity, &mut Tile, Option<&Corner>, Option<&Chance>), (Without<Money>, Without<Position>)>,
    mut tokens: Query<(Entity, &mut Money, &Position), (Without<Tile>, Without<Corner>, Without<Chance>)>,

    server: Server,
    mut commands: Commands
) {
    for events in event_reader.iter() {
        for (key, _message) in events.read::<PlayerActionChannel, Forfeit>() {
            if *players.current_player_key() == key { // make sure we aren't on the forfeit player's turn
                players.next_player();
                // run next player code
            }

            let entity_bits = players.list.remove(&key).expect("Non existant player on channel").to_bits();
            players.name.remove(&key);

            tiles.iter_mut().for_each(|(_, mut relinquish_tile, _, _)| {
                if *relinquish_tile.owner == Some(entity_bits) {
                    *relinquish_tile.owner = None;
                    *relinquish_tile.tier = Tier::None;
                }
            });

            commands.get_entity(Entity::from_bits(entity_bits)).expect("Non existant player on channel").despawn_recursive();
        }

        for (key, _message) in events.read::<PlayerActionChannel, BuyOwnable>() {
            let (player_token, mut money, position) = tokens.get_mut(players.list[&key]).expect("Could not find player from key on buy");
            let (_entity, mut tile, _, _) = tiles.get_mut(Entity::from_bits(*position.tile)).expect("Player is not on a space");

            *money.worth -= *tile.cost;
            *tile.owner = Some(player_token.to_bits());
            *tile.tier = Tier::Owned;
        }
    }
}