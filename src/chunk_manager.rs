use std::cmp::Ordering::{Equal, Greater, Less};
use bevy::prelude::{Handle, Vec3};
use crate::{CHUNK_SIZE, IVec3, StandardMaterial};

pub struct ChunkManager {
    center: IVec3, // The chunk the player is in
    spawned_chunks: std::sync::Mutex<Vec<IVec3>>, // all The spawned chunks, mutex for when the generation is multi-threaded
    atlas: Handle<StandardMaterial>
}

impl ChunkManager {
    pub fn new(center: IVec3, atlas: Handle<StandardMaterial>) -> ChunkManager {
        ChunkManager {
            center,
            spawned_chunks: std::sync::Mutex::new(Vec::new()),
            atlas
        }
    }

    pub fn get_spawned_chunk_count(&self) -> usize {
        self.spawned_chunks.lock().unwrap().len()
    }

    pub fn clone_material(&self) -> Handle<StandardMaterial> {
        self.atlas.clone()
    }

    pub fn request_next_chunk(&mut self) -> std::option::Option<IVec3> {

        let mut inds = Vec::new();

        for x in -250..250 {
            for y in -250..250 {
                inds.push((x, y));
            }
        }

        inds.sort_by(|a: &(i32, i32), b: &(i32, i32)| {
            let max_a = i32::max(a.0.abs(), a.1.abs());
            let max_b = i32::max(b.0.abs(), b.1.abs());

            return i32::cmp(&max_a, &max_b);
        });


        // let inds = [
        //     (0, 0),
        //
        //     (0, 1),
        //     (0, -1),
        //     (1, 0),
        //     (1, 1),
        //     (1, -1),
        //     (-1, 0),
        //     (-1, 1),
        //     (-1, -1),
        //
        //     (2, -2),
        //     (2, -1),
        //     (2, 0),
        //     (2, 1),
        //     (2, 2)
        // ];

        let ring_inds = get_rings(&vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], &inds);

        for (x, y) in ring_inds {
            let chunk_x = x;
            let chunk_z = y;

            if !self.has_loaded(&IVec3::new(chunk_x, 0, chunk_z)) {
                self.set_loaded(IVec3::new(chunk_x, 0, chunk_z));
                return Option::Some(IVec3::new(chunk_x, 0, chunk_z));
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

pub fn get_chunk_containing_position(position: &Vec3) -> IVec3 {
    IVec3::new((position.x / CHUNK_SIZE as f32) as i32,
               (position.y / CHUNK_SIZE as f32) as i32,
               (position.z / CHUNK_SIZE as f32) as i32)
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