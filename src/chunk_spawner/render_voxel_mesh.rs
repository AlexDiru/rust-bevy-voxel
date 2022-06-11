use bevy::prelude::{Assets, Commands, Entity, Mesh, PbrBundle, Query, ResMut};
use bevy::tasks::Task;
use futures_lite::future;
use crate::chunk_manager::SpawnedChunk;
use crate::chunk_spawner::tasks::RenderChunkMeshesTask;
use crate::ChunkManager;

pub fn render_voxel_mesh(
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