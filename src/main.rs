use bevy::prelude::*;
use rpg_tournament::{GameEvent, GamePlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, GamePlugin))
        .add_systems(Startup, startup)
        .run();
}


fn startup(mut events: EventWriter<GameEvent>) {
    events.send(GameEvent::SpawnMap { name: "Map", file: "maps/map.tmx", position: Vec3::ZERO });
}