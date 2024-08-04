use std::f32::consts::PI;
use std::time::Duration;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use light_consts::lux::AMBIENT_DAYLIGHT;
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
    commands.spawn((dir_light, Sunlight::default()));

    let mut camera = GameCameraBundle::default();
    camera.projection.t = 0.0;
    camera.projection.perspective = PerspectiveProjection { near: 16.0, ..default() };
    camera.projection.orthographic.scale = 0.5;
    camera.transform = Transform::from_xyz(128.0, 256.0, 256.0).looking_to(Vec3::new(0.0, -1.0, -1.0), Vec3::Y);
    camera.tonemapping = Tonemapping::None;
    commands.spawn((camera, Flycam::default(), Round));
}

const TIME_NOON: f32        = 0.5;
const TIME_MORNING: f32     = 0.25;
const TIME_NIGHT: f32       = 0.75;

pub fn update_game_time(
    mut game_time: ResMut<GameTime>,
    time: Res<Time>,
    mut sunlights: Query<(&mut DirectionalLight, &Sunlight, &mut Transform)>,
) {

    // Updates game time
    let game_time = &mut *game_time;
    game_time.time_of_day += time.delta();
    while game_time.time_of_day > game_time.day_duration {
        game_time.time_of_day -= game_time.day_duration;
        game_time.days_elapsed += 1;
    }

    // Updates sunlight
    let time_ms = game_time.time_of_day.as_millis() as i64;
    let day_length_ms = game_time.day_duration.as_millis();
    let t = time_ms as f32 / day_length_ms as f32;

    let sun_bright = sun_brightness(t);
    let sun_rot_y = sun_rotation_y(t, -0.9, 0.9);
    let sun_rot = Quat::from_euler(EulerRot::YXZ, sun_rot_y, -1.0, 0.0);

    for (mut dir_light, sunlight, mut transf) in &mut sunlights {
        dir_light.color = sunlight.color;
        dir_light.illuminance = sunlight.illuminance * sun_bright;
        *transf = Transform::from_rotation(sun_rot);
    }
}

fn sun_brightness(t: f32) -> f32 {
    let segment = CubicSegment::new_bezier(Vec2::new(0.3, 1.085), Vec2::new(0.08, 0.935));
    if t > TIME_MORNING && t < TIME_NIGHT {
        let t_linear = 1.0 - 4.0 * (TIME_NOON - t).abs();
        segment.ease(t_linear)
    }
    else {
        0.0
    }
}

fn sun_rotation_y(t: f32, start_rot: f32, end_rot: f32) -> f32 {
    if t > TIME_MORNING && t < TIME_NIGHT {
        let t = (t - TIME_MORNING) / (TIME_NIGHT - TIME_MORNING);
        start_rot + (end_rot - start_rot) * t
    }
    else {
        0.0
    }
}


#[derive(Resource, Copy, Clone, Eq, PartialEq, Debug)]
pub struct GameTime {
    pub time_of_day: Duration,
    pub day_duration: Duration,
    pub days_elapsed: u32,
}

impl Default for GameTime {
    fn default() -> Self {
        Self {
            time_of_day: Duration::from_secs_f32(2.5),
            day_duration: Duration::from_secs_f32(10.0),
            days_elapsed: 1,
        }
    }
}


#[derive(Component, Copy, Clone, PartialEq, Debug)]
pub struct Sunlight {
    pub illuminance: f32,
    pub color: Color,
}

impl Default for Sunlight {
    fn default() -> Self {
        Self {
            illuminance: AMBIENT_DAYLIGHT / 4.0,
            color: Color::WHITE,
        }
    }
}

pub mod messages {

    use bevy::prelude::*;

    #[derive(Event, Copy, Clone, Eq, PartialEq, Default, Debug)]
    pub struct InitOverworld;
}