mod screen;
mod sprite;
mod animation;
mod asset;
mod task;
mod ui;
mod dsl;
mod spawn;
mod ext;

use std::f32::consts::{PI, SQRT_2};

use animation::animation_plugin;
use asset::asset_extension_plugin;
use bevy::core_pipeline::core_3d::ScreenSpaceTransmissionQuality;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::render::camera::ScalingMode;
use bevy::window::WindowResolution;
#[cfg(feature="inspector")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use ext::CommandsExt;
use screen::{screen_plugin, FadeInitial, Keep};
use sprite::sprite_plugin;
use task::task_plugin;
use ui::ui_plugin;

use bevy::prelude::*;
use bevy::render::texture::ImageSamplerDescriptor;

const RES_WIDTH: f32 = 960.0;
const RES_HEIGHT: f32 = 540.0;
const PIXEL_SCALE: f32 = 2.0;

fn main() {
    let default_plugins = DefaultPlugins
        .set(ImagePlugin { default_sampler: ImageSamplerDescriptor::nearest() })
        .set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(RES_WIDTH, RES_HEIGHT),
                ..default()
            }),
            ..default()
        });

    App::new()
        .add_plugins((
            default_plugins,
            task_plugin,
            sprite_plugin,
            animation_plugin,
            screen_plugin,
            ui_plugin,
            asset_extension_plugin,
            #[cfg(feature="inspector")]
            WorldInspectorPlugin::new(),
        ))
        .add_systems(Startup, start_game)
        .init_state::<GameState>()
        .run();
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum GameState {
    /// Regular game state.
    #[default]
    Running,
    /// Transitioning between screens.
    Transitioning,
}


fn start_game(mut commands: Commands, mut scale: ResMut<UiScale>, mut ambient: ResMut<AmbientLight>) {
    scale.0 = 2.0;
    ambient.brightness *= 8.0;

    let mut camera_2d = Camera2dBundle::default();
    camera_2d.camera.order = 1;
    camera_2d.tonemapping = Tonemapping::None;
    commands.spawn((camera_2d, Keep));
    
    let mut camera_3d = Camera3dBundle::default();
    camera_3d.tonemapping = Tonemapping::None;
    camera_3d.camera_3d.screen_space_specular_transmission_steps = 0;
    camera_3d.camera_3d.screen_space_specular_transmission_quality = ScreenSpaceTransmissionQuality::Low;
    camera_3d.transform.translation = Vec3::new(0.0, 128.0, 128.0);
    camera_3d.transform.rotate_x(-PI / 4.0);
    camera_3d.projection = Projection::Orthographic(OrthographicProjection {
        near: 1.0,
        far: 2048.0,
        scaling_mode: ScalingMode::Fixed {
            width: RES_WIDTH,
            height: RES_HEIGHT / SQRT_2,
        },
        scale: 1.0 / PIXEL_SCALE,
        ..default()
    });
    commands.spawn((camera_3d, Keep));

    commands.spawn_task(FadeInitial);
}