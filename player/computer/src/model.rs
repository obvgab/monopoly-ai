use bevy::prelude::*;
use dfdx::{optim::{Adam, AdamConfig}, prelude::{SplitInto, modules::Linear, ReLU, DeviceBuildExt, ZeroGrads, Module}, tensor::{Cpu, Gradients, TensorFrom}};
use monai_store::{transfer::{BeginTurn, BoardUpdateChannel, PlayerActionChannel, SendPlayer, EndTurn, BuyOwnable, SellOwnable}, tile::{Tile, Corner, Chance, ServerSide}, player::{Money, Position, ServerPlayer, Action}};
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
    pub epsilon: f32,
    pub experience: Vec<Transition>
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
        epsilon: 1f32,
        experience: vec![]
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
            let mut action_selection_mask = [0.0; SQUARES]; // maybe make this 0/1
            for (_, tile, _, _, server_side) in &tiles {
                if *tile.owner == Some(stateful.entity) {
                    squares[*server_side.index] = 1;
                    action_selection_mask[*server_side.index] = 1.0;
                } else if tile.owner.is_some() {
                    squares[*server_side.index] = 2; // We don't own and can't buy the property
                }
            }
            let action_selection_mask = stateful.device.tensor(action_selection_mask.map(|v: f32| v.log10()));

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

                    for x in &tiles { // breaks when first player, message is received before tiles spawn?
                        if *x.4.id == *position.tile {
                            location = Some(*x.4.index);
                        }
                    }

                    location.expect("Could not find player position") as i32
                }
            }

            let mut action_type_mask = [0.0; ACTION];
            for action in turn.available_actions {
                match action {
                    Action::Purchase => action_type_mask[0] = 1.0,
                    Action::Sell => action_type_mask[1] = 1.0,
                    Action::None => action_type_mask[2] = 1.0
                }
            }
            let action_type_mask = stateful.device.tensor(action_type_mask.map(|v: f32| v.log10()));

            let state: [f32; STATE] = 
                squares.iter().chain(players.iter())
                    .map(|v| *v as f32).collect::<Vec<f32>>()
                    .try_into().expect("Couldn't convert state");
            let state_tensor = stateful.device.tensor(state);

            // make tensors
            let (action_type, action_selection) = 
                stateful.model.forward(state_tensor);
            let action = 
                ((action_type + action_type_mask).softmax().as_vec().iter().enumerate()
                    .max_by(|(_, a), (_, b)| a.total_cmp(b)).map(|(i, _)| i).expect("Head empty"),
                (action_selection + action_selection_mask).softmax().as_vec().iter().enumerate()
                    .max_by(|(_, a), (_, b)| a.total_cmp(b)).map(|(i, _)| i).expect("Head empty"));

            match action.0 {
                0 => {
                    client.send_message::<PlayerActionChannel, BuyOwnable>(&BuyOwnable);
                    client.send_message::<PlayerActionChannel, EndTurn>(&EndTurn);

                    info!("Bought property");
                }
                1 => {
                    let (_, _, _, _, server_side) = 
                        tiles.iter().find(|x| *x.4.index == action.1)
                        .expect("Selling property not found");

                    client.send_message::<PlayerActionChannel, SellOwnable>(&SellOwnable { id: *server_side.id });
                    client.send_message::<PlayerActionChannel, EndTurn>(&EndTurn);

                    info!("Sold property");
                }
                2 => {
                    client.send_message::<PlayerActionChannel, EndTurn>(&EndTurn);

                    info!("Did not act");
                }
                _ => { error!("Invalid decision"); }
            }

            stateful.experience.push((state, 0.0, action, None)); // Default case for the experience. When the server responds we will change .1 and .3, if necessary
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