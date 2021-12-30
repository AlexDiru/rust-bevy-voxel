mod map;
mod point;

use std::f32::consts::PI;
use bevy::prelude::*;
use bevy::render::camera::PerspectiveProjection;
use bevy::render::pipeline::PrimitiveTopology;
use bevy::render::RenderPlugin;
use bevy::render::texture::TextureViewDimension::Cube;
use bevy::render::wireframe::{Wireframe, WireframePlugin};
use bevy::wgpu::{WgpuFeature, WgpuFeatures, WgpuOptions};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use crate::map::Map;
use crate::point::Point;

fn top_plane_vertices(x_offset: f32, y_offset: f32, z_offset: f32) -> [([f32; 3], [f32; 3], [f32; 2]); 6] {
    [
        ([0.0 + x_offset, 1.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([0.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([1.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([1.0 + x_offset, 1.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([0.0 + x_offset, 1.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([1.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
    ]
}

fn left_plane_vertices(x_offset: f32, y_offset: f32, z_offset: f32) -> [([f32; 3], [f32; 3], [f32; 2]); 6] {
    [
        ([0.0 + x_offset, 0.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([0.0 + x_offset, 1.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([1.0 + x_offset, 1.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([0.0 + x_offset, 0.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([1.0 + x_offset, 1.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([1.0 + x_offset, 0.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
    ]
}

fn right_plane_vertices(x_offset: f32, y_offset: f32, z_offset: f32) -> [([f32; 3], [f32; 3], [f32; 2]); 6] {
    [
        ([1.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([0.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([0.0 + x_offset, 0.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([1.0 + x_offset, 0.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([1.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([0.0 + x_offset, 0.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
    ]
}

fn bottom_plane_vertices() -> [([f32; 3], [f32; 3], [f32; 2]); 6] {
    [
        ([1.0, 0.0, 1.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([0.0, 0.0, 1.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([1.0, 0.0, 1.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
    ]
}

fn back_plane_vertices(x_offset: f32, y_offset: f32, z_offset: f32) -> [([f32; 3], [f32; 3], [f32; 2]); 6] {
    [
        ([1.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([1.0 + x_offset, 0.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([1.0 + x_offset, 0.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([1.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([1.0 + x_offset, 0.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([1.0 + x_offset, 1.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
    ]
}

fn front_plane_vertices(x_offset: f32, y_offset: f32, z_offset: f32) -> [([f32; 3], [f32; 3], [f32; 2]); 6] {
    [
        ([0.0 + x_offset, 0.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([0.0 + x_offset, 0.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([0.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([0.0 + x_offset, 1.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([0.0 + x_offset, 0.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([0.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
    ]
}

fn generate_mesh(points: &Vec<Point>) -> Mesh {
    let mut vertices = Vec::new();
    for (_, point) in points.iter().enumerate() {
        vertices.extend_from_slice(&top_plane_vertices(point.x as f32, 0.0, point.y as f32));

        // if up point doesn't exist, render back wall
        if !points.contains(&point.right()) {
            vertices.extend_from_slice(&back_plane_vertices(point.x as f32, 0.0, point.y as f32));
        }

        if !points.contains(&point.left()) {
            vertices.extend_from_slice(&front_plane_vertices(point.x as f32, 0.0, point.y as f32));
        }

        if !points.contains(&point.down()) {
            vertices.extend_from_slice(&left_plane_vertices(point.x as f32, 0.0, point.y as f32));
        }

        if !points.contains(&point.up()) {
            vertices.extend_from_slice(&right_plane_vertices(point.x as f32, 0.0, point.y as f32));
        }

    }

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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn().insert_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });

    let map = Map::dfs_maze();
    let wall_mesh = meshes.add(generate_mesh(map.get_walls()));
    let floor_mesh = meshes.add(generate_mesh(map.get_floors()));

    let indices = bevy::render::mesh::Indices::U32(vec![0, 2, 1, 0, 3, 2]);

    let wall_material = materials.add(Color::rgb(1.0, 0.2, 0.3).into());
    let floor_material = materials.add(Color::rgb(0.1, 0.7, 0.3).into());

    commands.spawn().insert_bundle(PbrBundle {
        mesh: wall_mesh.clone(),
        material: wall_material.clone(),
        transform: Transform::from_translation(Vec3::ZERO),
        ..Default::default()
    }).insert(Wireframe);

    commands.spawn().insert_bundle(PbrBundle {
        mesh: floor_mesh.clone(),
        material: floor_material.clone(),
        transform: Transform::from_xyz(0.0, -1.0, 0.0),
        ..Default::default()
    });

    commands
        .spawn()
        .insert_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(map.get_start().x as f32 + 0.5, 0.25, map.get_start().y as f32 + 0.5),
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