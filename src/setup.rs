use bevy::{prelude::*, render::{camera::ScalingMode}};

pub struct SetupPlugin;

/*
    Implimenting the Plugin trait for SetupPlugin
    When loaded, SetupPlugin will prepare the window,
    create the camera, load assets, and place static
    game objects
*/
impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(WindowDescriptor {
                title: "Monopoly AI (DEBUG)".into(),
                width: 1300.0,
                height: 1300.0,
                ..default()
            })
            .insert_resource(ClearColor(Color::rgb(0.1171875, 0.1171875, 0.1796875))) // #1e1e2e
            .add_startup_system(SetupPlugin::create_camera)
            .add_startup_system(SetupPlugin::load_sprites);
    }
}

impl SetupPlugin {
    /*
        Create an Orthographic 2d Camera to visualize the scene
        Turn off scaling to prevent uneven sprites
        Placed on Z-layer 10, so we have all of 0-10 to work with
    */
    fn create_camera(mut commands: Commands) {
        let mut camera = Camera2dBundle::new_with_far(10.0);
        camera.projection.scaling_mode = ScalingMode::None;

        commands.spawn_bundle(camera);
    }

    // ! Replace this later with a texture atlas for character sprites
    /*
        Loading all the 2d sprites necessary for the game to run
        Requires two arguments from the bevy engine: Commands and Res<AssetServer>
        Acts as a startup system, as it only runs once
    */
    fn load_sprites(
        mut commands: Commands,
        server: Res<AssetServer>,
    ) {
        // Load and place the game board. We don't need a resource to retain the handle as the board is a static image.
        let board_image = server.load("board.png");

        commands.spawn_bundle(SpriteBundle {
            transform: Transform::default().with_scale(Vec3::splat(0.001535)),
            texture: board_image.into(),
            ..default()
        });
    }
}