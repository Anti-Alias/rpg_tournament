mod screen;
mod task;

use bevy::prelude::*;
use screen::screen_plugin;
use task::TaskPlugin;


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            TaskPlugin::new(Update),
            screen_plugin,
        ))
        .add_systems(Startup, startup)
        .run();
}


fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

