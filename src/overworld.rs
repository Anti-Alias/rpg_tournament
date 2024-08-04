use std::f32::consts::PI;
use bevy::prelude::*;
use crate::camera::{Flycam, GameCameraBundle};
use crate::pixel::Round;


pub fn init_overworld(
    _trigger: Trigger<messages::InitOverworld>,
    mut commands: Commands,
) {
    let mut dir_light = DirectionalLightBundle::default();
    dir_light.directional_light.shadows_enabled = true;
    dir_light.directional_light.illuminance *= 0.5;
    
    dir_light.transform.rotate(Quat::from_euler(EulerRot::YXZ, PI/4.0, -PI/4.0, 0.0));
    commands.spawn(dir_light);

    let mut camera = GameCameraBundle::default();
    camera.projection.t = 0.0;
    camera.projection.perspective = PerspectiveProjection { near: 16.0, ..default() };
    camera.projection.orthographic.scale = 0.5;
    camera.transform = Transform::from_xyz(128.0, 256.0, 256.0).looking_to(Vec3::new(0.0, -1.0, -1.0), Vec3::Y);
    commands.spawn((camera, Flycam::default(), Round));
}

pub mod messages {

    use bevy::prelude::*;

    #[derive(Event, Copy, Clone, Eq, PartialEq, Default, Debug)]
    pub struct InitOverworld;
}