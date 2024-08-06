mod map;
mod action;
mod act;
mod overworld;
mod camera;
mod pixel;
mod daynight;
mod mobs;
mod common;

use bevy::pbr::PbrProjectionPlugin;
use bevy::render::camera::CameraProjectionPlugin;
use camera::DualProjection;
use bevy::prelude::*;
use bevy::utils::HashMap;

pub use action::ActionKind;


/// Game engine plugin.
pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::Off);
        app.add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            CameraProjectionPlugin::<DualProjection>::default(),
            PbrProjectionPlugin::<DualProjection>::default(),
            pixel::PixelPlugin::default(),
        ));

        // Lib
        app.init_state::<ScreenStates>();
        app.init_state::<DebugStates>();
        app.init_resource::<EntityIndex>();
        app.insert_resource(AmbientLight { color: Color::WHITE, brightness: 300.0, });

        // Observers
        app.observe(action::run_action);
        app.observe(action::quit_action);
        app.observe(map::spawn_map);
        app.observe(map::despawn_map);
        app.observe(map::spawn_entity);
        app.observe(overworld::init_overworld);

        // Daynight
        app.init_resource::<daynight::GameTime>();

        // Map
        app.init_asset::<map::Map>();
        app.init_asset::<map::Tileset>();
        app.init_asset_loader::<map::MapLoader>();
        app.init_asset_loader::<map::TilesetLoader>();
        
        // Common
        app.init_resource::<common::CommonAssets>();

        // Systems
        app.add_systems(Update, (
            action::run_action_queues,
            map::process_loaded_maps.after(action::run_action_queues),
            camera::control_flycam.run_if(in_state(DebugStates::Enabled)),
            daynight::update_game_time,
            mobs::update_fireflies,
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