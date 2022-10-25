use bevy::{prelude::*, render::camera::ScalingMode};

pub struct SetupPlugin;

/*
    Implimenting the Plugin trait for SetupPlugin
    When loaded, SetupPlugin will
    (1) Prepare the window
    (2) Create the camera
    (3) Load assets
    (4) Place static objects
*/
impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor {
            title: "Monopoly AI (DEBUG)".into(),
            width: 650.0,
            height: 650.0,
            ..default()
        }) // 1
        .add_startup_system(create_camera) // 2
        .add_startup_system(load_sprites); // 3 and 4
    }
}

/*
    Resources to track game values and settings
    (1) Universal player data
        .0 Current player index
        .1 Total players in the game
    (2) The settings of the game chosen with the menu
        .0 Visual Mode (Faster when off, for training the AI)
        .1 Debt (Ability to go into debt. (!) Being at 0 cash means backrupt [Correlate .2])
        .2 Sell (Removes the ability to sell houses/hotels. (!) Being at 0 cash means backrupt [Correlate .1])
        .3 Homes (Build structures. (!) Less complex game [Implies (!) .2])
        .4 Chance (Chance Tile Actions. (!) Less complex game)
        .5 Chest (Community Chest Actions/Cards. (!) Less complex game)
        .6 Tax (Taxing on non-owned tiles. (!) Less complex game)
        .7 Jail (Going to jail, out of jail free cards, and triple doubles. (!) Less complex game)
        .8 Auction (Auction a property when a player decides not to buy. (!) Less complex game)
*/
pub struct CurrentPlayer(pub i32, pub i32); // 1
impl Default for CurrentPlayer { fn default() -> Self { CurrentPlayer { 0: 0, 1: 2 } } }
#[derive(Default)]
pub struct GameSettings( // 2
    pub bool, // .0
    pub bool, // .1
    pub bool, // .2
    pub bool, // .3
    pub bool, // .4
    pub bool, // .5
    pub bool, // .6
    pub bool, // .7
    pub bool, // .8
);

/*
    Create an Orthographic 2d Camera to visualize the scene
    (1) Turn off scaling to prevent uneven sprites
    (2) Placed on Z-layer 10, so we have all of 0-10 to work with
*/
fn create_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::new_with_far(10.0); // 2
    camera.projection.scaling_mode = ScalingMode::None; // 1

    commands.spawn_bundle(camera); // 3
}

// ! Replace this later with a texture atlas for character sprites
/*
    Loading all the 2d sprites necessary for the game to run
    Requires two arguments from the bevy engine: Commands and Res<AssetServer>
    After running at startup, Handle<>s to the common assets will
    be stored in Resources.
*/
fn load_sprites(mut commands: Commands, server: Res<AssetServer>) {
    // * We should probably make this dynamic later so we can have larger game board sizes
    let board_image = server.load("board.png");

    commands.spawn_bundle(SpriteBundle {
        transform: Transform::default().with_scale(Vec3::splat(0.001535)),
        texture: board_image.into(),
        ..default()
    });
}

