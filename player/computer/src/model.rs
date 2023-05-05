use bevy::prelude::*;
use dfdx::{optim::{Adam, AdamConfig}, prelude::{SplitInto, modules::Linear, ReLU, DeviceBuildExt, ZeroGrads, Module, huber_loss, Optimizer, SaveToNpz, LoadFromNpz}, tensor::{Cpu, TensorFrom, Trace}, tensor_ops::{SelectTo, Backward}};
use monai_store::{transfer::{BeginTurn, BoardUpdateChannel, PlayerActionChannel, SendPlayer, EndTurn, BuyOwnable, SellOwnable, IssueReward, EndGame}, tile::{Tile, Corner, Chance, ServerSide}, player::{Money, Position, ServerPlayer, Action}};
use naia_bevy_client::{events::MessageEvents, Client};
use rand::{prelude::Distribution, seq::SliceRandom};
use crate::{SQUARES, GameState, ClientResources};

const PLAYERS: usize = 4;

const STATE: usize = SQUARES + (PLAYERS * 2); // Whether the user owns a square + each player's worth and position
const ACTION: usize = 3;

const BATCH: usize = 32; // number of turns before learning, 30 is the average for a game
const DISCOUNT: f32 = 0.9;
const DECAY: f32 = 0.005;

type Device = Cpu;
// type Device = Cuda;

type QModel = SplitInto<(
    ( // action type head
        (dfdx::prelude::Linear<STATE, 192>, ReLU),
        (dfdx::prelude::Linear<192, 96>, ReLU),
        dfdx::prelude::Linear<96, ACTION>,
    ),
    ( // property head
        (dfdx::prelude::Linear<STATE, 192>, ReLU),
        (dfdx::prelude::Linear<192, 96>, ReLU),
        dfdx::prelude::Linear<96, SQUARES>,
    ),
)>;

type QModule = SplitInto<( // copy of QModel to match output of build_module
    ( // prelude::Linear becomes modules::Linear, we need more Generics
        (Linear<STATE, 192, f32, Device>, ReLU),
        (Linear<192, 96, f32, Device>, ReLU),
        Linear<96, ACTION, f32, Device>,
    ),
    (
        (Linear<STATE, 192, f32, Device>, ReLU),
        (Linear<192, 96, f32, Device>, ReLU),
        Linear<96, SQUARES, f32, Device>,
    ),
)>;

pub struct StatefulInformation {
    pub device: Device,
    pub entity: u64,
    pub target: QModule,
    pub model: QModule,
    pub optimizer: Adam<QModule, f32, Device>,
    pub epsilon: f32,
    pub experience: Vec<Transition>,
    pub steps: i32
}

type Transition = (
    [f32; STATE], // our transition state
    f32, // reward (might need to be a tensor)
    (usize, usize), // converted actions ([action index], [sold square]) (might need to change to tensor?)
    Option<[f32; STATE]> // next state (if None, means END)
);

pub fn add_stateful(
    world: &mut World
) { // &mut World makes exclusive, first startup system. Stateful should always exist
    let dev = Device::default();
    let mut model = dev.build_module::<QModel, f32>();
    if let Some(info) = world.get_resource::<ClientResources>() {
        if let Some(model_path) = info.model_path.clone() {
            model.load(model_path).expect("Could not load model from .npz");
            println!("Loaded model properly");
        }
    }
    let optim = Adam::new(&model, AdamConfig::default());

    world.insert_non_send_resource(StatefulInformation {
        device: dev,
        entity: 0,
        target: model.clone(), // target as first argument to avoid borrow checker issues
        model: model,
        optimizer: optim, // We remove gradients since it annihilates the borrow checker
        epsilon: 0.3f32,
        experience: vec![],
        steps: 0
    });
}

pub fn message_event( // action picker
    mut stateful: NonSendMut<StatefulInformation>,
    info: Res<ClientResources>,

    tiles: Query<(Entity, &mut Tile, Option<&Corner>, Option<&Chance>, &ServerSide), (Without<Money>, Without<Position>)>,
    tokens: Query<(Entity, &mut Money, &Position, &ServerPlayer), (Without<Tile>, Without<Corner>, Without<Chance>)>,

    mut event_reader: EventReader<MessageEvents>,
    mut game_state: ResMut<NextState<GameState>>,
    mut client: Client
) {
    for events in event_reader.iter() {
        for turn in events.read::<BoardUpdateChannel, BeginTurn>() {
            // First see if we are exploring vs exploiting
            let state = get_state(&tiles, &tokens, stateful.entity);
            let action: (usize, usize);
            if stateful.epsilon > rand::random::<f32>() { // explore!
                println!("Exploring, epsilon {}", stateful.epsilon);
                let available = turn.available_actions.iter().map(|x| {
                    match x {
                        Action::Purchase => 0,
                        Action::Sell => 1,
                        Action::None => 2
                    }
                }).collect::<Vec<usize>>();

                let squares = tiles.iter()
                    .filter(|(_, x, _, _, _)| *x.owner == Some(stateful.entity))
                    .map(|(_, _, _, _, x)| *x.index).collect::<Vec<usize>>();

                println!("Available actions: {:?}\nAvailable squares: {:?}", available, squares);
                action = 
                    (*available.choose(&mut rand::thread_rng()).expect("Couldn't choose explore option"),
                    *squares.choose(&mut rand::thread_rng()).unwrap_or(&0));
            } else { // exploit.
                println!("Exploiting, epsilon {}", stateful.epsilon);
                // Query state and create action masks
                let mut action_selection_mask = [0.0; SQUARES];
                for (_, tile, _, _, server_side) in &tiles {
                    if *tile.owner == Some(stateful.entity) {
                        action_selection_mask[*server_side.index] = 1.0;
                    }
                }
                println!("Squares mask: {:?}", action_selection_mask);
                let action_selection_mask = 
                    stateful.device.tensor(action_selection_mask.map(|v: f32| v.log10()));

                let mut action_type_mask = [0.0; ACTION];
                for action in turn.available_actions {
                    match action {
                        Action::Purchase => action_type_mask[0] = 1.0,
                        Action::Sell => action_type_mask[1] = 1.0,
                        Action::None => action_type_mask[2] = 1.0
                    }
                }
                println!("Actions mask: {:?}", action_type_mask);
                let action_type_mask = 
                    stateful.device.tensor(action_type_mask.map(|v: f32| v.log10()));

                let state_tensor = stateful.device.tensor(state);
                let (action_type, action_selection) = 
                    stateful.model.forward(state_tensor);
                action = 
                    ((action_type + action_type_mask).softmax().as_vec().iter().enumerate()
                        .max_by(|(_, a), (_, b)| a.total_cmp(b)).map(|(i, _)| i).expect("Head empty"),
                    (action_selection + action_selection_mask).softmax().as_vec().iter().enumerate()
                        .max_by(|(_, a), (_, b)| a.total_cmp(b)).map(|(i, _)| i).expect("Head empty"));
            }
            stateful.epsilon = (stateful.epsilon - DECAY).max(0.05);

            match action.0 {
                0 => {
                    client.send_message::<PlayerActionChannel, BuyOwnable>(&BuyOwnable);
                    client.send_message::<PlayerActionChannel, EndTurn>(&EndTurn);

                    println!("Bought property");
                }
                1 => {
                    let (_, _, _, _, server_side) = 
                        tiles.iter().find(|x| *x.4.index == action.1)
                        .expect("Selling property not found");

                    client.send_message::<PlayerActionChannel, SellOwnable>(&SellOwnable { id: *server_side.id });
                    client.send_message::<PlayerActionChannel, EndTurn>(&EndTurn);

                    println!("Sold property");
                }
                2 => {
                    client.send_message::<PlayerActionChannel, EndTurn>(&EndTurn);

                    println!("Did not act");
                }
                _ => { println!("Invalid decision"); }
            }

            stateful.experience.push((state, 0.0, action, None)); // Default case for the experience. When the server responds we will change .1 and .3, if necessary
        }

        for issued in events.read::<BoardUpdateChannel, IssueReward>() {
            let entity = stateful.entity;
            if let Some(transition) = stateful.experience.last_mut() {
                println!("Received reward {}", issued.reward);
                transition.1 = issued.reward; // +=
                transition.3 = Some(get_state(&tiles, &tokens, entity)); // only on next turn?
            }

            if stateful.experience.len() > BATCH {
                println!("Training model");
                stateful.train();
            }

            if stateful.steps > 10 { // arbitrary number for episodes between merges
                println!("Syncing target model");
                stateful.target = stateful.model.clone();
                stateful.steps = 0;
            } else {
                stateful.steps += 1;
            }
        }

        for _ in events.read::<BoardUpdateChannel, EndGame>() {
            stateful.steps = 0;
            stateful.target = stateful.model.clone();
            println!("Saving model");
            stateful.model.save(format!("models/{}.npz", info.name)).expect("Couldn't save model to .npz");
            stateful.entity = 0;
            game_state.set(GameState::Despawning);
        }
    }
}

pub fn get_state(
    tiles: &Query<(Entity, &mut Tile, Option<&Corner>, Option<&Chance>, &ServerSide), (Without<Money>, Without<Position>)>,
    tokens: &Query<(Entity, &mut Money, &Position, &ServerPlayer), (Without<Tile>, Without<Corner>, Without<Chance>)>,

    owner: u64
) -> [f32; SQUARES + (PLAYERS * 2)] { // handle end of the game for just one player
    let mut players = [0; PLAYERS * 2];
    for (_, money, position, server_side) in tokens {
        players[*server_side.index * 2] = {
            let mut net_worth = *money.worth;
            tiles.for_each(|x| {
                if *x.1.owner == Some(owner) {
                    net_worth += (1.5 * *x.1.cost as f32).ceil() as i32;
                }
            });

            net_worth
        };

        players[*server_side.index * 2 + 1] = {
            let mut location = None;

            for x in tiles { // breaks when first player, message is received before tiles spawn?
                if *x.4.id == *position.tile {
                    location = Some(*x.4.index);
                }
            }

            location.expect("Could not find player position") as i32
        }
    }
    // swap player index with our player so it can keep track between sessions
    for (_, _, _, server_side) in tokens {
        if *server_side.id == owner {
            let current_initial = (players[0], players[1]);
            players[0] = players[*server_side.index * 2];
            players[1] = players[*server_side.index * 2 + 1];

            players[*server_side.index * 2] = current_initial.0;
            players[*server_side.index * 2 + 1] = current_initial.1;
        }
    }

    let mut squares = [0; SQUARES];
    for (_, tile, _, _, server_side) in tiles {
        if *tile.owner == Some(owner) {
            squares[*server_side.index] = 1;
        } else if tile.owner.is_some() {
            squares[*server_side.index] = 2; // We don't own and can't buy the property
        }
    }

    let state: [f32; STATE] = 
    squares.iter().chain(players.iter())
        .map(|v| *v as f32).collect::<Vec<f32>>()
        .try_into().expect("Couldn't convert state");

    return state;
}

impl StatefulInformation {
    pub fn train(&mut self) {
        let mut rng = rand::thread_rng();
        let uniform = rand::distributions::Uniform::from(0..self.experience.len());
        let sample: Vec<Transition> = (0..BATCH).map(|_| self.experience[uniform.sample(&mut rng)]).collect();

        let previous: [[f32; STATE]; BATCH] = sample.iter().map(|x| x.0)
            .collect::<Vec<[f32; STATE]>>().try_into().expect("Couldn't map experiences");
        let previous = self.device.tensor(previous);

        let gradients = self.model.alloc_grads();
        let predictions = self.model.forward(previous.trace(gradients));

        let (action_type, action_selection): ([usize; BATCH], [usize; BATCH]) = 
            (sample.iter().map(|x| x.2.0)
                .collect::<Vec<usize>>().try_into().expect("Couldn't map action types"),
            sample.iter().map(|x| x.2.1)
                .collect::<Vec<usize>>().try_into().expect("Couldn't map action selections"));
        let (action_type, action_selection) = 
            (self.device.tensor(action_type), self.device.tensor(action_selection));

        let predictions = 
            (predictions.0.select(action_type), predictions.1.select(action_selection));

        // target network time
        let mut target_predictions: ([f32; BATCH], [f32; BATCH]) = ([0.0; BATCH], [0.0; BATCH]);
        for (index, experience) in sample.iter().enumerate() {
            match experience.3 {
                Some(next_state) => {
                    let (target_type, target_selection) = 
                        self.model.forward(self.device.tensor(next_state));
                        
                    let action = 
                        (target_type.as_vec().into_iter().max_by(|a, b| a.total_cmp(b)).expect("Could not find max quality"),
                        target_selection.as_vec().into_iter().max_by(|a, b| a.total_cmp(b)).expect("Could not find max quality"));

                    // hey! look! the bellman equation! kind of...
                    // reward + (maxNextQ * discount), for both heads
                    target_predictions.0[index] = action.0 * DISCOUNT + experience.1;
                    target_predictions.1[index] = action.1 * DISCOUNT + experience.1;
                }
                None => { // terminal state, reward is the same
                    target_predictions.0[index] = experience.1;
                    target_predictions.1[index] = experience.1;
                }
            }
        }
        let target_predictions = 
            (self.device.tensor(target_predictions.0), self.device.tensor(target_predictions.1));

        let losses = 
            (huber_loss(predictions.0, target_predictions.0, 1.0), // test different deltas
            huber_loss(predictions.1, target_predictions.1, 1.0));
        let loss = (losses.0 + losses.1).backward(); // this may become an issue?

        self.optimizer.update(&mut self.model, &loss).expect("Updating failed");
    }
}

pub fn read_entity(
    mut stateful: NonSendMut<StatefulInformation>,

    mut event_reader: EventReader<MessageEvents>,
) {
    for events in event_reader.iter() {
        for entity in events.read::<BoardUpdateChannel, SendPlayer>() {
            println!("Entity assigned");
            stateful.entity = entity.id;
        }
    }
}

// ? https://towardsdatascience.com/rainbow-dqn-the-best-reinforcement-learning-has-to-offer-166cb8ed2f86