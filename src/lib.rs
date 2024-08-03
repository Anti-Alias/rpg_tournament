mod map;
mod action;
mod act;
mod overworld;
mod camera;

use map::*;
use bevy::prelude::*;
use bevy::utils::HashMap;

pub use action::ActionKind;


/// Game engine plugin.
pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<Screen>();
        app.init_asset::<Map>();
        app.init_asset::<Tileset>();
        app.init_asset_loader::<MapLoader>();
        app.init_asset_loader::<TilesetLoader>();
        app.init_resource::<EntityIndex>();
        app.observe(overworld::init_overworld);
        app.observe(action::run_action);
        app.observe(action::quit_action);
        app.observe(map::spawn_map);
        app.observe(map::despawn_map);
        app.add_systems(Update, (
            action::run_action_queues,
            map::finish_maps.after(action::run_action_queues),
            camera::control_flycam,
        ));
    }
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
    pub maps: HashMap<&'static str, Entity>,
}


/// All high-level messages that drive application logic.
pub mod messages {
    pub use crate::overworld::messages::InitOverworld;
    pub use crate::map::messages::SpawnMap;
    pub use crate::map::messages::DespawnMap;
    pub use crate::action::messages::RunAction;
    pub use crate::action::messages::QuitAction;
}