mod map;
mod point;

use std::f32::consts::PI;
use bevy::prelude::*;
use bevy::render::camera::PerspectiveProjection;
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use crate::map::Map;

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn().insert_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });

    let box_mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let wall_material = materials.add(Color::rgb(1.0, 0.2, 0.3).into());
    let floor_material = materials.add(Color::rgb(0.1, 0.7, 0.3).into());

    let map = Map::dfs_maze();

    for (_, wall) in map.get_walls().iter().enumerate() {
        commands.spawn().insert_bundle(PbrBundle {
            mesh: box_mesh.clone(),
            material: wall_material.clone(),
            transform: Transform::from_translation(Vec3::new(
                wall.x as f32, 0.0, wall.y as f32,
            )),
            ..Default::default()
        });
    }

    for (_, floor) in map.get_floors().iter().enumerate() {
        commands.spawn().insert_bundle(PbrBundle {
            mesh: box_mesh.clone(),
            material: floor_material.clone(),
            transform: Transform::from_translation(Vec3::new(
                floor.x as f32, -1.0, floor.y as f32,
            )),
            ..Default::default()
        });
    }

    commands
        .spawn()
        .insert_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(map.get_start().x as f32, 0.25, map.get_start().y as f32),
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
        .add_plugins(DefaultPlugins)
        .add_startup_system(init.system())
        .add_plugin(FlyCameraPlugin)
        .add_system(toggle_button_system.system())
        .run();
}