use bevy::prelude::*;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(WindowDescriptor {
                title: "Monopoly AI (DEBUG)".into(),
                width: 1440.0,
                height: 1440.0,
                ..default()
            })
            .insert_resource(ClearColor(Color::rgb(0.8470588235294118, 0.7686274509803922, 0.8470588235294118)))
            .add_startup_system(SetupPlugin::create_camera)
            .add_startup_system(SetupPlugin::load_character_sprites);
    }
}

impl SetupPlugin {
    fn create_camera(mut commands: Commands) {
        commands
            .spawn_bundle(
                Camera2dBundle {
                transform: Transform::from_xyz(0.0, 0.0, 5.0),
                ..default()
                }
            );
    }

    fn load_character_sprites(
        mut commands: Commands,
        server: Res<AssetServer>
    ) {
        commands
            .spawn_bundle(SpriteBundle {
                //texture: server.load("test.png"),
                transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3 { x: 0.5, y: 0.5, z: 0.0 }),
                ..default()
            });
    }
}