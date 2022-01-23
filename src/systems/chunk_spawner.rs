use bevy::input::Input;
use bevy::math::IVec3;
use bevy::prelude::{Assets, Commands, Entity, KeyCode, Mesh, PbrBundle, Query, Res, ResMut, Transform, With};
use bevy::tasks::{AsyncComputeTaskPool, Task};
use bevy_fly_camera::FlyCamera;
use futures_lite::future;
use crate::{Chunk, ChunkManager, generate_mesh, StandardMaterial, Vec3};
use crate::chunk_manager::{get_chunk_containing_position, SpawnedChunk};

pub fn chunk_despawner() {

}

pub fn chunk_spawner(
    mut commands: Commands,
    camera_query: Query<&Transform, With<FlyCamera>>,
    mut chunk_manager_query: Query<&mut ChunkManager>,
    input: Res<Input<KeyCode>>,
    thread_pool: Res<AsyncComputeTaskPool>,
) {
    let camera_transform = camera_query.single();
    let mut chunk_manager = chunk_manager_query.single_mut();

    let player_chunk = get_chunk_containing_position(&camera_transform.translation, &chunk_manager.chunk_size);
    println!("Player is in Chunk region {} {} {} with position {} {} {}",
             player_chunk.x, player_chunk.y, player_chunk.z,
             camera_transform.translation.x, camera_transform.translation.y, camera_transform.translation.z);

    for chunk_to_despawn in chunk_manager.chunks_to_despawn(player_chunk) {
        let task = thread_pool.spawn(async move {
            DespawnChunkTask {
                chunk: IVec3::new(chunk_to_despawn.x, chunk_to_despawn.y, chunk_to_despawn.z)
            }
        });
        commands.spawn().insert(task);
    }

    let chunks_to_spawn = chunk_manager.request_chunks_to_spawn(player_chunk);

    for chunk_to_spawn in chunks_to_spawn {
        chunk_manager.set_chunk_being_spawned(chunk_to_spawn.clone());

        let chunk_size = chunk_manager.chunk_size.clone();

        let task  = thread_pool.spawn(async move {
            let chunk = Chunk::noise(chunk_size, chunk_to_spawn.clone());
            let mesh = generate_mesh(&chunk);

            RenderChunkMeshesTask {
                chunk,
                mesh,
            }
        });

        commands.spawn().insert(task);
    }
}

pub struct RenderChunkMeshesTask {
    mesh: Mesh,
    chunk: Chunk,
}

pub struct DespawnChunkTask {
    chunk: IVec3,
}

pub fn despawn_chunk_processor(
    mut commands: Commands,
    mut despawn_chunk_tasks: Query<(Entity, &mut Task<DespawnChunkTask>)>,
    mut chunk_manager_query: Query<&mut ChunkManager>
) {
    let mut chunk_manager = chunk_manager_query.single_mut();
    for (entity, mut task) in despawn_chunk_tasks.iter_mut() {
        if let Some(despawn_chunk_task) = future::block_on(future::poll_once(&mut *task)) {
            println!("Despawning chunk {} {} {}", despawn_chunk_task.chunk.x, despawn_chunk_task.chunk.y, despawn_chunk_task.chunk.z);
            let entity_to_despawn = chunk_manager.despawn_chunk(despawn_chunk_task.chunk);

            if entity_to_despawn.is_some() {
                commands.entity(entity_to_despawn.unwrap()).despawn();
            }
            commands.entity(entity).remove::<Task<DespawnChunkTask>>();
        }
    }
}

pub fn render_voxel_mesh(
    mut commands: Commands,
    mut transform_tasks: Query<(Entity, &mut Task<RenderChunkMeshesTask>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut chunk_manager_query: Query<&mut ChunkManager>) {

    let mut chunk_manager = chunk_manager_query.single_mut();

    for (entity, mut task) in transform_tasks.iter_mut() {
        if let Some(render_chunk_mesh_task) = future::block_on(future::poll_once(&mut *task)) {
            let chunk = render_chunk_mesh_task.chunk;
            let voxel_mesh = render_chunk_mesh_task.mesh;

            println!("Spawning chunk {} {} {}", chunk.location.x, chunk.location.y, chunk.location.z);

            let mut pbr_bundles = Vec::new();

            let chunk_transform = chunk.get_transform();

                pbr_bundles.push(PbrBundle {
                    mesh: meshes.add(voxel_mesh).clone(),
                    material: chunk_manager.clone_material(),
                    transform: chunk_transform,
                    ..Default::default()
                });

            for pbr_bundle in pbr_bundles.into_iter() {
                let entity = commands.spawn_bundle(pbr_bundle).id();
                chunk_manager.add_chunk_entity(SpawnedChunk {
                    chunk_location: chunk.location,
                    entity,
                });
            }

            // Water

            // let water_pos = chunk_transform.translation + Transform::from_xyz(CHUNK_SIZE as f32 * 0.5, 9.4, CHUNK_SIZE as f32 * 0.5).translation;
            //
            // commands.spawn_bundle(PbrBundle {
            //     mesh: meshes.add(Mesh::from(crate::shape::Box::new(CHUNK_SIZE as f32, 1.0, CHUNK_SIZE as f32))),
            //     material: materials.add(StandardMaterial {
            //         base_color: Color::rgba(0.0, 0.0, 0.8, 0.85),
            //         ..Default::default()
            //     }),
            //     visible: Visible { is_visible: true, is_transparent: true, },
            //     transform: Transform::from_translation(water_pos),
            //     ..Default::default()
            // });
            //
            // println!("Chunk {}, {}, {} loaded", chunk_x, chunk_y, chunk_z);

            commands.entity(entity).remove::<Task<RenderChunkMeshesTask>>();
        }
    }

}

//
// use bevy::DefaultPlugins;
// use bevy::pbr::PbrBundle;
// use bevy::prelude::{App, Assets, Commands, Entity, Mesh, Query, Res, ResMut};
// use bevy::prelude::shape::Cube;
// use bevy::prelude::IntoSystem;
// use bevy::tasks::{AsyncComputeTaskPool, Task};
// use futures_lite::future;
//
// fn load_meshes(
//     mut commands: Commands,
//     thread_pool: Res<AsyncComputeTaskPool>,
// ) {
//     for _ in 0..10 {
//         let task = thread_pool.spawn(async move {
//             println!("Starting work on another thread");
//             // Build expensive mesh here
//             let mesh = Mesh::from(Cube::new(1.0));
//             println!("Finished work on another thread");
//             mesh
//         });
//         commands.spawn().insert(task);
//     }
// }
//
// fn poll_mesh_tasks(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut tasks: Query<(Entity, &mut Task<Mesh>)>) {
//     for (entity, mut task) in tasks.iter_mut() {
//         if let Some(mesh) = future::block_on(future::poll_once(&mut *task)) {
//             let mesh_handle = meshes.add(mesh);
//             // Remove the Task<Mesh> from the entity and attach a PbrBundle to it
//             commands.entity(entity)
//                 .remove::<Task<Mesh>>()
//                 .insert_bundle(PbrBundle {
//                     mesh: mesh_handle,
//                     ..Default::default()
//                 });
//         }
//     }
// }