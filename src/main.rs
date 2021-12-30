mod map;
mod point;
mod chunk;
mod chunk_mesh;
mod vert_gen;
mod chunks;

#[macro_use]
extern crate exec_time;

use std::f32::consts::PI;
use bevy::pbr::AmbientLight;
use bevy::prelude::*;
use bevy::render::camera::PerspectiveProjection;
use bevy::render::pipeline::PrimitiveTopology;
use bevy::render::RenderPlugin;
use bevy::render::texture::TextureViewDimension::Cube;
use bevy::render::wireframe::{Wireframe, WireframePlugin};
use bevy::wgpu::{WgpuFeature, WgpuFeatures, WgpuOptions};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use crate::chunk::{Chunk, CHUNK_SIZE, CHUNK_SIZE_I32};
use crate::chunk_mesh::{generate_chunk_mesh, Vertexes};
use crate::map::Map;
use crate::point::Point;
use crate::vert_gen::{back_plane_vertices, front_plane_vertices, left_plane_vertices, right_plane_vertices, top_plane_vertices};

fn generate_mesh(chunk_x: i32, chunk_y: i32, chunk_z: i32) -> Vec<Mesh> {
    let mut meshes = Vec::new();
    let vertices_arr = generate_chunk_mesh(&Chunk::noise(chunk_x, chunk_y, chunk_z));

    for (_, vertices) in vertices_arr.iter().enumerate() {
        meshes.push(create_chunk_mesh(vertices));
    }

    meshes
}

fn create_chunk_mesh(vertices: &Vertexes) -> Mesh {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    for (position, normal, uv) in vertices.iter() {
        positions.push(*position);
        normals.push(*normal);
        uvs.push(*uv);
    }

    let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::TriangleList);
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh
}

fn init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let enable_wireframe = false;

    let chunks_x = 3;
    let chunks_y = 2;
    let chunks_z = 3;

    let start_transform = Transform::from_translation(Vec3::new(
        (chunks_x * CHUNK_SIZE_I32) as f32 * 0.5,
        (CHUNK_SIZE_I32 * 2) as f32,
        (chunks_z * CHUNK_SIZE_I32) as f32 * 0.5));

    let texture_handle : Handle<Texture> = asset_server.load("wall.png");

    let wall_material = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        unlit: true,
        ..Default::default()
    });

    for chunk_x in 0..chunks_x {
        for chunk_y in 0..chunks_y {
            for chunk_z in 0..chunks_z {
                let vx_meshes = generate_mesh(chunk_x, chunk_z, chunk_y);

                for (_, vx_mesh) in vx_meshes.into_iter().enumerate() {
                    let wall_mesh = meshes.add(vx_mesh);

                    let chunk_transform = Transform::from_translation(Vec3::new(
                        (chunk_x * CHUNK_SIZE_I32) as f32,
                        (chunk_y * CHUNK_SIZE_I32) as f32,
                        (chunk_z * CHUNK_SIZE_I32) as f32));

                    commands.spawn().insert_bundle(PbrBundle {
                        mesh: wall_mesh.clone(),
                        material: wall_material.clone(),
                        transform: chunk_transform,
                        ..Default::default()
                    });

                    if enable_wireframe {
                        Wireframe;
                    }
                }
            }
        }
    }




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
        .run();
}