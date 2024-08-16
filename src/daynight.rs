use std::time::Duration;
use light_consts::lux::AMBIENT_DAYLIGHT;
use bevy::prelude::*;

// Times of the day as fractions
pub const TIME_FRAC_NOON: f32       = 0.5;
pub const TIME_FRAC_MORNING: f32    = 0.25;
pub const TIME_FRAC_NIGHT: f32      = 0.75;

// Light constants
const AMBIENT_MIN_BRIGHTNESS: f32   = 100.0;
const AMBIENT_BASE_BRIGHTNESS: f32  = 400.0;
const AMBIENT_DAY_COLOR: Color      = Color::WHITE;
const AMBIENT_NIGHT_COLOR: Color    = Color::linear_rgb(0.4, 0.4, 1.0);

pub fn update_game_time(
    mut game_time: ResMut<GameTime>,
    time: Res<Time>,
    mut ambient: ResMut<AmbientLight>,
    mut sunlights: Query<(&mut DirectionalLight, &Sunlight, &mut Transform)>,
) {
    // Updates game time
    let game_time = &mut *game_time;
    game_time.prev_elapsed = game_time.elapsed;
    game_time.elapsed += time.delta();
    let time_frac = game_time.time_fraction();

    // Manipulates ambient light using time fraction
    let amb_bright = ambient_brightness(time_frac);
    ambient.color = AMBIENT_NIGHT_COLOR.mix(&AMBIENT_DAY_COLOR, amb_bright);
    ambient.brightness = AMBIENT_MIN_BRIGHTNESS + AMBIENT_BASE_BRIGHTNESS * amb_bright;

    // Manipulates sun using time fraction
    let sun_bright = sun_brightness(time_frac);
    let sun_rot_y = sun_rotation_y(time_frac, -0.9, 0.9);
    let sun_rot_y = (sun_rot_y * 300.0).round() / 300.0;
    let sun_rot = Quat::from_euler(EulerRot::YXZ, sun_rot_y, -1.0, 0.0);
    for (mut dir_light, sunlight, mut transf) in &mut sunlights {
        dir_light.color = AMBIENT_NIGHT_COLOR.mix(&AMBIENT_DAY_COLOR, amb_bright);
        dir_light.illuminance = sunlight.illuminance * sun_bright;
        *transf = Transform::from_rotation(sun_rot);
    }
}

fn sun_brightness(time_frac: f32) -> f32 {
    let segment = CubicSegment::new_bezier(Vec2::new(0.3, 1.085), Vec2::new(0.08, 0.935));
    if time_frac >= TIME_FRAC_MORNING && time_frac < TIME_FRAC_NIGHT {
        let t_linear = 1.0 - 4.0 * (TIME_FRAC_NOON - time_frac).abs();
        segment.ease(t_linear)
    }
    else {
        let t_linear = if time_frac >= TIME_FRAC_NIGHT { time_frac - TIME_FRAC_NIGHT } else { TIME_FRAC_MORNING - time_frac } * 4.0;
        segment.ease(t_linear) * 0.2
    }
}

fn ambient_brightness(time_frac: f32) -> f32 {
    let segment = CubicSegment::new_bezier(Vec2::new(0.3, 1.085), Vec2::new(0.08, 0.935));
    if time_frac >= TIME_FRAC_MORNING && time_frac < TIME_FRAC_NIGHT {
        let t = 1.0 - 4.0 * (TIME_FRAC_NOON - time_frac).abs();
        segment.ease(t)
    }
    else {
        0.0
    }
}

fn sun_rotation_y(time_frac: f32, start_rot: f32, end_rot: f32) -> f32 {
    if time_frac >= TIME_FRAC_MORNING && time_frac < TIME_FRAC_NIGHT {
        let t = (time_frac - TIME_FRAC_MORNING) / (TIME_FRAC_NIGHT - TIME_FRAC_MORNING);
        start_rot + (end_rot - start_rot) * t
    }
    else {
        let t = if time_frac >= TIME_FRAC_NIGHT { time_frac - TIME_FRAC_NIGHT } else { time_frac + 0.25 } * 2.0;
        start_rot + (end_rot - start_rot) * t
    }
}


/// Used to track the time of day as a fraction.
#[derive(Resource, Reflect, Copy, Clone, PartialEq, Debug)]
#[reflect(Resource)]
pub struct GameTime {
    t_offset: f32,              // Out of 100 for easier debugging.
    elapsed: Duration,
    prev_elapsed: Duration,
    day_duration: Duration,
}

impl GameTime {
    
    /// Time of day represented as a number that cycles between 0.0 and 1.0.
    /// 0.0 = midnight, 0.5 = noon, 1.0 = midnight.
    pub fn time_fraction(&self) -> f32 {
        self.time_fraction_of(self.elapsed)
    }

    /// If the current time of day just passed a particular time fraction this frame.
    /// Useful for triggering actions when morning starts, nights starts etc.
    pub fn time_just_passed(&self, time_fraction: f32) -> bool {
        let time_frac = self.time_fraction_of(self.elapsed);
        let time_frac_prev = self.time_fraction_of(self.prev_elapsed);
        time_frac >= time_fraction && time_frac_prev < time_fraction
    }

    fn time_fraction_of(&self, elapsed: Duration) -> f32 {
        let elapsed = elapsed.as_millis();
        let day_duration = self.day_duration.as_millis();
        let day_elapsed = elapsed % day_duration;
        let t = day_elapsed as f32 / day_duration as f32;
        (t + self.t_offset / 100.0).rem_euclid(1.0)
    }
}

impl Default for GameTime {
    fn default() -> Self {
        let day_duration = Duration::from_secs(60 * 20);
        let elapsed = day_duration / 2;
        Self {
            elapsed,
            prev_elapsed: elapsed,
            day_duration,
            t_offset: 0.0,
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