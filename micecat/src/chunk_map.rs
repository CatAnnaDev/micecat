use std::collections::HashMap;
use bevy::prelude::*;
use crate::chunk::{BlockType, Chunk, CHUNK_SIZE};

#[derive(Resource)]
pub struct ChunkMap {
    pub chunks: HashMap<(i32, i32), Chunk>,
}

impl ChunkMap {

    pub(crate) fn is_solid(&self, x: i32, y: i32, z: i32) -> bool {
        let chunk_x = x.div_euclid(CHUNK_SIZE as i32);
        let chunk_z = z.div_euclid(CHUNK_SIZE as i32);

        let local_x = x.rem_euclid(CHUNK_SIZE as i32);
        let local_z = z.rem_euclid(CHUNK_SIZE as i32);

        if let Some(chunk) = self.chunks.get(&(chunk_x, chunk_z)) {
            // Ici, chunk.blocks[y][local_x][local_z] est-il Some(_) ?
            if y >= 0 && (y as usize) < chunk.blocks.len() {
                if let Some(_) = chunk.blocks[y as usize][local_x as usize][local_z as usize] {
                    return true;
                }
            }
        }
        false
    }

}

pub fn generate_initial_chunks(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    for cx in -1..=1 {
        for cz in -1..=1 {
            let chunk = Chunk::generate((cx, cz));
            chunk.spawn(commands, meshes, materials);
        }
    }
}
