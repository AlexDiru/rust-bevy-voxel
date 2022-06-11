use std::future;
use bevy::prelude::{Assets, Commands, Entity, Mesh, PbrBundle, Query, Res, ResMut, Transform, With};
use bevy::tasks::{AsyncComputeTaskPool, Task};
use crate::{ChunkManager, FlyCamera};
use crate::chunk_manager::SpawnedChunk;
use crate::chunk_spawner::tasks::RenderChunkMeshesTask;

pub mod chunk_spawner_plugin;
mod spawn_chunks;
mod tasks;
mod render_voxel_mesh;