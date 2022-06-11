use bevy::app::App;
use bevy::math::IVec3;
use bevy::prelude::{Assets, Commands, Entity, Mesh, PbrBundle, Plugin, Query, Res, ResMut, Transform, With};
use bevy::tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;
use crate::{Chunk, ChunkManager, FlyCamera, generate_mesh};
use crate::chunk_manager::{get_chunk_containing_position, SpawnedChunk};
use crate::chunk_spawner::spawn_chunks::spawn_chunks;
use crate::chunk_spawner::render_voxel_mesh::render_voxel_mesh;
use crate::chunk_spawner::tasks::{DespawnChunkTask, RenderChunkMeshesTask};

pub struct ChunkSpawnerPlugin;

impl Plugin for ChunkSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_chunks)
            .add_system(render_voxel_mesh)
            .add_system(despawn_chunk_processor);
    }
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