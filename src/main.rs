use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Monopoly AI (DEBUG)".into(),
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.8470588235294118, 0.7686274509803922, 0.8470588235294118)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(create_camera)
        .run();
}

fn create_camera(mut commands: Commands) {
    commands
        .spawn_bundle(
            Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 5.0),
            ..default()
            }
        );
}