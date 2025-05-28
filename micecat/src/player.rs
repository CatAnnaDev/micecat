use std::ops::{Add, AddAssign, Mul, MulAssign};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::ExternalForce;
use crate::chunk_map::ChunkMap;

#[derive(Component)]
pub struct PlayerCamera {
    pub(crate) yaw: f32,
    pub(crate) pitch: f32,
}

#[derive(Component)]
pub struct Player;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec3);

#[derive(Component)]
pub struct PlayerCollider {
    size: Vec3,
}

fn is_colliding(pos: Vec3, size: Vec3, chunk_map: &ChunkMap) -> bool {
    let min = pos - size / 2.0;
    let max = pos + size / 2.0;

    for x in (min.x.floor() as i32)..=(max.x.floor() as i32) {
        for y in (min.y.floor() as i32)..=(max.y.floor() as i32) {
            for z in (min.z.floor() as i32)..=(max.z.floor() as i32) {
                if chunk_map.is_solid(x, y, z) {
                    return true;
                }
            }
        }
    }
    false
}


pub fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut ExternalForce, With<Player>>,
    player_transform_query: Query<&Transform, With<Player>>, // pour récupérer rotation du player (yaw)
    time: Res<Time>,
) {
    if let Ok(mut force) = query.get_single_mut() {
        if let Ok(player_transform) = player_transform_query.get_single() {
            let mut direction = Vec3::ZERO;

            let forward = player_transform.forward();
            let right = player_transform.right();

            if keys.pressed(KeyCode::KeyW) {
                direction += *forward;
            }
            if keys.pressed(KeyCode::KeyS) {
                direction -= *forward;
            }
            if keys.pressed(KeyCode::KeyD) {
                direction += *right;
            }
            if keys.pressed(KeyCode::KeyA) {
                direction -= *right;
            }

            direction.y = 0.0;

            if direction.length_squared() > 0.0 {
                direction = direction.normalize();
                let speed = 3.0;
                force.add_assign(direction * speed);
            }

            if keys.just_pressed(KeyCode::Space) {
                force.add_assign(Vec3::Y * 100.0);
            }

            if direction.length_squared() == 0.0 {
                force.mul_assign(0.2);
            }
        }
    }
}




pub fn mouse_look(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut param_set: ParamSet<(
        Query<&mut Transform, With<Player>>,            // 0 = player (yaw)
        Query<(&mut Transform, &mut PlayerCamera)>,     // 1 = camera (pitch)
    )>,
) {
    let sensitivity = 0.001;

    for event in mouse_motion_events.read() {
        if let Ok(mut player_transform) = param_set.p0().get_single_mut() {
            player_transform.rotation =
                player_transform.rotation * Quat::from_rotation_y(-event.delta.x * sensitivity);
        }

        if let Ok((mut camera_transform, mut player_cam)) = param_set.p1().get_single_mut() {
            player_cam.pitch = (player_cam.pitch - event.delta.y * sensitivity).clamp(-1.54, 1.54);
            camera_transform.rotation = Quat::from_rotation_x(player_cam.pitch);
        }
    }
}

fn is_on_ground(transform: &Transform, colliders: &Query<&Transform, (With<PlayerCollider>, Without<Player>)>,
) -> bool {
    let feet_pos = transform.translation + Vec3::new(0.0, -1.0, 0.0);
    colliders.iter().any(|col_trans| {
        let delta = col_trans.translation - feet_pos;
        delta.x.abs() < 0.5 && delta.y.abs() < 0.1 && delta.z.abs() < 0.5
    })
}

fn collides_with_world(pos: Vec3, colliders: &Query<&Transform, (With<PlayerCollider>, Without<Player>)>,
) -> bool {
    for col_trans in colliders.iter() {
        let delta = col_trans.translation - pos;
        if delta.x.abs() < 0.5 && delta.y.abs() < 1.0 && delta.z.abs() < 0.5 {
            return true;
        }
    }
    false
}