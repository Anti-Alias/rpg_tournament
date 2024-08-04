use std::f32::consts::PI;
use std::time::Duration;
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
    commands.spawn((camera, Flycam::default(), Round));
}

pub fn update_game_time(
    mut game_time: ResMut<GameTime>,
    time: Res<Time>,
    mut sunlights: Query<(&mut DirectionalLight, &Sunlight, &mut Transform)>,
) {
    let game_time = &mut *game_time;
    game_time.time_of_day += time.delta();
    while game_time.time_of_day > game_time.day_duration {
        game_time.time_of_day -= game_time.day_duration;
        game_time.days_elapsed += 1;
    }

    let time_of_day = game_time.time_of_day.as_millis() as i64;
    let half_day = game_time.day_duration.as_millis() as i64 / 2;
    let quarter_day = game_time.day_duration.as_millis() as i64 / 4;
    let t = 1.0 - (half_day - time_of_day) as f32 / quarter_day as f32;
    let t_illuminance = if t < 0.0 || t > 2.0 { 0.0 } else if t > 1.0 { 2.0 - t } else { t };
    let rotation = Quat::from_euler(
        EulerRot::YXZ,
        -PI/2.0 + PI/2.0 * t,
        -1.0,
        0.0,
    );
    for (mut dir_light, sunlight, mut transf) in &mut sunlights {
        dir_light.color = sunlight.color;
        dir_light.illuminance = sunlight.illuminance * t_illuminance;
        *transf = Transform::from_rotation(rotation);
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
            time_of_day: Duration::ZERO,
            day_duration: Duration::from_secs(10),
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
            illuminance: AMBIENT_DAYLIGHT / 2.0,
            color: Color::WHITE,
        }
    }
}

pub mod messages {

    use bevy::prelude::*;

    #[derive(Event, Copy, Clone, Eq, PartialEq, Default, Debug)]
    pub struct InitOverworld;
}