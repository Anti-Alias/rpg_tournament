use std::f32::consts::PI;

use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

#[derive(Component)]
pub struct Flycam {
    pub speed: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub sensitivity: f32,
}

impl Default for Flycam {
    fn default() -> Self {
        Self {
            speed: 256.0,
            yaw: 0.0,
            pitch: -PI/4.0,
            sensitivity: 0.005,
        }
    }
}

const EPS: f32 = 0.001;

pub fn control_flycam(
    mut flycams: Query<(&mut Transform, &mut Flycam)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut mouse_motions: EventReader<MouseMotion>,
    time: Res<Time>,
) {
    let secs = time.delta_seconds();
    for (mut transform, mut flycam) in &mut flycams {

        // Rotates flycam
        if mouse.pressed(MouseButton::Middle) {
            for mouse_motion in mouse_motions.read() {
                flycam.yaw -= mouse_motion.delta.x * flycam.sensitivity;
                flycam.pitch -= mouse_motion.delta.y * flycam.sensitivity;
                flycam.pitch = flycam.pitch.min(PI/2.0 - EPS).max(-PI/2.0 + EPS);
            }
            let look_dir = Quat::from_euler(EulerRot::YXZ, flycam.yaw, flycam.pitch, 0.0) * Vec3::NEG_Z;
            *transform = transform.looking_to(look_dir, Vec3::Y);
        }

        // Moves flycam
        let rotation = Quat::from_euler(EulerRot::YXZ, flycam.yaw, 0.0, 0.0);
        let forwards = rotation * Vec3::NEG_Z;
        let right = rotation * Vec3::X;
        let up = Vec3::Y;
        let mut movement = Vec3::ZERO;
        if keyboard.pressed(KeyCode::KeyA) {
            movement -= right * flycam.speed * secs;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            movement += right * flycam.speed * secs;
        }
        if keyboard.pressed(KeyCode::KeyW) {
            movement += forwards * flycam.speed * secs;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            movement -= forwards * flycam.speed * secs;
        }
        if keyboard.pressed(KeyCode::KeyE) {
            movement += up * flycam.speed * secs;
        }
        if keyboard.pressed(KeyCode::KeyQ) {
            movement -= up * flycam.speed * secs;
        }
        transform.translation += movement;
    }
}