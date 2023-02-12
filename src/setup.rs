use bevy::{prelude::*, render::{camera::ScalingMode/* , settings::WgpuSettings */}};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor {
            title: "Monopoly AI (DEBUG)".into(),
            width: 650.0,
            height: 650.0,
            ..default()
        })
        //.insert_resource(WgpuSettings {
        //        backends: None,
        //        ..default()
        //    })
        .add_startup_system(create_camera)
        .add_startup_system(load_sprites);
    }
}

fn create_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::new_with_far(10.0);
    camera.projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}

fn load_sprites(mut commands: Commands, server: Res<AssetServer>) {
    // * We should probably make this dynamic later so we can have larger game board sizes
    let board_image = server.load("board.png");

    commands.spawn_bundle(SpriteBundle {
        transform: Transform::default().with_scale(Vec3::splat(0.001535)),
        texture: board_image.into(),
        ..default()
    });
}

