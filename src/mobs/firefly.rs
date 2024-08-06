use rand::prelude::*;
use std::f32::consts::{SQRT_2, TAU};
use std::time::Duration;
use bevy::prelude::*;
use crate::common::CommonAssets;
use crate::daynight::{GameTime, TIME_FRAC_MORNING, TIME_FRAC_NIGHT};

const FIREFLY_BODY_SIZE: Vec3 = Vec3::new(1.2, 1.2/SQRT_2, 1.2/SQRT_2);
const FIREFLY_LIGHT_INTENSITY: f32 = 20_000_000.0;
const FIREFLY_MOVE_HEIGHT: f32 = 3.0;

#[derive(Component)]
pub struct Firefly {
    home: Vec3,
    timer: Timer,
    behavior: FireflyBehavior,
    fly_radius: f32,
    scale: f32,
    sphere_id: Entity,
}

impl Firefly {
    pub fn new(sphere_id: Entity, home: Vec3) -> Self {
        let mut rng = rand::thread_rng();
        let cycle_duration: f32 = rng.gen_range(4.0..6.0);
        let cycle_start: f32 = rng.gen_range(0.0..cycle_duration);
        let radius = rng.gen_range(8.0..12.0);
        let mut timer = Timer::new(Duration::from_secs_f32(cycle_duration), TimerMode::Repeating);
        timer.set_elapsed(Duration::from_secs_f32(cycle_start));
        Self {
            home,
            timer,
            behavior: FireflyBehavior::Flying,
            fly_radius: radius,
            scale: 1.0,
            sphere_id
        }
    }
}

/// Current behavior of a [`Firefly`]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum FireflyBehavior {
    Flying,     // Regular flying. Triggered when revealing finishes.
    Hiding,     // Flying + shrinking. Triggered when night comes.
    Hidden,     // Hidden and stationary. Triggered when hiding finishes.
    Revealing,  // Flying + growing. Triggered when morning comes.
}

pub fn spawn_firefly(
    commands: &mut Commands,
    position: Vec3,
    common_assets: &CommonAssets,
    time_frac: f32,
) {
    // Root light
    let mut light = PointLightBundle::default();
    let light_color = Color::linear_rgb(0.2, 1.0, 1.0);
    light.point_light.color = light_color;
    light.point_light.intensity = FIREFLY_LIGHT_INTENSITY;
    light.point_light.range = 64.0;
    light.transform = Transform::from_translation(position);
    // Child sphere
    let mut sphere = PbrBundle::default();
    sphere.material = common_assets.materials.white.clone();
    sphere.mesh = common_assets.meshes.sphere.clone();
    sphere.transform.scale = FIREFLY_BODY_SIZE;
    let sphere_id = commands.spawn(sphere).id();
    // Firefly
    let mut firefly = Firefly::new(sphere_id, position);
    if time_frac >= TIME_FRAC_MORNING && time_frac < TIME_FRAC_NIGHT {
        firefly.behavior = FireflyBehavior::Hidden;
        firefly.scale = 0.0;
        light.visibility = Visibility::Hidden;
    }
    commands
        .spawn((firefly, light))
        .add_child(sphere_id);
}


/// Updates all firefly entities each frame.
pub fn update_fireflies(
    mut fireflies: Query<(&mut Firefly, &mut PointLight, &mut Transform, &mut Visibility)>,
    mut firefly_bodies: Query<&mut Transform, Without<Firefly>>,
    game_time: Res<GameTime>,
    time: Res<Time>,
) {

    if game_time.time_just_passed(TIME_FRAC_MORNING) {
        for (mut firefly, _, _, _) in &mut fireflies {
            firefly.behavior = FireflyBehavior::Hiding;
        }
    }
    else if game_time.time_just_passed(TIME_FRAC_NIGHT) {
        for (mut firefly, _, _, mut visibility) in &mut fireflies {
            firefly.behavior = FireflyBehavior::Revealing;
            *visibility = Visibility::Inherited;
        }
    }

    // Controls non-hidden fireflies.
    for (mut firefly, _, mut transf, _) in &mut fireflies {
        if firefly.behavior == FireflyBehavior::Hidden { continue };
        firefly.timer.tick(time.delta());
        let radians = TAU * firefly.timer.fraction();
        let offset = Vec3::new(radians.cos(), (radians * 3.0).sin(), radians.sin());
        let scale = Vec3::new(firefly.fly_radius, FIREFLY_MOVE_HEIGHT, firefly.fly_radius);
        transf.translation = firefly.home + offset * scale;
    }

    // Controls fireflies that are hiding or revealing.
    for (mut firefly, mut light, _, mut visibility) in &mut fireflies {
        match firefly.behavior {
            FireflyBehavior::Hiding => {
                firefly.scale -= 1.0 * time.delta_seconds();
                firefly.scale = firefly.scale.max(0.0);
                light.intensity = FIREFLY_LIGHT_INTENSITY * firefly.scale;
                let mut body_transf = firefly_bodies.get_mut(firefly.sphere_id).unwrap();
                body_transf.scale = FIREFLY_BODY_SIZE * firefly.scale;
                if firefly.scale == 0.0 {
                    firefly.behavior = FireflyBehavior::Hidden;
                    *visibility = Visibility::Hidden;
                }
            },
            FireflyBehavior::Revealing => {
                firefly.scale += 1.0 * time.delta_seconds();
                firefly.scale = firefly.scale.min(1.0);
                light.intensity = FIREFLY_LIGHT_INTENSITY * firefly.scale;
                let mut body_transf = firefly_bodies.get_mut(firefly.sphere_id).unwrap();
                body_transf.scale = FIREFLY_BODY_SIZE * firefly.scale;
                if firefly.scale == 1.0 {
                    firefly.behavior = FireflyBehavior::Flying;
                }
            },
            _ => {}
        }
    }
}