use bevy::{prelude::*, utils::HashMap};
use bracket_noise::prelude::*;
use rand::prelude::*;

pub struct VoxelEnginePlugin;

impl Plugin for VoxelEnginePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(VoxelWorld::default())
            .insert_resource(ChunkSettings::default())
            .add_systems(Startup, generate_world)
            .add_systems(Update, update_chunk_meshes);
    }
}

#[derive(Resource, Default)]
struct VoxelWorld {
    pub chunks: HashMap<IVec3, Entity>,
}

#[derive(Resource)]
struct ChunkSettings {
    height: i32,
    width: i32,
    depth: i32,
}

impl Default for ChunkSettings {
    fn default() -> Self {
        Self {
            height: 1,
            width: 32,
            depth: 32,
        }
    }
}

#[derive(Component, Default)]
struct Chunk {
    voxels: HashMap<IVec3, Entity>,
    position: IVec3,
    mesh: Option<Entity>,
}

#[derive(Component)]
struct NeedsUpdate {}

#[derive(Component)]

struct ChunkMesh {}

#[derive(Component)]
struct Voxel {
    is_solid: bool,
}

// fn setup_voxel_engine(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     let mut noise = FastNoise::seeded(1234);
//     noise.set_noise_type(NoiseType::Simplex);
//     noise.set_frequency(10.0);

//     for x in 0..8 {
//         for z in 0..8 {
//             for y in 0..8 {
//                 let noise_value = noise.get_noise3d(x as f32, y as f32, z as f32);
//                 let is_solid = noise_value > 0.0;
//                 println!(
//                     "New Voxel, X: {}, Y: {}, Z: {}, Noise Value: {}",
//                     x, y, z, noise_value
//                 );
//                 // let is_solid = true;
//                 commands.spawn((
//                     Voxel { is_solid },
//                     PbrBundle {
//                         mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
//                         material: materials.add(if is_solid {
//                             Color::GREEN.into()
//                         } else {
//                             Color::RED.into()
//                         }),
//                         transform: Transform::from_xyz(x as f32, y as f32 + 0.5, z as f32),
//                         ..default()
//                     },
//                 ));
//             }
//         }
//     }
// }

fn generate_chunk_data(commands: &mut Commands, settings: &ChunkSettings) -> Chunk {
    let mut chunk = Chunk::default();

    for x in 0..settings.width {
        for y in 0..settings.height {
            for z in 0..settings.depth {
                let voxel_entity = commands
                    .spawn(Voxel {
                        is_solid: rand::random(),
                    })
                    .id();
                chunk.voxels.insert(IVec3::new(x, y, z), voxel_entity);
            }
        }
    }

    chunk
}

fn generate_world(
    mut commands: Commands,
    mut voxel_world: ResMut<VoxelWorld>,
    chunk_settings: Res<ChunkSettings>,
) {
    for x in 0..1 {
        for y in 0..1 {
            for z in 0..1 {
                let mut chunk = generate_chunk_data(&mut commands, &chunk_settings);

                chunk.position = IVec3::new(x, y, z);

                let chunk_entity = commands.spawn((chunk, NeedsUpdate {})).id();
                voxel_world.chunks.insert(IVec3::new(x, y, z), chunk_entity);
            }
        }
    }

    println!("Generated World!");
}

// fn generate_chunk_mesh(commands: chunk: &Chunk, voxel_query: &Query<&Voxel>) {
//     // let voxel_entities = chunk.voxels;

//     for (position, voxel_entity) in chunk.voxels.iter() {
//         let voxel =
//     }
// }

fn update_chunk_meshes(
    mut commands: Commands,
    chunk_query: Query<(&Chunk, Entity), With<NeedsUpdate>>,
    voxel_query: Query<&Voxel>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (chunk, chunk_entity) in chunk_query.iter() {
        for (position, voxel_entity) in &chunk.voxels {
            let voxel = match voxel_query.get(*voxel_entity) {
                Ok(voxel) => voxel,
                Err(error) => {
                    format!(
                        "Cannot find voxel: {} in chunk: {}! Error: {}",
                        voxel_entity.index(),
                        chunk_entity.index(),
                        error
                    );
                    continue;
                }
            };

            let color = match voxel.is_solid {
                true => Color::GREEN,
                false => Color::RED,
            };

            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                    material: materials.add(color.into()),
                    transform: Transform::from_xyz(
                        position.x as f32 + chunk.position.x as f32 * 32.0,
                        position.y as f32 + chunk.position.y as f32 * 32.0,
                        position.z as f32 + chunk.position.z as f32 * 32.0,
                    ),
                    ..default()
                },
                ChunkMesh {},
            ));
        }
        // generate_chunk_mesh(chunk);
        // for (position, voxel_entity) in chunk.voxels.iter() {
        //     let voxel = match voxel_query.get(*voxel_entity) {
        //         Ok(voxel) => {
        //             println!("Ziihihih");
        //             voxel
        //         }
        //         Err(_) => continue,
        //     };
        //     if voxel.is_solid {
        //         commands.spawn(PbrBundle {
        //             mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        //             material: materials.add(Color::GREEN.into()),
        //             transform: Transform::from_xyz(
        //                 position.x as f32,
        //                 position.y as f32,
        //                 position.z as f32,
        //             ),
        //             ..default()
        //         });
        //     }
        // }
        println!("Updating chunk: {}", chunk_entity.index());

        if let Some(mesh_entity) = chunk.mesh {
            commands.entity(mesh_entity).despawn();
        }
        commands.entity(chunk_entity).remove::<NeedsUpdate>();
    }
}

// fn render_voxels(
//     mut commands: Commands,
//     voxel_query: Query<&Transform, With<Voxel>>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     for transform in voxel_query.iter() {
//         commands.spawn(PbrBundle {
//             mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
//             material: materials.add(Color::GREEN.into()),
//             transform: *transform,
//             ..default()
//         });
//     }
// }
