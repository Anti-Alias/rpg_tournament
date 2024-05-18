mod screen;
mod sprite;
mod task;
mod ui;
mod dsl;
mod spawn;
mod ext;

use bevy::window::WindowResolution;
#[cfg(feature="inspector")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use screen::{screen_plugin, Keep};
use sprite::sprite_plugin;
use task::task_plugin;
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
            screen_plugin,
            ui_plugin,
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
    let mut camera = Camera2dBundle::default();
    camera.camera.order = 1;
    commands.spawn(( camera, Keep));
}