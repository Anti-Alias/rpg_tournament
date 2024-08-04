use std::f32::consts::PI;
use bevy::prelude::*;
use crate::camera::Flycam;


pub fn init_overworld(
    _trigger: Trigger<messages::InitOverworld>,
    mut commands: Commands,
) {
    let mut dir_light = DirectionalLightBundle::default();
    dir_light.directional_light.shadows_enabled = true;
    dir_light.directional_light.illuminance *= 0.5;
    dir_light.transform.rotate(Quat::from_euler(EulerRot::YXZ, PI/4.0, -PI/4.0, 0.0));
    commands.spawn(dir_light);

    let mut camera = Camera3dBundle::default();
    camera.projection = Projection::Perspective(PerspectiveProjection { near: 16.0, ..default() });
    camera.transform = Transform::from_xyz(128.0, 256.0, 256.0).looking_to(Vec3::new(0.0, -1.0, -1.0), Vec3::Y);
    commands.spawn((camera, Flycam::default()));
}

pub mod messages {

    use bevy::prelude::*;

    #[derive(Event, Copy, Clone, Eq, PartialEq, Default, Debug)]
    pub struct InitOverworld;
}