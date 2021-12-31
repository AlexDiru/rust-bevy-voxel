use bevy::input::Input;
use bevy::math::IVec3;
use bevy::prelude::{Assets, Commands, KeyCode, Mesh, PbrBundle, Query, Res, ResMut, Transform, With};
use bevy::reflect::erased_serde::private::serde::de::Unexpected::Option;
use bevy::render::texture::TextureViewDimension::Cube;
use bevy::tasks::{AsyncComputeTaskPool, ComputeTaskPool};
use bevy_fly_camera::FlyCamera;
use crate::{CHUNK_SIZE, CHUNK_SIZE_I32, ChunkManager, generate_mesh, Vec3};
use crate::chunk_manager::get_chunk_containing_position;


pub fn chunk_spawner(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    camera_query: Query<&Transform, With<FlyCamera>>,
    mut chunk_manager_query: Query<&mut ChunkManager>,
    input: Res<Input<KeyCode>>,
    pool: Res<ComputeTaskPool>,
) {
    let camera_transform = camera_query.single().unwrap();
    let mut chunk_manager = chunk_manager_query.single_mut().unwrap();


    let chunk_vec = get_chunk_containing_position(&camera_transform.translation);
    //println!("Player is in Chunk region {} {} {}", chunk_vec.x, chunk_vec.y, chunk_vec.z);

    let mut next_chunk: std::option::Option<IVec3>  = None;

    if chunk_manager.get_spawned_chunk_count() == 0 || input.just_pressed(KeyCode::P) {
        next_chunk = chunk_manager.request_next_chunk();
    }

    if next_chunk.is_some() {

        let u_next_chunk = next_chunk.unwrap();
        let chunk_x = u_next_chunk.x;
        let chunk_y = u_next_chunk.y;
        let chunk_z = u_next_chunk.z;

        // TODO how to make async? Bevy has fuck all documentation for task

        let mut spawn_next_chunk = || {
            println!("Chunk {}, {}, {} has not been loaded, spawning it now", chunk_x, chunk_y, chunk_z);
            let voxel_meshes = generate_mesh(chunk_x, chunk_z, chunk_y);

            let mut pbr_bundles = Vec::new();

            for voxel_mesh in voxel_meshes {
                let chunk_transform = Transform::from_translation(Vec3::new(
                    (chunk_x * CHUNK_SIZE_I32) as f32,
                    (chunk_y * CHUNK_SIZE_I32) as f32,
                    (chunk_z * CHUNK_SIZE_I32) as f32));

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

            println!("Chunk {}, {}, {} loaded", chunk_x, chunk_y, chunk_z);
        };

        spawn_next_chunk();
    }
}