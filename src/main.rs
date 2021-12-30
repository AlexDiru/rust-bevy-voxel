mod map;
mod point;
mod chunk;
mod chunk_mesh;
mod vert_gen;
mod chunks;

#[macro_use]
extern crate exec_time;

use std::borrow::BorrowMut;
use bevy::prelude::*;
use bevy::render::camera::PerspectiveProjection;
use bevy::render::wireframe::{WireframePlugin};
use bevy::tasks::{AsyncComputeTaskPool, TaskPool, TaskPoolBuilder};
use bevy::wgpu::{WgpuFeature, WgpuFeatures, WgpuOptions};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use crate::chunk::{Chunk, CHUNK_SIZE, CHUNK_SIZE_I32};
use crate::chunk_mesh::{generate_chunk_mesh, Vertexes};

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
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let start_transform = Transform::from_translation(Vec3::new(32.0, 32.0, 32.0));

    let texture_handle : Handle<Texture> = asset_server.load("wall.png");

    let chunk_materials = ChunkMaterials {
        wall_material: materials.add(StandardMaterial {
            base_color_texture: Some(texture_handle.clone()),
            unlit: true,
            ..Default::default()
        })
    };

    commands.spawn().insert(chunk_materials);

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


    commands.spawn().insert(SpawnedChunks::new(get_chunk_containing_position(&start_transform.translation)));
}

fn main_spawner(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    chunk_mat_query: Query<&ChunkMaterials>,
) {

    let chunks_x = 3;
    let chunks_y = 2;
    let chunks_z = 3;

    println!("Getting chunk materials");
    let chunk_materials = chunk_mat_query.single().unwrap();
    println!("Done Getting chunk materials");

    for chunk_x in 0..chunks_x {
        for chunk_y in 0..chunks_y {
            for chunk_z in 0..chunks_z {
                let mut pbr_bundles = Vec::new();

                let vx_meshes = generate_mesh(chunk_x, chunk_z, chunk_y);

                for (_, vx_mesh) in vx_meshes.into_iter().enumerate() {
                    let chunk_transform = Transform::from_translation(Vec3::new(
                        (chunk_x * CHUNK_SIZE_I32) as f32,
                        (chunk_y * CHUNK_SIZE_I32) as f32,
                        (chunk_z * CHUNK_SIZE_I32) as f32));

                    pbr_bundles.push(PbrBundle {
                        mesh: meshes.add(vx_mesh).clone(),
                        material: chunk_materials.wall_material.clone(),
                        transform: chunk_transform,
                        ..Default::default()
                    });
                }

                for (_, pbr_bundle) in pbr_bundles.into_iter().enumerate() {
                    commands.spawn_bundle(pbr_bundle);
                }
            }
        }
    }
}

struct ChunkMaterials {
    wall_material: Handle<StandardMaterial>
}

struct SpawnedChunks {
    center: IVec3, // The chunk the player is in
    spawned_chunks: std::sync::Mutex<Vec<IVec3>> // all The spawned chunks, mutex for when the generation is multi-threaded
}

impl SpawnedChunks {
    pub fn new(center: IVec3) -> SpawnedChunks {
        SpawnedChunks {
            center,
            spawned_chunks: std::sync::Mutex::new(Vec::new()),
        }
    }

    pub fn request_next_chunk(&mut self) -> std::option::Option<IVec3> {
        for x in 0..5 {
            for y in 0..5 {
                for z in 0..3 {
                    // TODO HAS and SET need to be in the same lock
                    if !self.has_loaded(&IVec3::new(x, y ,z)) {
                        self.set_loaded(IVec3::new(x, y ,z));
                        return Option::Some(IVec3::new(x, y, z));
                    }
                }
            }
        }

        Option::None
    }

    pub fn has_loaded(&self, xyz: &IVec3) -> bool {
        let vec = self.spawned_chunks.lock().unwrap();
        vec.contains(xyz)
    }

    pub fn set_loaded(&mut self, xyz: IVec3) {
        let mut vec = self.spawned_chunks.lock().unwrap();
        vec.push(xyz);
    }
}

fn chunk_spawner(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    camera_query: Query<&Transform, With<FlyCamera>>,
    mut spawned_chunks_query: Query<&mut SpawnedChunks>,
    chunk_materials_query: Query<&ChunkMaterials>,
) {
    let camera_transform = camera_query.single().unwrap();
    let mut spawned_chunks = spawned_chunks_query.single_mut().unwrap();
    let chunk_materials = chunk_materials_query.single().unwrap();


    let chunk_vec = get_chunk_containing_position(&camera_transform.translation);
    //println!("Player is in Chunk region {} {} {}", chunk_vec.x, chunk_vec.y, chunk_vec.z);

    let next_chunk = spawned_chunks.request_next_chunk();
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
                    material: chunk_materials.wall_material.clone(),
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