use std::cmp::Ordering::{Equal, Greater, Less};
use std::ops::Range;
use std::thread::spawn;
use bevy::prelude::{Handle, Vec3};
use bevy::prelude::Entity;
use crate::{CHUNK_SIZE, Commands, IVec3, StandardMaterial};

pub struct SpawnedChunk {
    pub chunk: IVec3,
    pub entity: Entity
}

pub struct ChunkManager {
    center: IVec3, // The chunk the player is in
    atlas: Handle<StandardMaterial>,


    chunks_currently_being_spawned: std::sync::Mutex<Vec<IVec3>>,
    spawned_chunks: std::sync::Mutex<Vec<SpawnedChunk>>,

    chunk_render_distance: i32,
    chunk_render_distance_y_range: Range<i32>,
}

impl ChunkManager {
    pub fn new(center: IVec3, atlas: Handle<StandardMaterial>) -> ChunkManager {
        ChunkManager {
            center,
            atlas,
            spawned_chunks: std::sync::Mutex::new(Vec::new()),
            chunks_currently_being_spawned: std::sync::Mutex::new(Vec::new()), // The thread is doing work to spawn this chunk, once spawned it is removed from here and pushed to spawned_chunks
            chunk_render_distance: 3,
            chunk_render_distance_y_range: 0..1,
        }
    }

    pub fn is_chunk_spawned(&self, chunk: &IVec3) -> bool {
        for spawned_chunk in self.spawned_chunks.lock().unwrap().iter() {
            if spawned_chunk.chunk == *chunk {
                return true
            }
        }

        false
    }

    pub fn is_chunk_spawned_or_spawning(&self, chunk: &IVec3) -> bool {
        let chunks_currently_being_spawned = self.chunks_currently_being_spawned.lock().unwrap();

        if chunks_currently_being_spawned.contains(chunk) {
            return true;
        }

        if self.is_chunk_spawned(chunk) {
            return true;
        }

        false
    }

    pub fn clone_material(&self) -> Handle<StandardMaterial> {
        self.atlas.clone()
    }

    pub fn chunks_to_despawn(&mut self, center_chunk: IVec3) -> Vec<IVec3> {
        let mut chunks_in_render_zone = self.chunks_in_render_zone_worldspace(&center_chunk);

        let spawned_chunks = self.spawned_chunks.lock().unwrap();

        let mut to_despawn = Vec::new();
        for spawned_chunk in spawned_chunks.iter() {
            if !chunks_in_render_zone.contains(&spawned_chunk.chunk) {
                println!("Adding chunk {} {} {} to despawn list because not the same as center_chunk {} {} {}",
                         spawned_chunk.chunk.x, spawned_chunk.chunk.y, spawned_chunk.chunk.z,
                center_chunk.x, center_chunk.y, center_chunk.z);
                to_despawn.push(spawned_chunk.chunk.clone());
            }
        }

        to_despawn
    }

    pub fn add_chunk_entity(&mut self, spawned_chunk: SpawnedChunk) {
        let mut chunks_currently_being_spawned = self.chunks_currently_being_spawned.lock().unwrap();
        chunks_currently_being_spawned.retain(|v| *v != spawned_chunk.chunk);

        self.spawned_chunks.lock().unwrap().push(spawned_chunk);
    }

    pub fn despawn_chunk(&mut self, chunk: IVec3) -> std::option::Option<Entity> {
        let mut spawned_chunks = self.spawned_chunks.lock().unwrap();

        // TODO if chunk is being spawned right now, find some way to cancel it, and remove from being_spawned vec

        for i in 0..spawned_chunks.len() {
            if spawned_chunks.get(i).unwrap().chunk == chunk {
                let entity = spawned_chunks.get(i).unwrap().entity;
                spawned_chunks.remove(i);
                return Some(entity);
            }
        }

        None
    }

    pub fn set_chunk_being_spawned(&mut self, chunk: IVec3) {
        self.chunks_currently_being_spawned.lock().unwrap().push(chunk);
    }

    pub fn chunks_in_render_zone_worldspace(&self, center_chunk: &IVec3) -> Vec<IVec3> {
        let mut chunks_in_render_zone = Vec::new();

        let range = -self.chunk_render_distance..self.chunk_render_distance;

        for x in range.clone() {
            for y in range.clone() {
                for z in self.chunk_render_distance_y_range.clone() {
                    chunks_in_render_zone.push(IVec3::new(x, z, y));
                }
            }
        }
        chunks_in_render_zone.sort_by(|a: &IVec3, b: &IVec3| {
            let max_a = i32::max(i32::max(a.x.abs(), a.y.abs()), a.z.abs());
            let max_b = i32::max(i32::max(b.x.abs(), b.y.abs()), b.z.abs());

            return i32::cmp(&max_a, &max_b);
        });

        let mut chunks_in_render_zone_worldspace = Vec::new();

        for mut chunk in chunks_in_render_zone {
            chunks_in_render_zone_worldspace.push(chunk + *center_chunk);
        }

        chunks_in_render_zone_worldspace
    }

    pub fn request_chunks_to_spawn(&mut self, center_chunk: IVec3) -> Vec<IVec3> {
        let mut chunks_in_render_zone = self.chunks_in_render_zone_worldspace(&center_chunk);

        let mut chunks_to_spawn = Vec::new();

        for chunk in chunks_in_render_zone {
            if !self.is_chunk_spawned_or_spawning(&chunk) {
                chunks_to_spawn.push(chunk.clone());
            }
        }

        chunks_to_spawn
    }
}

pub fn get_chunk_containing_position(position: &Vec3) -> IVec3 {
    let mut chunk_offset_x = 0;
    let mut chunk_offset_y = 0;
    let mut chunk_offset_z = 0;

    if position.x < 0.0 {
        chunk_offset_x = -1;
    }

    if position.y < 0.0 {
        chunk_offset_y = -1;
    }

    if position.z < 0.0 {
        chunk_offset_z = -1;
    }

    IVec3::new((position.x / CHUNK_SIZE as f32) as i32 + chunk_offset_x,
               (position.y / CHUNK_SIZE as f32) as i32 + chunk_offset_y,
               (position.z / CHUNK_SIZE as f32) as i32 + chunk_offset_z)
}

fn get_rings(layers: &Vec<i32>, inds: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut ring_indexes = Vec::new();
    for layer in layers {
        ring_indexes.extend_from_slice(&get_ring(layer, inds));
    }
    ring_indexes
}

fn get_ring(layer: &i32, inds: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    // 0 -> 0
    // layer = 1 -> 1,2,3,4,5,6,7,8 -> (1^2)..(3^2)
    // layer = 2 -> 9 ... 24        -> (3^2)...(5^2 - 1)
    // layer = 3 -> (5^2) -> (7^2 - 1)

    if *layer == 0 {
        return inds[0..1].to_vec();
    } else {
        let start = ((layer * 2) - 1) as usize;
        let end = start + 2;

        println!("Inds: {} {}", start*start, end*end);
        return inds[(start*start)..(end*end)].to_vec();
    }
}