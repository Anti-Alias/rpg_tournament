use bevy::prelude::*;
use rpg_tournament::{ActionKind, GamePlugin};
use rpg_tournament::messages::{InitOverworld, SpawnMap, RunAction};

fn main() {
    App::new()
        .add_plugins(GamePlugin)
        .add_systems(Startup, startup)
        .run();
}


fn startup(mut commands: Commands) {
    commands.trigger(RunAction(ActionKind::Cutscene));
    commands.trigger(InitOverworld);
    commands.trigger(SpawnMap {
        name: "Map",
        file: "maps/small_map.tmx",
        position: Vec3::ZERO
    });
}