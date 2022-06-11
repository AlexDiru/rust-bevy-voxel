use bevy::app::App;
use bevy::math::IVec3;
use bevy::prelude::{Assets, Commands, Entity, Mesh, PbrBundle, Plugin, Query, Res, ResMut, Transform, With};
use bevy::tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;
use crate::{Chunk, ChunkManager, FlyCamera, generate_mesh};
use crate::chunk_manager::{get_chunk_containing_position, SpawnedChunk};

pub struct ChunkSpawnerPlugin;

impl Plugin for ChunkSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(chunk_spawner)
            .add_system(render_voxel_mesh)
            .add_system(despawn_chunk_processor);
    }
}

fn chunk_spawner(
    mut commands: Commands,
    camera_query: Query<&Transform, With<FlyCamera>>,
    mut chunk_manager_query: Query<&mut ChunkManager>,
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

        let task = thread_pool.spawn(async move {
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

fn despawn_chunk_processor(
    mut commands: Commands,
    mut despawn_chunk_tasks: Query<(Entity, &mut Task<DespawnChunkTask>)>,
    mut chunk_manager_query: Query<&mut ChunkManager>
) {
    let mut chunk_manager = chunk_manager_query.single_mut();
    for (entity, mut task) in despawn_chunk_tasks.iter_mut() {
        if let Some(despawn_chunk_task) = future::block_on(future::poll_once(&mut *task)) {
            println!("Despawning chunk_spawner {} {} {}", despawn_chunk_task.chunk.x, despawn_chunk_task.chunk.y, despawn_chunk_task.chunk.z);
            let entity_to_despawn = chunk_manager.despawn_chunk(despawn_chunk_task.chunk);

            if entity_to_despawn.is_some() {
                commands.entity(entity_to_despawn.unwrap()).despawn();
            }
            commands.entity(entity).remove::<Task<DespawnChunkTask>>();
        }
    }
}

fn render_voxel_mesh(
    mut commands: Commands,
    mut transform_tasks: Query<(Entity, &mut Task<RenderChunkMeshesTask>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut chunk_manager_query: Query<&mut ChunkManager>) {

    let mut chunk_manager = chunk_manager_query.single_mut();

    for (entity, mut task) in transform_tasks.iter_mut() {
        if let Some(render_chunk_mesh_task) = future::block_on(future::poll_once(&mut *task)) {
            let chunk = render_chunk_mesh_task.chunk;
            let voxel_mesh = render_chunk_mesh_task.mesh;

            println!("Spawning chunk_spawner {} {} {}", chunk.location.x, chunk.location.y, chunk.location.z);

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

            commands.entity(entity).remove::<Task<RenderChunkMeshesTask>>();
        }
    }

}