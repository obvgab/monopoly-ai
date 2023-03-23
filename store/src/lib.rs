use std::time::Duration;

use naia_bevy_shared::{LinkConditionerConfig, Protocol, Message};
use player::Money;
pub mod player;

pub fn protocol_builder() -> Protocol {
    Protocol::builder()
        .tick_interval(Duration::from_millis(25))
        .link_condition(LinkConditionerConfig::average_condition())
        .add_message::<Auth>()
        .add_component::<Money>()
        .build()
}

#[derive(Message)]
pub struct Auth {
    pub name: String,
    pub code: String
}

impl Auth {
    pub fn new(name: &str, code: &str) -> Self {
        Self {
            name: name.to_string(),
            code: code.to_string()
        }
    }
}
