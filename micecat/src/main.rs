use std::collections::HashMap;
use bevy::prelude::*;
use bevy_xpbd_3d::math::Vector;
use bevy_xpbd_3d::prelude::{ExternalForce, Gravity, PhysicsPlugins, RigidBody, Collider as Col, ExternalImpulse, GravityScale, LockedAxes};
use crate::chunk::Chunk;
use crate::chunk_map::ChunkMap;
use crate::player::*;

mod noise;
mod player;
mod chunk;
mod chunk_map;

mod noise3d;

fn main() {
    App::new()
        .insert_resource(ChunkMap { chunks: HashMap::new() })
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .insert_resource(Gravity(Vector::NEG_Y * 9.81))
        .add_systems(Startup, grab_cursor)
        .add_systems(Startup, (generate_chunks, spawn_player, setup_lighting))
        .add_systems(FixedUpdate, (mouse_look, player_movement))
        .run();
}

fn generate_chunks(
    mut commands: Commands,
    mut chunk_map: ResMut<ChunkMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for dx in -1..=1 {
        for dz in -1..=1 {
            let pos = (dx, dz);
            let chunk = Chunk::generate(pos);
            chunk.spawn(&mut commands, &mut meshes, &mut materials);
            chunk_map.chunks.insert(pos, chunk);
        }
    }

    commands.spawn((
        RigidBody::Static,
        Col::cuboid(1000.0, 0.1, 1000.0),
        Transform::from_xyz(0.0, -1.0, 0.0),
        GlobalTransform::default(),
    ));

}

pub fn spawn_player(mut commands: Commands, chunk_map: Res<ChunkMap>) {
    if let Some(chunk) = chunk_map.chunks.get(&(0, 0)) {
        let x = 8;
        let z = 8;

        for y in (0..chunk.blocks.len()).rev() {
            if chunk.blocks[y][x][z].is_some() {
                let spawn_y = y as f32 + 2.0; // un peu plus haut pour éviter le sol

                commands.spawn((
                    Player,
                    RigidBody::Dynamic,
                    Col::capsule(0.5, 0.9), // rayon 0.5, demi-hauteur 0.9 ≈ perso de 2.3m
                    ExternalForce::default(),
                    ExternalImpulse::default(),
                    GravityScale(1.0),
                    LockedAxes::ROTATION_LOCKED,
                    Transform::from_xyz(x as f32, spawn_y, z as f32),
                    GlobalTransform::default(),
                ))
                    .with_children(|parent| {
                        parent.spawn((
                            Camera3dBundle {
                                transform: Transform::from_xyz(0.0, 1.7, 0.0)
                                    .looking_at(Vec3::new(0.0, 1.7, 1.0), Vec3::Y),
                                ..default()
                            },
                            PlayerCamera { yaw: 0.0, pitch: 0.0 },
                        ));
                    });

                return;
            }
        }
    }

    // Fallback
    commands.spawn((
        Player,
        RigidBody::Dynamic,
        Col::capsule(0.5, 0.9),
        ExternalForce::default(),
        ExternalImpulse::default(),
        GravityScale(1.0),
        LockedAxes::ROTATION_LOCKED,
        Transform::from_xyz(8.0, 70.0, 8.0),
        GlobalTransform::default(),
    ))
        .with_children(|parent| {
            parent.spawn((
                Camera3dBundle {
                    transform: Transform::from_xyz(0.0, 1.7, 0.0)
                        .looking_at(Vec3::new(0.0, 1.7, 1.0), Vec3::Y),
                    ..default()
                },
                PlayerCamera { yaw: 0.0, pitch: 0.0 },
            ));
        });
}


fn setup_lighting(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 10000.0,
            ..default()
        },
        transform: Transform::from_xyz(100.0, 100.0, 100.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}


fn grab_cursor(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    window.cursor.visible = false;
    window.cursor.grab_mode = bevy::window::CursorGrabMode::Confined;
}