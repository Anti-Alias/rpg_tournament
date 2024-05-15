mod screen;
mod task;
mod ui;
mod dsl;
mod ext;

use bevy::window::WindowResolution;
use screen::screen_plugin;
use task::TaskPlugin;
use ui::ui_plugin;

use bevy::prelude::*;
use bevy::render::texture::ImageSamplerDescriptor;

const RES_SCALE: f32 = 60.0;

fn main() {
    let default_plugins = DefaultPlugins
        .set(ImagePlugin { default_sampler: ImageSamplerDescriptor::nearest() })
        .set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(16.0*RES_SCALE, 9.0*RES_SCALE),
                ..default()
            }),
            ..default()
        });

    App::new()
        .add_plugins((
            default_plugins,
            TaskPlugin,
            screen_plugin,
            ui_plugin,
            game_plugin,
        ))
        .run();
}


fn game_plugin(app: &mut App) {
    app.init_state::<GameState>();
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum GameState {
    /// Regular game state.
    #[default]
    Running,
    /// Transitioning between screens.
    Transitioning,
}
