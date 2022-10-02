use bevy::prelude::*;

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
            .add_startup_system(setup_menu) // 1 and 2
            .add_system(alter_player_count); // 3
    }
}

// ? Is this the best way to impliment the buttons
// ? Possibly make these more universal for future UI
/*
    Defining structs here to allow for queries
    and resource manipulation. Component is the
    C in ECS, and the data is required by Systems
    [S in ECS]. PlayerCount and MonoRegular are
    re-used Resources that we only want one of
*/
#[derive(Component)]
struct DecreasePlayerCount;
#[derive(Component)]
struct IncreasePlayerCount;
struct PlayerCount(i32);
struct MonoRegular(Handle<Font>);

/*
    Create the main menu for selecting players
    (1) Get the font loaded
    (2) Store the amount of players [2-6]
    (3) Spawn an UiNodeBundle [Container]
    (4) Spawn the increase/decrease button
    (5) Spawn the text
    Ui is horizontally aligned from
    JustifyContent::SpaceEvenly. Any re-used data
    like the font or the player count are considered
    resources, and the rest can be queried by the components
    we made above [Increase/Decrease PlayerCount]
*/
fn setup_menu(mut commands: Commands, assets: Res<AssetServer>) {
    let mono = assets.load("fonts/JetBrainsMono-Regular.ttf"); // 1
    commands.insert_resource(MonoRegular(mono.clone()));

    commands.insert_resource(PlayerCount(2)); // 2

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
                    font_size: 60.0,
                    color: Color::hex("cdd6f4").unwrap()
                }),
                ..default()
            });
        }).insert(DecreasePlayerCount);
        node_parent.spawn_bundle(TextBundle { // 5
            text: Text::from_section("0", TextStyle { // TODO: Eventually have the text update (probably through a system) with changes
                font: mono.clone(),
                font_size: 60.0,
                color: Color::hex("cdd6f4").unwrap()
            }),
            ..default()
        });
        node_parent.spawn_bundle(ButtonBundle { // 4
            color: UiColor(Color::hex("1e1e2e").unwrap()),
            ..default() 
        }).with_children(|button_parent| {
            button_parent.spawn_bundle(TextBundle {
                text: Text::from_section("+", TextStyle {
                    font: mono.clone(),
                    font_size: 60.0,
                    color: Color::hex("cdd6f4").unwrap()
                }),
                ..default()
            });
        }).insert(IncreasePlayerCount);
    });
}

fn alter_player_count(
    mut player_count: ResMut<PlayerCount>,
    query: Query<(Option<&DecreasePlayerCount>, Option<&IncreasePlayerCount>, &Interaction), (Changed<Interaction>, Or<(With<DecreasePlayerCount>, With<IncreasePlayerCount>)>)>
) {
    for (decrease, increase, interaction) in query.iter() {
        match interaction {
            Interaction::Clicked => { if decrease.is_some() && player_count.0 != 2 { player_count.0 -= 1; } 
                else if increase.is_some() && player_count.0 != 6 { player_count.0 += 1; } println!("{:?}", player_count.0) }, // TODO: Change text here? Currently just outputing to console
            Interaction::Hovered | Interaction::None => {}
        }
    }
}