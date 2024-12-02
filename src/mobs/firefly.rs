use rand::prelude::*;
use std::f32::consts::{SQRT_2, TAU};
use std::time::Duration;
use bevy::prelude::*;
use crate::area::AreaLocal;
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

impl Default for Firefly {
    fn default() -> Self {
        Self {
            home: default(),
            timer: default(),
            behavior: default(),
            fly_radius: default(),
            scale: default(),
            sphere_id: Entity::PLACEHOLDER,
        }
    }
}

/// Current behavior of a [`Firefly`]
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub enum FireflyBehavior {
    #[default]
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
    // Firefly settings based on the time of day
    let mut visibility = Visibility::Visible;
    let mut behavior = FireflyBehavior::Flying; 
    let mut scale = 1.0;
    if time_frac >= TIME_FRAC_MORNING && time_frac < TIME_FRAC_NIGHT {
        visibility = Visibility::Hidden;
        behavior = FireflyBehavior::Hidden;
        scale = 0.0;
    }

    // Spawns child sphere 
    let sphere = commands.spawn((
       Mesh3d(common_assets.meshes.sphere.clone()),
       MeshMaterial3d(common_assets.materials.white.clone()),
       Transform::from_scale(FIREFLY_BODY_SIZE),
    )).id();

    // Spawns firefly
    commands
        .spawn((
            Name::new("firefly"),
            AreaLocal::default(),
            Transform::from_translation(position),
            visibility,
            Firefly { behavior, scale, ..default() },
            PointLight {
                color: Color::linear_rgb(1.0, 1.0, 0.5),
                intensity: FIREFLY_LIGHT_INTENSITY,
                range: 64.0,
                ..default()
            },
        ))
        .with_child(sphere);
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
                firefly.scale -= 1.0 * time.delta_secs();
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
                firefly.scale += 1.0 * time.delta_secs();
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
