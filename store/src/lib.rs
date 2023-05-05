use std::time::Duration;
use naia_bevy_shared::{LinkConditionerConfig, Protocol, Message, ChannelDirection, ChannelMode, ReliableSettings};

pub mod player;
pub mod tile;
pub mod transfer;

pub fn protocol_builder() -> Protocol {
    Protocol::builder()
        .tick_interval(Duration::from_millis(25))
        .link_condition(LinkConditionerConfig::average_condition())

        // Auth Channel
        .add_message::<Auth>()

        // Components
        .add_component::<player::Money>()
        .add_component::<player::Position>()
        .add_component::<player::ServerPlayer>()

        .add_component::<tile::Group>()
        .add_component::<tile::ServerSide>()
        .add_component::<tile::Chance>()
        .add_component::<tile::Corner>()
        .add_component::<tile::Tile>()

        // Channels
        .add_channel::<transfer::PlayerActionChannel>( // These don't **need** to be separate, but bidirectional is weird
            ChannelDirection::ClientToServer,
            ChannelMode::OrderedReliable(ReliableSettings::default()) // doesn't **need** to be tick buffered
        )

        .add_channel::<transfer::BoardUpdateChannel>(
            ChannelDirection::ServerToClient,
            ChannelMode::OrderedReliable(ReliableSettings::default())
        )

        // Messages
        .add_message::<transfer::AlterOwnable>()
        .add_message::<transfer::SellOwnable>()
        .add_message::<transfer::BuyOwnable>()
        .add_message::<transfer::EndTurn>()
        .add_message::<transfer::Forfeit>() // Realistically only available to human players
        .add_message::<transfer::Ready>()
        .add_message::<transfer::Finish>()

        .add_message::<transfer::BeginTurn>()
        .add_message::<transfer::SendPlayer>()
        .add_message::<transfer::StartGame>()
        .add_message::<transfer::IssueReward>()
        .add_message::<transfer::EndGame>() // for resetting gamestate on clients

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
