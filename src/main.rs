use bevy::prelude::*;
use rpg_tournament::{ActionKind, GamePlugin, RunAction, SpawnMapMsg};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            GamePlugin
        ))
        .add_systems(Startup, startup)
        .run();
}


fn startup(mut commands: Commands) {
    commands.trigger(RunAction(ActionKind::Cutscene));
    commands.trigger(SpawnMapMsg {
        name: "Map",
        file: "maps/map.tmx",
        position: Vec3::ZERO
    });
}