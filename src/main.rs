mod chunk_utils;
mod chunk;
mod chunk_vertexes;
mod chunk_manager;
mod voxel;
mod chunk_mesh;
mod systems;
mod biome;
mod flycamerafork;

#[macro_use]
extern crate exec_time;

use bevy::prelude::*;
use bevy::render::camera::PerspectiveProjection;
use flycamerafork::fly_camera::{FlyCamera, FlyCameraPlugin};
use crate::chunk::{Chunk};
use crate::chunk_manager::{ChunkManager, get_chunk_containing_position};
use crate::chunk_mesh::generate_mesh;

fn init(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    let start_transform = Transform::from_translation(Vec3::new(32.0, 64.0, 32.0));
    let fly_camera = FlyCamera {
        sensitivity: 10.0,
        friction: 5.0,
        accel: 10.0,
        max_speed: 1000.0,
        pitch: 13.0,
        yaw: 33884.0,
        ..Default::default()
    };

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
        .insert(fly_camera);

    let chunk_size = IVec3::new(32, 64, 32);
    let center_chunk_location = get_chunk_containing_position(&start_transform.translation, &chunk_size);
    let atlas = materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("atlas.png").clone()),
        unlit: true,
        ..Default::default()
    });

    commands.spawn().insert(ChunkManager::new(
        chunk_size,
        center_chunk_location,
        atlas
    ));
}

fn camera_debug_print(camera_query: Query<&FlyCamera>,) {
    let camera = camera_query.single();
    println!("Camera Pitch {} Yaw {}", camera.pitch, camera.yaw);
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.8)))
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(bevy::render::settings::WgpuFeatures::all_native_mask())
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy::pbr::wireframe::WireframePlugin)
        .add_startup_system(init)
        .add_plugin(FlyCameraPlugin)
        .add_system(systems::mouse_toggle::mouse_toggle)
        .add_system(systems::chunk_spawner::chunk_spawner)
        .add_system(systems::chunk_spawner::render_voxel_mesh)
        .add_system(systems::chunk_spawner::despawn_chunk_processor)
        .run();
}