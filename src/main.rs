use bevy::{
    app::AppExit,
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    window::PrimaryWindow,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use flycam::FlycamPlugin;
use voxelengine::VoxelEnginePlugin;

mod flycam;
mod voxelengine;
fn main() {
    App::new()
        // .insert_resource(MovementSettings {
        //     sensitivity: 0.00015,
        //     speed: 12.0,
        // })
        .add_plugins((DefaultPlugins, VoxelEnginePlugin, FlycamPlugin))
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .add_systems(Update, exit_game)
        .run();
}

pub fn setup(
    mut commands: Commands,
    mut primary_window_query: Query<&mut Window, With<PrimaryWindow>>,
    mut ambient_light: ResMut<AmbientLight>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    ambient_light.color = Color::WHITE;
    ambient_light.brightness = 1.0;

    if let Ok(mut window) = primary_window_query.get_single_mut() {
        window.title = "Unnamed Voxel Game".into();
    } else {
        warn!("Primary window not found for `setup`!");
    }
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}
