mod screen;
mod task;
mod ui;
mod dsl;
mod batch;
mod ext;

use bevy::ui::UiPlugin;
use bevy::window::WindowResolution;
use screen::screen_plugin;
use task::TaskPlugin;
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
