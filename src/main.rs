use bevy::prelude::*;
use rpg_tournament::GamePlugin;
use rpg_tournament::messages::*;

fn main() {
    App::new()
        .add_plugins(GamePlugin)
        .add_systems(Startup, startup)
        .run();
}


fn startup(mut commands: Commands) {
    commands.trigger(InitArea { name: "Overworld", file: "worlds/overworld.world" });
    commands.trigger(SpawnPlayer { position: Vec3::new(0.0, 32.0, 0.0) });
}