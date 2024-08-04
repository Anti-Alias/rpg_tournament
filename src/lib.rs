mod map;
mod action;
mod act;
mod overworld;
mod camera;
mod pixel;

use bevy::pbr::PbrProjectionPlugin;
use bevy::render::camera::CameraProjectionPlugin;
use camera::DualProjection;
use map::*;
use bevy::prelude::*;
use bevy::utils::HashMap;

pub use action::ActionKind;


/// Game engine plugin.
pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            CameraProjectionPlugin::<DualProjection>::default(),
            PbrProjectionPlugin::<DualProjection>::default(),
            pixel::PixelPlugin::default(),
        ));
        app.init_state::<ScreenStates>();
        app.init_state::<DebugStates>();
        app.init_asset::<Map>();
        app.init_asset::<Tileset>();
        app.init_asset_loader::<MapLoader>();
        app.init_asset_loader::<TilesetLoader>();
        app.init_resource::<EntityIndex>();
        app.init_resource::<overworld::GameTime>();
        app.insert_resource(AmbientLight { color: Color::WHITE, brightness: 400.0, });
        app.observe(overworld::init_overworld);
        app.observe(action::run_action);
        app.observe(action::quit_action);
        app.observe(map::spawn_map);
        app.observe(map::despawn_map);
        app.add_systems(Update, (
            action::run_action_queues,
            map::finish_maps.after(action::run_action_queues),
            camera::control_flycam.run_if(in_state(DebugStates::Enabled)),
            overworld::update_game_time,
        ));
    }
}


#[derive(States, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub enum DebugStates {
    #[default]
    Enabled,
    Disabled,
}

#[derive(States, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub enum ScreenStates {
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