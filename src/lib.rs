mod map;

use bevy::utils::HashMap;
use map::{finish_spawning_maps, Map, MapData, MapLoader, Tileset};
use bevy::prelude::*;

/// Game engine plugin.
pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<Screen>();
        app.init_asset::<Map>();
        app.init_asset::<Tileset>();
        app.init_asset_loader::<MapLoader>();
        app.init_resource::<EntityIndex>();
        app.add_event::<GameEvent>();
        app.add_systems(Update, (
            handle_events,
            finish_spawning_maps.after(handle_events),
        ));
    }
}

// Game's main event handler.
// Responsible for executing high-level tasks.
fn handle_events(
    mut events: EventReader<GameEvent>,
    mut commands: Commands,
    mut entities: ResMut<EntityIndex>,
    assets: Res<AssetServer>,
) {
    for event in events.read() {
        let env = Environment { commands: &mut commands, entities: &mut entities, assets: &assets };
        match event {
            GameEvent::SpawnMap { name, file, position }    => map::start_spawning_map(name, file, *position, env),
            GameEvent::DespawnMap { name }                  => map::despawn_map(&name, env),
        }
    }
}

/// Tracks various resources relevant to event handler functions.
pub struct Environment<'a, 'w, 's> {
    pub commands: &'a mut Commands<'w, 's>,
    pub entities: &'a mut EntityIndex,
    pub assets: &'a AssetServer,
}

#[derive(States, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub enum Screen {
    #[default]
    Title,
    Overworld,
}

/// An index that keeps track of particular entities.
#[derive(Resource, Default, Debug)]
pub struct EntityIndex {
    pub player: Option<Entity>,
    pub maps: HashMap<&'static str, MapData>,
}

#[derive(Event, Clone, PartialEq, Debug)]
pub enum GameEvent {
    SpawnMap {
        name: &'static str,
        file: &'static str,
        position: Vec3,
    },
    DespawnMap {
        name: &'static str
    }
}