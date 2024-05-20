mod screen;
mod sprite;
mod animation;
mod asset;
mod task;
mod ui;
mod dsl;
mod spawn;
mod ext;

use std::time::Duration;

use animation::animation_plugin;
use asset::asset_extension_plugin;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::window::WindowResolution;
#[cfg(feature="inspector")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use ext::CommandsExt;
use screen::{screen_plugin, FadeInitial, Keep, ScreenEvent};
use sprite::sprite_plugin;
use task::{task_plugin, Start, Task, TaskQueue};
use ui::ui_plugin;

use bevy::prelude::*;
use bevy::render::texture::ImageSamplerDescriptor;

fn main() {
    let default_plugins = DefaultPlugins
        .set(ImagePlugin { default_sampler: ImageSamplerDescriptor::nearest() })
        .set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(960.0, 540.0),
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


fn start_game(mut commands: Commands, mut scale: ResMut<UiScale>) {
    scale.0 = 2.0;

    let mut camera_2d = Camera2dBundle::default();
    camera_2d.camera.order = 1;
    camera_2d.tonemapping = Tonemapping::None;
    commands.spawn((camera_2d, Keep));
    
    let mut camera_3d = Camera3dBundle::default();
    camera_3d.transform.translation = Vec3::new(0.0, 100.0, 100.0);
    camera_3d.transform.look_at(Vec3::ZERO, Vec3::Y);
    camera_3d.tonemapping = Tonemapping::None;
    commands.spawn((camera_3d, Keep));

    commands.spawn_task(FadeInitial);
}