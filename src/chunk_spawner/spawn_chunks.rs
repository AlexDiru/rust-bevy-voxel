use bevy::prelude::{Commands, IVec3, Query, Res, Transform, With};
use bevy::tasks::AsyncComputeTaskPool;
use crate::{Chunk, ChunkManager, FlyCamera, generate_mesh, get_chunk_containing_position};
use crate::chunk_spawner::tasks::{DespawnChunkTask, RenderChunkMeshesTask};

pub fn spawn_chunks(
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