use bevy::prelude::*;
use crate::{GameState, setup::CurrentPlayer, player::{PlayerBundle, Money, TokenPosition, PlayerId, HeldJailFree, IsComputer}};

pub struct MainMenuPlugin;

/*
    Implementing the Plugin trait for MainMenuPlugin
    When loaded, MainMenuPlugin will:
    (1) Craft the UI
    (2) Load common Resources
    (3) Initiate the UI buttons
*/
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::Menu)
                .with_system(setup_menu)) // 1 and 2
            .add_system_set(SystemSet::on_update(GameState::Menu)
                .with_system(alter_player_count)
                .with_system(submit_player_count)); // 3
    }
}

// ? Is this the best way to impliment the buttons
// ? Possibly make these more universal for future UI
/*
    Defining structs here to allow for queries
    and resource manipulation. Component is the
    C in ECS, and the data is required by Systems
    [S in ECS]. MonoRegular is re-used
*/
#[derive(Component)]
struct DecreasePlayerCount;
#[derive(Component)]
struct IncreasePlayerCount;
#[derive(Component)]
struct SubmitPlayerCount;

#[derive(Component)]
struct PlayerCount(i32);
#[derive(Component)]
struct NodePlayerCount;

pub struct MonoRegular(Handle<Font>);

/*
    Create the main menu for selecting players
    (1) Get the font loaded
    (2) Store the amount of players [2-6]
    (3) Spawn an UiNodeBundle [Container]
    (4) Spawn the increase/decrease/start button
    (5) Spawn button text
    Ui is horizontally aligned from
    JustifyContent::SpaceEvenly. Any re-used data
    like the font or the player count are considered
    resources, and the rest can be queried by the components
    we made above [Increase/Decrease PlayerCount]
*/
fn setup_menu(mut commands: Commands, assets: Res<AssetServer>) {
    let mono = assets.load("fonts/JetBrainsMono-Regular.ttf"); // 1
    commands.insert_resource(MonoRegular(mono.clone()));

    commands.spawn_bundle(NodeBundle { // 3
        style: Style {
            align_self: AlignSelf::Center,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceEvenly,
            size: Size::new(Val::Percent(20.0), Val::Percent(10.0)),
            margin: UiRect::all(Val::Auto),
            ..default()
        },
        color: UiColor::from(Color::hex("1e1e2e").unwrap()),
        ..default()
    }).with_children(|node_parent| {
        node_parent.spawn_bundle(ButtonBundle { // 4
            color: UiColor(Color::hex("1e1e2e").unwrap()),
            ..default() 
        }).with_children(|button_parent| {
            button_parent.spawn_bundle(TextBundle {
                text: Text::from_section("-", TextStyle {
                    font: mono.clone(),
                    font_size: 40.0,
                    color: Color::hex("cdd6f4").unwrap()
                }),
                ..default()
            });
        }).insert(DecreasePlayerCount);
        node_parent.spawn_bundle(ButtonBundle { // 4
            color: UiColor(Color::hex("1e1e2e").unwrap()),
            ..default()
        }).with_children(|button_parent| {
            button_parent.spawn_bundle(TextBundle { // 5
                text: Text::from_section("2", TextStyle {
                    font: mono.clone(),
                    font_size: 40.0,
                    color: Color::hex("cdd6f4").unwrap()
                }),
                ..default()
            }).insert(PlayerCount(2)); // 2
        }).insert(SubmitPlayerCount);
        node_parent.spawn_bundle(ButtonBundle { // 4
            color: UiColor(Color::hex("1e1e2e").unwrap()),
            ..default() 
        }).with_children(|button_parent| {
            button_parent.spawn_bundle(TextBundle {
                text: Text::from_section("+", TextStyle {
                    font: mono.clone(),
                    font_size: 40.0,
                    color: Color::hex("cdd6f4").unwrap()
                }),
                ..default()
            });
        }).insert(IncreasePlayerCount);
    }).insert(NodePlayerCount);
}

/*
    Query for both our increase/decrease player
    count buttons and gather the player count data
    Alter the text shown and the stored value
*/
fn alter_player_count(
    mut player_count_text: Query<&mut Text, With<PlayerCount>>,
    mut player_count_value: Query<&mut PlayerCount>,
    query: Query<(AnyOf<(&DecreasePlayerCount, &IncreasePlayerCount)>, &Interaction), Changed<Interaction>>
) {
    let mut text_ref = player_count_text.get_single_mut().unwrap();
    let mut player_count = player_count_value.get_single_mut().unwrap();
    for (change, interaction) in query.iter() {
        match interaction {
            Interaction::Clicked => {
                // * We should allow for more than 2-6 for larger game boards
                if change.0.is_some() && player_count.0 != 2 { player_count.0 -= 1; } 
                else if change.1.is_some() && player_count.0 != 6 { player_count.0 += 1; }
                text_ref.sections[0].value = format!("{}", player_count.0); // * This solution is probably over-engineered
            },
            Interaction::Hovered | Interaction::None => {}
        }
    }
}

/*
    Submit the player count selected
    Changing the game state to Rolling
    a.k.a the start of the game
    Hide the player count UI
    Spawn all the player entities
*/
fn submit_player_count(
    mut commands: Commands,
    player_count_value: Query<&mut PlayerCount>,
    query: Query<&Interaction, (Changed<Interaction>, With<SubmitPlayerCount>)>,
    mut node_visibility: Query<&mut Visibility, With<NodePlayerCount>>,
    mut state: ResMut<State<GameState>>
) {
    let player_count = player_count_value.get_single().unwrap();
    let mut visibility = node_visibility.get_single_mut().unwrap();
    for interaction in query.iter() {
        match interaction {
            Interaction::Clicked => {
                visibility.is_visible = false;
                commands.insert_resource(CurrentPlayer(0, player_count.0));
                for i in 0..player_count.0 {
                    commands.spawn().insert_bundle(
                        PlayerBundle {
                            money: Money(1500),
                            token_position: TokenPosition(0, 0),
                            player_id: PlayerId(i),
                            held_jail_free: HeldJailFree(0),
                            is_computer: IsComputer(false)
                        }
                    ).insert(Name::new(format!("Player {}", i)));
                }
                state.set(GameState::Rolling).unwrap();
            },
            Interaction::Hovered | Interaction::None => {}
        }
    }
}