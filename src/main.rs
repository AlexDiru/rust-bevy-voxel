mod map;
mod point;
mod chunk;
mod chunk_vertexes;
mod vert_gen;
mod chunks;
mod chunk_manager;
mod voxel;
mod chunk_mesh;

#[macro_use]
extern crate exec_time;

use std::borrow::BorrowMut;
use std::cmp::min;
use bevy::prelude::*;
use bevy::render::camera::PerspectiveProjection;
use bevy::render::wireframe::{WireframePlugin};
use bevy::tasks::{AsyncComputeTaskPool, TaskPool, TaskPoolBuilder};
use bevy::wgpu::{WgpuFeature, WgpuFeatures, WgpuOptions};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use crate::chunk::{Chunk, CHUNK_SIZE, CHUNK_SIZE_I32};
use crate::chunk_manager::ChunkManager;
use crate::chunk_mesh::generate_mesh;
use crate::chunk_vertexes::generate_chunk_vertexes;

fn init(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let start_transform = Transform::from_translation(Vec3::new(32.0, 32.0, 32.0));

    commands
        .spawn()
        .insert_bundle(PerspectiveCameraBundle {
            transform: start_transform,
            perspective_projection: PerspectiveProjection {
                near: 0.01,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(FlyCamera {
            sensitivity: 10.0,
            friction: 5.0,
            accel: 5.0,
            yaw: 225.0,
            ..Default::default()
        });


    commands.spawn().insert(ChunkManager::new(
        get_chunk_containing_position(&start_transform.translation),
            materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.load("atlas.png").clone()),
                unlit: true,
                ..Default::default()
            })
        ));
}

fn chunk_spawner(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    camera_query: Query<&Transform, With<FlyCamera>>,
    mut chunk_manager_query: Query<&mut ChunkManager>,
) {
    let camera_transform = camera_query.single().unwrap();
    let mut chunk_manager = chunk_manager_query.single_mut().unwrap();


    let chunk_vec = get_chunk_containing_position(&camera_transform.translation);
    //println!("Player is in Chunk region {} {} {}", chunk_vec.x, chunk_vec.y, chunk_vec.z);

    let next_chunk = chunk_manager.request_next_chunk();
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
                    mesh: meshes.add( voxel_mesh).clone(),
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
    } else {
        println!("No Chunks to spawn");
    }
}

fn get_chunk_containing_position(position: &Vec3) -> IVec3 {
    IVec3::new((position.x / CHUNK_SIZE as f32) as i32,
               (position.y / CHUNK_SIZE as f32) as i32,
              (position.z / CHUNK_SIZE as f32) as i32)
}

// Press "T" to toggle keyboard+mouse control over the camera
fn toggle_button_system(
    input: Res<Input<KeyCode>>,
    mut query: Query<&mut FlyCamera>,
) {
    for mut options in query.iter_mut() {
        if input.just_pressed(KeyCode::T) {
            println!("Toggled FlyCamera enabled!");
            options.enabled = !options.enabled;
        }
    }
}

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.8)))
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WgpuOptions {
            features: WgpuFeatures {
                features: vec![WgpuFeature::NonFillPolygonMode]
            },
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WireframePlugin)
        .add_startup_system(init.system())
        .add_plugin(FlyCameraPlugin)
        .add_system(toggle_button_system.system())
        .add_system(chunk_spawner.system())
        .run();
}