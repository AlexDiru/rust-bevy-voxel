use bevy::input::Input;
use bevy::math::IVec3;
use bevy::prelude::{Assets, Color, Commands, Entity, KeyCode, Mesh, PbrBundle, Query, Res, ResMut, Transform, Visible, With};
use bevy::tasks::{AsyncComputeTaskPool, Task};
use bevy_fly_camera::FlyCamera;
use futures_lite::future;
use crate::{CHUNK_SIZE, CHUNK_SIZE_I32, ChunkManager, generate_mesh, StandardMaterial, Vec3};
use crate::chunk_manager::get_chunk_containing_position;


pub fn chunk_spawner(
    mut commands: Commands,
    camera_query: Query<&Transform, With<FlyCamera>>,
    mut chunk_manager_query: Query<&mut ChunkManager>,
    input: Res<Input<KeyCode>>,
    thread_pool: Res<AsyncComputeTaskPool>,
) {
    let camera_transform = camera_query.single().unwrap();
    let mut chunk_manager = chunk_manager_query.single_mut().unwrap();


    let chunk_vec = get_chunk_containing_position(&camera_transform.translation);
    //println!("Player is in Chunk region {} {} {}", chunk_vec.x, chunk_vec.y, chunk_vec.z);

    let mut next_chunk: std::option::Option<IVec3> = None;

    if chunk_manager.get_spawned_chunk_count() < 250 {
        next_chunk = chunk_manager.request_next_chunk();
    }

    if next_chunk.is_some() {

        let u_next_chunk = next_chunk.unwrap();
        let chunk_x = u_next_chunk.x;
        let chunk_y = u_next_chunk.y;
        let chunk_z = u_next_chunk.z;

        // TODO how to make async? Bevy has fuck all documentation for task
        let task  = thread_pool.spawn(async move {
            println!("Chunk {}, {}, {} has not been loaded, spawning it now", chunk_x, chunk_y, chunk_z);
            VoxelMeshesTask {
                meshes: generate_mesh(chunk_x, chunk_z, chunk_y),
                chunk_x,
                chunk_y,
                chunk_z
            }
        });

        commands.spawn().insert(task);
    }
}

pub struct VoxelMeshesTask {
    meshes: Vec<Mesh>,
    chunk_x: i32,
    chunk_y: i32,
    chunk_z: i32
}

pub fn foobar(
    mut commands: Commands,
    mut transform_tasks: Query<(Entity, &mut Task<VoxelMeshesTask>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut chunk_manager_query: Query<&mut ChunkManager>) {

    let mut chunk_manager = chunk_manager_query.single_mut().unwrap();

    for (entity, mut task) in transform_tasks.iter_mut() {
        if let Some(voxel_mesh_task) = future::block_on(future::poll_once(&mut *task)) {
            println!("THREAD");
            let chunk_x = voxel_mesh_task.chunk_x;
            let chunk_y = voxel_mesh_task.chunk_y;
            let chunk_z = voxel_mesh_task.chunk_z;
            let voxel_meshes = voxel_mesh_task.meshes;

            let mut pbr_bundles = Vec::new();

            let chunk_transform = Transform::from_translation(Vec3::new(
                (chunk_x * CHUNK_SIZE_I32) as f32,
                (chunk_y * CHUNK_SIZE_I32) as f32,
                (chunk_z * CHUNK_SIZE_I32) as f32));

            for voxel_mesh in voxel_meshes {
                pbr_bundles.push(PbrBundle {
                    mesh: meshes.add(voxel_mesh).clone(),
                    material: chunk_manager.clone_material(),
                    transform: chunk_transform,
                    ..Default::default()
                });
            }

            for pbr_bundle in pbr_bundles.into_iter() {
                commands.spawn_bundle(pbr_bundle);
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

            commands.entity(entity).remove::<Task<VoxelMeshesTask>>();
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