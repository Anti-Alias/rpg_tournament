use bevy::prelude::*;
use rpg_tournament::GamePlugin;
use rpg_tournament::messages::{InitOverworld, SpawnMap};

fn main() {
    App::new()
        .add_plugins(GamePlugin)
        .add_systems(Startup, startup)
        .run();
}


fn startup(mut commands: Commands) {
    commands.trigger(InitOverworld);
    commands.trigger(SpawnMap {
        name: "Map",
        file: "maps/test_map_2.tmx",
        position: Vec3::ZERO
    });
}