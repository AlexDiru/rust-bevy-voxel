use bevy::prelude::Handle;
use crate::{IVec3, StandardMaterial};

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

    pub fn clone_material(&self) -> Handle<StandardMaterial> {
        self.atlas.clone()
    }

    pub fn request_next_chunk(&mut self) -> std::option::Option<IVec3> {
        for x in 0..10 {
            for y in 0..2 {
                for z in 0..10 {
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