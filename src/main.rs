mod screen;
mod task;
mod ui;
mod dsl;

use screen::screen_plugin;
use task::TaskPlugin;
use ui::ui_plugin;

use bevy::prelude::*;
use bevy::render::texture::ImageSamplerDescriptor;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin { default_sampler: ImageSamplerDescriptor::nearest() }),
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
