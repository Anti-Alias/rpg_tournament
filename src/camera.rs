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
            speed: 0.1,
            yaw: 0.0,
            pitch: 0.0,
            sensitivity: 0.01,
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

    // Moves flycams
    let secs = time.elapsed_seconds();
    for (mut transform, mut flycam) in &mut flycams {
        if mouse.pressed(MouseButton::Middle) {
            for mouse_motion in mouse_motions.read() {
                flycam.yaw -= mouse_motion.delta.x * flycam.sensitivity;
                flycam.pitch -= mouse_motion.delta.y * flycam.sensitivity;
                flycam.pitch = flycam.pitch.min(PI/2.0 - EPS).max(-PI/2.0 + EPS);
            }
        }
        let rot = Quat::from_euler(EulerRot::YXZ, flycam.yaw, flycam.pitch, 0.0);
        let forwards_look = rot * Vec3::NEG_Z;

        let move_rot = Quat::from_euler(EulerRot::YXZ, flycam.yaw, 0.0, 0.0);
        let forwards_move = move_rot * Vec3::NEG_Z;
        let right_move = move_rot * Vec3::X;
        let up_move = Vec3::Y;

        let mut movement = Vec3::ZERO;
        if keyboard.pressed(KeyCode::KeyA) {
            movement -= right_move * flycam.speed * secs;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            movement += right_move * flycam.speed * secs;
        }
        if keyboard.pressed(KeyCode::KeyW) {
            movement += forwards_move * flycam.speed * secs;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            movement -= forwards_move * flycam.speed * secs;
        }
        if keyboard.pressed(KeyCode::KeyE) {
            movement += up_move * flycam.speed * secs;
        }
        if keyboard.pressed(KeyCode::KeyQ) {
            movement -= up_move * flycam.speed * secs;
        }
        transform.translation += movement;
        *transform = transform.looking_to(forwards_look, Vec3::Y);
    }
}