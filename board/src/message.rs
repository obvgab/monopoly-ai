use std::time::Instant;

use bevy::prelude::*;
use rand::Rng;
use monai_store::{transfer::{Forfeit, PlayerActionChannel, BuyOwnable, SellOwnable, EndTurn, BeginTurn, BoardUpdateChannel}, tile::{Chance, Tile, Corner, Tier}, player::{Money, Position, Action}};
use naia_bevy_server::{events::MessageEvents, Server, UserKey};
use crate::state::{Players, Tiles};

pub fn message_receive(
    mut players: ResMut<Players>,

    mut event_reader: EventReader<MessageEvents>,
    mut event_writer: EventWriter<NextTurn>,
    mut tiles: Query<(Entity, &mut Tile, Option<&Corner>, Option<&Chance>), (Without<Money>, Without<Position>)>,
    mut tokens: Query<(Entity, &mut Money, &Position), (Without<Tile>, Without<Corner>, Without<Chance>)>//,

    // mut commands: Commands
) {
    for events in event_reader.iter() {
        for (key, _message) in events.read::<PlayerActionChannel, Forfeit>() {
            let (_, mut money, _) = tokens.get_mut(*players.current_player_entity()).expect("Unable to find forfeiting player");
            *money.worth = -1;

            players.next_player();
            event_writer.send(NextTurn(Some(key)))
        }

        for (key, _message) in events.read::<PlayerActionChannel, BuyOwnable>() {
            let (player_token, mut money, position) = tokens.get_mut(players.list[&key]).expect("Could not find player from key on buy");
            let (_entity, mut tile, _, _) = tiles.get_mut(Entity::from_bits(*position.tile)).expect("Player is not on a space");

            *money.worth -= *tile.cost;
            *tile.owner = Some(player_token.to_bits());
            *tile.tier = Tier::Owned;
        }

        for (key, message) in events.read::<PlayerActionChannel, SellOwnable>() {
            let (_, mut money, _) = tokens.get_mut(players.list[&key]).expect("Could not find player from key on sell");
            let (_, mut tile, _, _) = tiles.get_mut(Entity::from_bits(message.id)).expect("Player tried to sell unavailable space");

            *money.worth += *tile.cost;
            *tile.owner = None;
            *tile.tier = Tier::None;
        }

        // AlterOwnable

        for (key, _message) in events.read::<PlayerActionChannel, EndTurn>() {
            players.next_player();
            event_writer.send(NextTurn(Some(key)));
        }
    }
}

pub struct NextTurn(pub Option<UserKey>);
pub struct AwardPlayer(pub Entity, pub i32);

pub fn next_turn(
    mut players: ResMut<Players>,
    spaces: Res<Tiles>,

    mut event_reader: EventReader<NextTurn>,
    mut event_writer: EventWriter<AwardPlayer>,
    mut tiles: Query<(Entity, &mut Tile, Option<&Corner>, Option<&Chance>), (Without<Money>, Without<Position>)>,
    mut tokens: Query<(Entity, &mut Money, &mut Position), (Without<Tile>, Without<Corner>, Without<Chance>)>,

    mut server: Server,
    mut commands: Commands,
) {
    for NextTurn(last_player) in event_reader.iter() {
        if let Some(key) = last_player {
            let (_, money, _) = tokens.get(players.list[key]).expect("Last player is missing");

            if *money.worth < 0 { // bankruptcy handling
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
        }

        let (token, mut money, mut position) = tokens.get_mut(*players.current_player_entity()).expect("Current player could not be found between turns");

        {
            let mut property = spaces.list.iter().position(|entity| entity.to_bits() == *position.tile).expect("Couldn't find current position of player in Vec form");
            let mut random = rand::thread_rng();

            let roll = random.gen_range(2..=12) as usize;
            property += roll;
            property %= spaces.list.len();

            *position.tile = spaces.list[property].to_bits();
        }

        let (_property, tile, corner, chance) = tiles.get(Entity::from_bits(*position.tile)).expect("Current player is sitting on an unknown tile");
        
        // TEMPORARY COST SPACE CODE
        if *tile.owner != None && *tile.owner != Some(token.to_bits()) {
            event_writer.send(AwardPlayer(Entity::from_bits(tile.owner.unwrap()), *tile.cost / 10));
            *money.worth -= *tile.cost / 10;
        }
        // END TEMPORARY COST SPACE

        let mut action_space: Vec<Action> = vec![];
        // TEMPORARY ACTION SPACE CODE, MONO ACTIONS ONLY
        action_space.push(Action::Sell);
        action_space.push(Action::None);
        if *money.worth >= 0 && *tile.tier == Tier::None && corner.is_none() && chance.is_none() { action_space.push(Action::Purchase); }
        // END TEMPORARY ACTION SPACE

        let new_turn = BeginTurn {
            available_actions: action_space
        };

        server.send_message::<BoardUpdateChannel, BeginTurn>(players.current_player_key(), &new_turn);
    }
}

pub fn reward_player(
    mut event_reader: EventReader<AwardPlayer>,
    mut money_query: Query<&mut Money>,
) {
    for AwardPlayer(entity, amount) in event_reader.iter() {
        *money_query.get_mut(*entity).expect("Attempted to award non-existant entity").worth += amount;
    }
}