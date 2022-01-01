mod map;
mod point;
mod chunk;
mod chunk_vertexes;
mod chunk_manager;
mod voxel;
mod chunk_mesh;
mod systems;

#[macro_use]
extern crate exec_time;

use bevy::prelude::*;
use bevy::render::camera::PerspectiveProjection;
use bevy::render::wireframe::{WireframePlugin};
use bevy::wgpu::{WgpuFeature, WgpuFeatures, WgpuOptions};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use crate::chunk::{Chunk, CHUNK_SIZE, CHUNK_SIZE_I32};
use crate::chunk_manager::{ChunkManager, get_chunk_containing_position};
use crate::chunk_mesh::generate_mesh;

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
            pitch: 13.0,
            yaw: 33884.0,
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

fn camera_debug_print(camera_query: Query<&FlyCamera>,) {
    let camera = camera_query.single().unwrap();
    println!("Camera Pitch {} Yaw {}", camera.pitch, camera.yaw);
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
        .add_system(systems::mouse_toggle::mouse_toggle.system())
        .add_system(systems::chunk_spawner::chunk_spawner.system())
        .add_system(systems::chunk_spawner::foobar.system())
        .run();
}

//
// fn main() {
//     App::build()
//         .add_plugins(DefaultPlugins)
//         .add_startup_system(load_meshes.system())
//         .add_system(poll_mesh_tasks.system())
//         .run();
// }