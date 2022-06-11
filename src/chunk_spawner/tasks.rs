use bevy::prelude::{IVec3, Mesh};
use crate::Chunk;

pub struct RenderChunkMeshesTask {
    pub mesh: Mesh,
    pub chunk: Chunk,
}

pub struct DespawnChunkTask {
    pub chunk: IVec3,
}