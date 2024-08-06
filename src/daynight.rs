use std::time::Duration;
use light_consts::lux::AMBIENT_DAYLIGHT;
use bevy::prelude::*;

pub const TIME_FRAC_NOON: f32       = 0.5;
pub const TIME_FRAC_MORNING: f32    = 0.25;
pub const TIME_FRAC_NIGHT: f32      = 0.75;

pub fn update_game_time(
    mut game_time: ResMut<GameTime>,
    time: Res<Time>,
    mut sunlights: Query<(&mut DirectionalLight, &Sunlight, &mut Transform)>,
) {

    // Updates game time
    let game_time = &mut *game_time;
    game_time.time_of_day_prev = game_time.time_of_day;
    game_time.time_of_day += time.delta();
    while game_time.time_of_day > game_time.day_duration {
        game_time.time_of_day -= game_time.day_duration;
        game_time.days_elapsed += 1;
    }

    // Manipulates sun
    let time_frac = game_time.time_fraction();
    let sun_bright = sun_brightness(time_frac);
    let sun_rot_y = sun_rotation_y(time_frac, -0.9, 0.9);
    let sun_rot = Quat::from_euler(EulerRot::YXZ, sun_rot_y, -1.0, 0.0);
    for (mut dir_light, sunlight, mut transf) in &mut sunlights {
        dir_light.color = sunlight.color;
        dir_light.illuminance = sunlight.illuminance * sun_bright;
        *transf = Transform::from_rotation(sun_rot);
    }
}

fn sun_brightness(t: f32) -> f32 {
    let segment = CubicSegment::new_bezier(Vec2::new(0.3, 1.085), Vec2::new(0.08, 0.935));
    if t >= TIME_FRAC_MORNING && t < TIME_FRAC_NIGHT {
        let t_linear = 1.0 - 4.0 * (TIME_FRAC_NOON - t).abs();
        segment.ease(t_linear)
    }
    else {
        let t_linear = if t >= TIME_FRAC_NIGHT { t - TIME_FRAC_NIGHT } else { TIME_FRAC_MORNING - t } * 4.0;
        segment.ease(t_linear) * 0.2
    }
}

fn sun_rotation_y(t: f32, start_rot: f32, end_rot: f32) -> f32 {
    /// Morning hours
    if t >= TIME_FRAC_MORNING && t < TIME_FRAC_NIGHT {
        let t = (t - TIME_FRAC_MORNING) / (TIME_FRAC_NIGHT - TIME_FRAC_MORNING);
        start_rot + (end_rot - start_rot) * t
    }

    // Night hours
    else {
        let t = if t >= TIME_FRAC_NIGHT { t - TIME_FRAC_NIGHT } else { t + 0.25 } * 2.0;
        start_rot + (end_rot - start_rot) * t
    }
}


#[derive(Resource, Copy, Clone, Eq, PartialEq, Debug)]
pub struct GameTime {
    pub time_of_day: Duration,
    pub time_of_day_prev: Duration,
    pub day_duration: Duration,
    pub days_elapsed: u32,
}

impl GameTime {
    /// Time of day represented as a number that cycles between 0.0 and 1.0.
    /// 0.0 = midnight.
    pub fn time_fraction(&self) -> f32 {
        let time_of_day = self.time_of_day.as_nanos() as i64;
        let day_length = self.day_duration.as_nanos();
        time_of_day as f32 / day_length as f32
    }

    /// Time of day represented as a number that cycles between 0.0 and 1.0.
    /// 0.0 = midnight.
    /// Previous frame
    pub fn time_fraction_prev(&self) -> f32 {
        let time_of_day = self.time_of_day_prev.as_nanos() as i64;
        let day_length = self.day_duration.as_nanos();
        time_of_day as f32 / day_length as f32
    }

    pub fn time_just_passed(&self, time_fraction: f32) -> bool {
        let time_frac = self.time_fraction();
        let time_frac_prev = self.time_fraction_prev();
        time_frac >= time_fraction && time_frac_prev < time_fraction
    }
}

impl Default for GameTime {
    fn default() -> Self {
        let day_duration = Duration::from_secs(10);
        let time_of_day = day_duration * 4/8;
        Self {
            time_of_day,
            time_of_day_prev: time_of_day,
            day_duration,
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
            illuminance: AMBIENT_DAYLIGHT / 5.0,
            color: Color::WHITE,
        }
    }
}