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

    commands.spawn(PbrBundle {
        mesh: meshes.add(generate_cube_mesh(Vec3::new(3.0, 3.0, 3.0))),
        material: materials.add(Color::RED.into()),
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(generate_cube_mesh(Vec3::new(1., 1., 1.))),
        material: materials.add(Color::GREEN.into()),
        transform: Transform::from_xyz(0., 8.0, 0.),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(generate_cube_mesh(Vec3::new(0.5, 0.5, 0.5))),
        material: materials.add(Color::BLUE.into()),
        transform: Transform::from_xyz(0., 9.0, 0.),
        ..default()
    });
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn generate_cube_mesh(size: Vec3) -> Mesh {
    let mut cube_mesh = Mesh::new(PrimitiveTopology::TriangleList);

    cube_mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            // Top vertices
            [0., size.y, 0.],
            [0., size.y, size.z],
            [size.x, size.y, size.z],
            [size.x, size.y, 0.],
            // Bottom vertices
            [0., 0., 0.],
            [0., 0., size.z],
            [size.x, 0., size.z],
            [size.x, 0., 0.],
            // Right vertices
            [size.x, 0., 0.],
            [size.x, size.y, 0.],
            [size.x, size.y, size.z],
            [size.x, 0., size.z],
            // Left vertices
            [0., 0., size.z],
            [0., size.y, size.z],
            [0., size.y, 0.],
            [0., 0., 0.],
            // Front vertices
            [size.x, 0., size.z],
            [size.x, size.y, size.z],
            [0., size.y, size.z],
            [0., 0., size.z],
            // Back vertices
            [0., 0., 0.],
            [0., size.y, 0.],
            [size.x, size.y, 0.],
            [size.x, 0., 0.],
        ],
    );

    cube_mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![
            // Top normals
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            // Bottom normals
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            // Right normals
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            // Left normals
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            // Front normals
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            // Back normals
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
        ],
    );

    cube_mesh.set_indices(Some(Indices::U32(vec![
        0, 1, 2, 2, 3, 0, // Top triangles
        4, 7, 6, 6, 5, 4, // Bottom triangles
        8, 9, 10, 10, 11, 8, // Right triangles
        12, 13, 14, 14, 15, 12, // Left triangles
        16, 17, 18, 18, 19, 16, // Front triangles
        20, 21, 22, 22, 23, 20, // Back triangles
    ])));

    cube_mesh
}
