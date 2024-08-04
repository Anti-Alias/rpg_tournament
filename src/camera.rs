use std::f32::consts::{PI, SQRT_2};

use bevy::core_pipeline::core_3d::graph::Core3d;
use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::math::Vec3A;
use bevy::prelude::*;
use bevy::render::camera::{CameraMainTextureUsages, CameraProjection, CameraRenderGraph, Exposure};
use bevy::render::primitives::Frustum;
use bevy::render::view::{ColorGrading, VisibleEntities};


/// Copied from bevy's Camera3dBundle.
/// TODO: Update when bevy updates.
#[derive(Bundle, Clone)]
pub struct GameCameraBundle {
    pub camera: Camera,
    pub camera_render_graph: CameraRenderGraph,
    pub projection: DualProjection,
    pub visible_entities: VisibleEntities,
    pub frustum: Frustum,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub camera_3d: Camera3d,
    pub tonemapping: Tonemapping,
    pub deband_dither: DebandDither,
    pub color_grading: ColorGrading,
    pub exposure: Exposure,
    pub main_texture_usages: CameraMainTextureUsages,
}

impl Default for GameCameraBundle {
    fn default() -> Self {
        Self {
            camera_render_graph: CameraRenderGraph::new(Core3d),
            camera: Default::default(),
            projection: Default::default(),
            visible_entities: Default::default(),
            frustum: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            camera_3d: Default::default(),
            tonemapping: Default::default(),
            color_grading: Default::default(),
            exposure: Default::default(),
            main_texture_usages: Default::default(),
            deband_dither: DebandDither::Enabled,
        }
    }
}


#[derive(Component)]
pub struct Flycam {
    pub speed: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub mouse_sensitivity: f32,
    pub wheel_sensitivity: f32,
}

impl Flycam {
    fn direction(&self) -> Vec3 {
        Quat::from_euler(EulerRot::YXZ, self.yaw, self.pitch, 0.0) * Vec3::NEG_Z
    }
}

impl Default for Flycam {
    fn default() -> Self {
        Self {
            speed: 256.0,
            yaw: 0.0,
            pitch: -PI/4.0,
            mouse_sensitivity: 0.005,
            wheel_sensitivity: 0.1,
        }
    }
}

#[derive(Component, Debug, Clone, Default, Reflect)]
#[reflect(Component, Default)]
pub struct DualProjection {
    pub orthographic: OrthographicProjection,
    pub perspective: PerspectiveProjection,
    pub t: f32,
}

const ORTHO_SCALE: Mat4 =  Mat4::from_cols_array(
    &[
        1.0, 0.0, 0.0, 0.0,
        0.0, SQRT_2, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ]
);

const ORTHO_SCALE_3: Mat3 =  Mat3::from_cols_array(
    &[
        1.0, 0.0, 0.0,
        0.0, SQRT_2, 0.0,
        0.0, 0.0, 1.0,
    ]
);

impl CameraProjection for DualProjection {    
    fn get_clip_from_view(&self) -> Mat4 {
        let ortho_clip = ORTHO_SCALE * self.orthographic.get_clip_from_view();
        let perspective_clip = self.perspective.get_clip_from_view();
        interp_mat4(ortho_clip, perspective_clip, self.t)
    }

    fn update(&mut self, width: f32, height: f32) {
        self.orthographic.update(width, height);
        self.perspective.update(width, height);
    }

    fn far(&self) -> f32 {
        let ortho_far = self.orthographic.far();
        let perspective_far = self.perspective.far();
        ortho_far + (perspective_far - ortho_far) * self.t
    }

    fn get_frustum_corners(&self, z_near: f32, z_far: f32) -> [Vec3A; 8] {
        let ortho_corners = self.orthographic.get_frustum_corners(z_near, z_far);
        let perspective_corners = self.perspective.get_frustum_corners(z_near, z_far);
        let mut result = [Vec3A::ZERO; 8];
        for i in 0..8 {
            let ortho_corner = ORTHO_SCALE_3 * Vec3::from(ortho_corners[i]);
            let persp_corner = Vec3::from(perspective_corners[i]);
            let corner = ortho_corner + (persp_corner - ortho_corner) * self.t;
            result[i] = Vec3A::from(corner);
        }
        result
    }
}

fn interp_mat4(a: Mat4, b: Mat4, t: f32) -> Mat4 {
    let segment = CubicSegment::new_bezier([1.0, 0.2], [0.6, 0.0]);
    let t = segment.ease(t);
    let t = segment.ease(t);
    let t = segment.ease(t);
    let a_cols = a.to_cols_array();
    let b_cols = b.to_cols_array();
    let mut result = [0.0; 16];
    for i in 0..a_cols.len() {
        result[i] = a_cols[i] + (b_cols[i] - a_cols[i]) * t;
    }
    Mat4::from_cols_array(&result)
}


pub fn control_flycam(
    mut flycams: Query<(&mut Transform, &mut Flycam, &mut DualProjection)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut mouse_motions: EventReader<MouseMotion>,
    mut mouse_wheels: EventReader<MouseWheel>,
    time: Res<Time>,
) {
    const EPS: f32 = 0.001;
    let secs = time.delta_seconds();
    for (mut transform, mut flycam, mut projection) in &mut flycams {

        // Rotates flycam
        if mouse.pressed(MouseButton::Middle) {
            for mouse_motion in mouse_motions.read() {
                flycam.yaw -= mouse_motion.delta.x * flycam.mouse_sensitivity;
                flycam.pitch -= mouse_motion.delta.y * flycam.mouse_sensitivity;
                flycam.pitch = flycam.pitch.min(PI/2.0 - EPS).max(-PI/2.0 + EPS);
                *transform = transform.looking_to(flycam.direction(), Vec3::Y);
            }
        }

        // Interpolates between orthographic and perspective
        for mouse_wheel in mouse_wheels.read() {
            projection.t += mouse_wheel.y * flycam.wheel_sensitivity;
            projection.t = projection.t.min(1.0).max(0.0);
        }

        // Resets camera rotation
        if keyboard.pressed(KeyCode::ShiftLeft) && keyboard.just_pressed(KeyCode::KeyR) {
            flycam.yaw = 0.0;
            flycam.pitch = -PI/4.0;
            *transform = transform.looking_to(flycam.direction(), Vec3::Y);
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