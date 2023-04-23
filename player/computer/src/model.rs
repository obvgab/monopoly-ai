use bevy::prelude::*;
use dfdx::{optim::{Adam, AdamConfig}, prelude::{SplitInto, modules::Linear, ReLU, DeviceBuildExt, ZeroGrads}, tensor::{Cpu, Gradients}};

const PLAYERS: usize = 4;
const SQUARES: usize = 40;

const STATE: usize = SQUARES + (PLAYERS * 2); // Whether the user owns a square + each player's worth and position
const ACTION: usize = 3;

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
    pub entity: u64,
    pub target: QModule,
    pub model: QModule,
    pub gradients: Gradients<f32, Device>,
    pub optimizer: Adam<QModule, f32, Device>
}

pub fn add_stateful(world: &mut World) { // &mut World makes exclusive, first startup system. Stateful should always exist
    let dev = Device::default();
    let model = dev.build_module::<QModel, f32>();
    let grads = model.alloc_grads();
    let optim = Adam::new(&model, AdamConfig::default());

    world.insert_non_send_resource(StatefulInformation {
        entity: 0,
        target: model.clone(), // target as first argument to avoid borrow checker issues
        model: model,
        gradients: grads,
        optimizer: optim,
    });
}