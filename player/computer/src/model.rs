use bevy::prelude::*;
use dfdx::{optim::{Adam, AdamConfig}, prelude::{SplitInto, modules::Linear, ReLU, DeviceBuildExt, ZeroGrads, Module}, tensor::{Cpu, Gradients, TensorFromVec, TensorFrom}, tensor_ops::SelectTo};
use monai_store::{transfer::{BeginTurn, BoardUpdateChannel, PlayerActionChannel, SendPlayer}, tile::{Tile, Corner, Chance, ServerSide}, player::{Money, Position, ServerPlayer, Action}};
use naia_bevy_client::{events::MessageEvents, Client};

const PLAYERS: usize = 4;
const SQUARES: usize = 40;

const STATE: usize = SQUARES + (PLAYERS * 2); // Whether the user owns a square + each player's worth and position
const ACTION: usize = 3;

const BATCH: usize = 32; // number of turns before learning
const DISCOUNT: f32 = 0.9;
const DECAY: f32 = 0.0002;

type Device = Cpu;
// type Device = Cuda;

type QModel = SplitInto<(
    ( // action type head
        (dfdx::prelude::Linear<STATE, 96>, ReLU),
        dfdx::prelude::Linear<96, ACTION>,
    ),
    ( // property head
        (dfdx::prelude::Linear<STATE, 96>, ReLU),
        dfdx::prelude::Linear<96, SQUARES>,
    ),
)>;

type QModule = SplitInto<( // copy of QModel to match output of build_module
    ( // prelude::Linear becomes modules::Linear, we need more Generics
        (Linear<STATE, 96, f32, Device>, ReLU),
        Linear<96, ACTION, f32, Device>,
    ),
    (
        (Linear<STATE, 96, f32, Device>, ReLU),
        Linear<96, SQUARES, f32, Device>,
    ),
)>;

pub struct StatefulInformation {
    pub device: Device,
    pub entity: u64,
    pub target: QModule,
    pub model: QModule,
    pub gradients: Gradients<f32, Device>,
    pub optimizer: Adam<QModule, f32, Device>,
    pub epsilon: f32
}

type Transition = (
    [f32; STATE], // our transition state
    f32, // reward (might need to be a tensor)
    (usize, usize), // converted actions ([action index], [sold square]) (might need to change to tensor?)
    Option<[f32; STATE]> // next state (if None, means END)
);

pub fn add_stateful(world: &mut World) { // &mut World makes exclusive, first startup system. Stateful should always exist
    let dev = Device::default();
    let model = dev.build_module::<QModel, f32>();
    let grads = model.alloc_grads();
    let optim = Adam::new(&model, AdamConfig::default());

    world.insert_non_send_resource(StatefulInformation {
        device: dev,
        entity: 0,
        target: model.clone(), // target as first argument to avoid borrow checker issues
        model: model,
        gradients: grads,
        optimizer: optim,
        epsilon: 1f32
    });
}

pub fn message_event( // action picker
    mut stateful: NonSendMut<StatefulInformation>,

    tiles: Query<(Entity, &mut Tile, Option<&Corner>, Option<&Chance>, &ServerSide), (Without<Money>, Without<Position>)>,
    tokens: Query<(Entity, &mut Money, &Position, &ServerPlayer), (Without<Tile>, Without<Corner>, Without<Chance>)>,

    mut event_reader: EventReader<MessageEvents>,
    mut client: Client
) {
    for events in event_reader.iter() {
        for turn in events.read::<BoardUpdateChannel, BeginTurn>() {
            // First see if we are exploring vs exploiting
            // Query state and create action masks
            let mut squares = [0; SQUARES];
            let mut action_selection_mask = [false; SQUARES]; // maybe make this 0/1
            for (_, tile, _, _, server_side) in &tiles {
                if *tile.owner == Some(stateful.entity) {
                    squares[*server_side.index] = 1;
                    action_selection_mask[*server_side.index] = true;
                } else if tile.owner.is_some() {
                    squares[*server_side.index] = 2; // We don't own and can't buy the property
                }
            }
            let action_selection_mask = stateful.device.tensor(action_selection_mask.map(|v| v as i32 as f32));

            let mut players = [0; PLAYERS * 2];
            for (_, money, position, server_side) in &tokens {
                players[*server_side.index * 2] = {
                    let mut net_worth = *money.worth;
                    tiles.for_each(|x| {
                        if *x.1.owner == Some(stateful.entity) {
                            net_worth += (1.5 * *x.1.cost as f32).ceil() as i32;
                        }
                    });

                    net_worth
                };

                players[*server_side.index * 2 + 1] = {
                    let mut location = None;

                    for x in &tiles {
                        if *x.4.id == *position.tile {
                            location = Some(*x.4.index);
                        }
                    }

                    location.expect("Could not find player position") as i32
                }
            }

            let mut action_type_mask = [false; ACTION];
            for action in turn.available_actions {
                match action {
                    Action::Purchase => action_type_mask[0] = true,
                    Action::Sell => action_type_mask[1] = true,
                    Action::None => action_type_mask[2] = true
                }
            }
            let action_type_mask = stateful.device.tensor(action_type_mask.map(|v| v as i32 as f32));

            let state: [f32; STATE] = 
                squares.iter().chain(players.iter())
                    .map(|v| *v as f32).collect::<Vec<f32>>()
                    .try_into().expect("Couldn't convert state");
            let state = stateful.device.tensor(state);

            // make tensors
            let (action_type, action_selection) = 
                stateful.model.forward(state);
            // let (action_type_idx, action_selection_idx) = 
            //     (stateful.device.tensor(0usize), stateful.device.tensor(0usize));

            // (action_type * action_type_mask).softmax().select(action_type_idx);
            // (action_selection * action_selection_mask).softmax().select(action_selection_idx);

            // let action_type_idx = (action_type * action_type_mask).softmax().max;
            // let action_selection_idx = (action_selection * action_selection_mask).softmax();

            dbg!(action_type.softmax());
            dbg!(action_selection.softmax());

            // merge models after a bit
        }

        for entity in events.read::<BoardUpdateChannel, SendPlayer>() {
            stateful.entity = entity.id;
        }
    }
}

pub fn _train() {}

// ! We likely need TD learning (temporal difference)
// !    the working idea is:
// * BeginTurn will make the agent take an action and start saving it
// *    will need to eventually add a message for the server to give reward
// *    after the reward is giving, next_state can either be right after or the next BeginTurn
// *    this can be saved into a Vec<Transition> for our experiences
// ? https://github.com/SilasMarvin/bevy-dfdx-and-the-classic-cart-pole/blob/main/src/main.rs#L37
// ? https://github.com/coreylowman/dfdx/blob/main/examples/11-multi-headed.rs
// ? https://github.com/coreylowman/dfdx/blob/main/examples/rl-dqn.rs
// https://towardsdatascience.com/rainbow-dqn-the-best-reinforcement-learning-has-to-offer-166cb8ed2f86
// PASS GO!!
// Focus on MAXIMIZING rewards
// RESTART GAME, CALCULATE BANKRUPTCY