use std::f32::consts::{PI, SQRT_2};
use bevy::prelude::*;
use bevy::core_pipeline::core_3d::graph::Core3d;
use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::input::mouse::MouseMotion;
use bevy::math::Vec3A;
use bevy::render::camera::{CameraMainTextureUsages, CameraProjection, CameraRenderGraph, Exposure};
use bevy::render::primitives::Frustum;
use bevy::render::view::{ColorGrading, VisibleEntities};
use crate::EntityIndex;


/// Copied from bevy's Camera3dBundle.
/// TODO: Update when bevy updates.
#[derive(Bundle, Clone)]
pub struct GameCameraBundle {
    pub game_camera: GameCamera,
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
        let mut camera = Self {
            game_camera: GameCamera::default(),
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
        };
        camera.color_grading.global.post_saturation = 1.1;
        camera.projection.perspective = PerspectiveProjection { near: 16.0, ..default() };
        camera.projection.orthographic.far = 10000.0;
        camera.projection.orthographic.scale = 0.5;
        camera.transform = Transform::from_xyz(128.0, 256.0, 256.0).looking_to(Vec3::new(0.0, -1.0, -1.0), Vec3::Y);
        camera.tonemapping = Tonemapping::None;
        camera
    }
}

/// Causes a camera to follow a target entity.
#[derive(Component, Clone, PartialEq, Debug)]
pub struct GameCamera {
    pub target: Option<CameraTarget>,
    pub offset: Vec3,
}

impl Default for GameCamera {
    fn default() -> Self {
        Self {
            target: Some(CameraTarget::Player),
            offset: Vec3::new(0.0, 256.0, 256.0),
        }
    }
}

/// Which target a [`GameCamera`] should follow.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CameraTarget {
    Player,
}


#[derive(Component)]
pub struct Flycam {
    pub speed: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub mouse_sensitivity: f32,
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
        }
    }
}


#[derive(Reflect, Copy, Clone, Eq, PartialEq, Default, Debug)]
pub enum ProjectionKind {
    #[default]
    Orthographic,
    Perspective,
}

impl ProjectionKind {
    pub fn toggle(self) -> Self {
        match self {
            Self::Orthographic => Self::Perspective,
            Self::Perspective => Self::Orthographic,
        }
    }
}

#[derive(Component, Debug, Clone, Default, Reflect)]
#[reflect(Component, Default)]
pub struct DualProjection {
    pub kind: ProjectionKind,
    pub orthographic: OrthographicProjection,
    pub perspective: PerspectiveProjection,
}

impl CameraProjection for DualProjection {    
    fn get_clip_from_view(&self) -> Mat4 {
        match self.kind {
            ProjectionKind::Orthographic => self.orthographic.get_clip_from_view() * Mat4::from_scale(Vec3::new(1.0, SQRT_2, 1.0)),
            ProjectionKind::Perspective => self.perspective.get_clip_from_view(),
        }
    }
    fn update(&mut self, width: f32, height: f32) {
        match self.kind {
            ProjectionKind::Orthographic => self.orthographic.update(width, height),
            ProjectionKind::Perspective => self.perspective.update(width, height),
        }
    }
    fn far(&self) -> f32 {
        match self.kind {
            ProjectionKind::Orthographic => self.orthographic.far(),
            ProjectionKind::Perspective => self.perspective.far(),
        }
    }
    fn get_frustum_corners(&self, z_near: f32, z_far: f32) -> [Vec3A; 8] {
        match self.kind {
            ProjectionKind::Orthographic => self.orthographic.get_frustum_corners(z_near, z_far),
            ProjectionKind::Perspective => self.perspective.get_frustum_corners(z_near, z_far),
        }
    }
}

pub fn update_game_camera(
    mut cameras: Query<(&mut GameCamera, &mut Transform), Without<Flycam>>,
    targets: Query<&Transform, Without<GameCamera>>,
    entity_index: Res<EntityIndex>,
) {
    for (mut game_camera, mut cam_transf) in &mut cameras {
        let target_entity = match game_camera.target {
            Some(CameraTarget::Player)          => entity_index.player,
            _                                   => None,
        };
        let Some(target_entity) = target_entity else { continue };
        let Ok(target_transf) = targets.get(target_entity) else {
            game_camera.target = None;
            continue;
        };
        cam_transf.translation = target_transf.translation + game_camera.offset;
        cam_transf.look_at(target_transf.translation, Vec3::Y);
    }
}


pub fn update_flycam(
    mut flycams: Query<(&mut Flycam, &mut Transform)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut mouse_motions: EventReader<MouseMotion>,
    time: Res<Time>,
) {
    const EPS: f32 = 0.001;
    let secs = time.delta_seconds();
    for (mut flycam, mut transform) in &mut flycams {

        // Rotates flycam
        if mouse.pressed(MouseButton::Middle) {
            for mouse_motion in mouse_motions.read() {
                flycam.yaw -= mouse_motion.delta.x * flycam.mouse_sensitivity;
                flycam.pitch -= mouse_motion.delta.y * flycam.mouse_sensitivity;
                flycam.pitch = flycam.pitch.min(PI/2.0 - EPS).max(-PI/2.0 + EPS);
                *transform = transform.looking_to(flycam.direction(), Vec3::Y);
            }
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

pub fn toggle_projection(
    mut cameras: Query<&mut DualProjection, With<GameCamera>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::KeyP) {
        for mut cam_proj in &mut cameras {
            cam_proj.kind = cam_proj.kind.toggle();
        }
    }
}

pub fn toggle_flycam(
    cameras: Query<
        (Entity, Option<&Flycam>),
        With<GameCamera>
    >,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    if keyboard.just_pressed(KeyCode::KeyF) {
        for (camera_id, flycam) in &cameras {
            if flycam.is_none() {
                commands.entity(camera_id).insert(Flycam::default());
            }
            else {
                commands.entity(camera_id).remove::<Flycam>();
            }
        }
    }
}

pub fn handle_disable_debug(
    mut commands: Commands,
    mut cameras: Query<(Entity, &mut DualProjection), With<GameCamera>>
) {
    for (cam_id, mut cam_proj) in &mut cameras {
        cam_proj.kind = ProjectionKind::default();
        commands.entity(cam_id).remove::<Flycam>();
    }
}