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
        app
            .insert_resource(WindowDescriptor {
                title: "Monopoly AI (DEBUG)".into(),
                width: 650.0,
                height: 650.0,
                ..default()
            }) // 1
            .add_startup_system(create_camera) // 2
            .add_startup_system(load_sprites); // 3 and 4
    }
}

// * Make these full structs with names, .0 and .1 are not descriptive/readable
pub struct CurrentPlayer(pub i32, pub i32);
pub struct VisualMode(pub bool); // Eventually for disabling visual effects for training the AI

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
fn load_sprites(
    mut commands: Commands,
    server: Res<AssetServer>,
) {
    // * We should probably make this dynamic later so we can have larger game board sizes
    let board_image = server.load("board.png");

    commands.spawn_bundle(SpriteBundle {
        transform: Transform::default().with_scale(Vec3::splat(0.001535)),
        texture: board_image.into(),
        ..default()
    });
}