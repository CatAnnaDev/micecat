use crate::noise::*;
use bevy::pbr::PbrBundle;
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::{Collider, Position, RigidBody, Rotation};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum BlockType {
    Air,
    Grass,
    Dirt,
    Stone,
}

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_HEIGHT: usize = 32;

#[derive(Debug)]
pub struct Chunk {
    pub position: (i32, i32),
    pub blocks: Vec<Vec<Vec<Option<BlockType>>>>, // Y, X, Z
}

impl Chunk {
    pub fn generate(pos: (i32, i32)) -> Self {
        let mut blocks = vec![vec![vec![None; 16]; 16]; 128]; // hauteur 128

        for x in 0..16 {
            for z in 0..16 {
                let world_x = pos.0 * 16 + x as i32;
                let world_z = pos.1 * 16 + z as i32;

                let height =
                    (perlin2d(world_x as f32 * 0.05, world_z as f32 * 0.05) * 20.0) as usize + 64;

                for y in 0..=height.min(127) {
                    let block = if y == height {
                        BlockType::Grass
                    } else if y > height - 4 {
                        BlockType::Dirt
                    } else {
                        BlockType::Stone
                    };

                    blocks[y][x][z] = Some(block);
                }
            }
        }

        Self {
            position: pos,
            blocks,
        }
    }

    pub fn spawn(
        &self,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) {
        let cube = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
        let grass = materials.add(Color::rgb(0.1, 0.8, 0.1));
        let dirt = materials.add(Color::rgb(0.6, 0.4, 0.2));
        let stone = materials.add(Color::rgb(0.4, 0.4, 0.4));

        let mut colliders: Vec<(Position, Rotation, Collider)> = Vec::new();

        for x in 0..16 {
            for z in 0..16 {
                for y in 0..128 {
                    if let Some(block) = self.blocks[y][x][z] {
                        let material = match block {
                            BlockType::Grass => grass.clone(),
                            BlockType::Dirt => dirt.clone(),
                            BlockType::Stone => stone.clone(),
                            BlockType::Air => continue,
                        };

                        // Spawn rendu PBR seulement (pas de collider ici)
                        commands.spawn(PbrBundle {
                            mesh: cube.clone(),
                            material,
                            transform: Transform::from_xyz(
                                (self.position.0 * 16 + x as i32) as f32,
                                y as f32,
                                (self.position.1 * 16 + z as i32) as f32,
                            ),
                            ..default()
                        });

                        // Ajoute au collider compound
                        let transform = Position::from_xyz(
                            (x as f32) + 0.0,
                            y as f32,
                            (z as f32) + 0.0,
                        );
                        let cuboid = Collider::cuboid(0.5, 0.5, 0.5);
                        let rotation = Quat::IDENTITY;
                        colliders.push((transform, rotation.into(), cuboid));
                    }
                }
            }
        }

        // Spawn du collider unique pour tout le chunk (et statique)
        commands.spawn((
            Name::new(format!("Chunk collider {:?}", self.position)),
            RigidBody::Static,
            Collider::compound(colliders),
            Transform::from_xyz(
                (self.position.0 * 16) as f32,
                0.0,
                (self.position.1 * 16) as f32,
            ),
            GlobalTransform::default(),
        ));
    }


    pub fn is_solid_block(&self, x: i32, y: i32, z: i32) -> bool {
        println!("Checking solidity at block coords: {}, {}, {}", x, y, z);
        if x < 0
            || y < 0
            || z < 0
            || x >= CHUNK_SIZE as i32
            || y >= CHUNK_HEIGHT as i32
            || z >= CHUNK_SIZE as i32
        {
            return false;
        }

        self.blocks[x as usize][y as usize][z as usize].is_some()
    }
}
