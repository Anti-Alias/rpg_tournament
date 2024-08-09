mod map;
mod action;
mod act;
mod area;
mod camera;
mod pixel;
mod daynight;
mod player;
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
        ));

        // States and resources
        app.init_state::<ScreenStates>();
        app.init_state::<DebugStates>();
        app.init_resource::<EntityIndex>();

        // Observers
        app.observe(action::run_action);
        app.observe(action::quit_action);
        app.observe(map::spawn_map);
        app.observe(map::despawn_map);
        app.observe(map::spawn_entity);
        app.observe(player::spawn_player);
        app.observe(area::init_world);

        // Daynight
        app.init_resource::<daynight::GameTime>();

        // Map
        app.init_asset::<map::Map>();
        app.init_asset::<map::Tileset>();
        app.init_asset::<map::Area>();
        app.init_asset_loader::<map::MapLoader>();
        app.init_asset_loader::<map::TilesetLoader>();
        app.init_asset_loader::<map::AreaLoader>();
        
        // Common
        app.init_resource::<common::CommonAssets>();

        // Systems
        app.add_systems(PreUpdate, (
            map::process_loaded_maps,
            area::stream_current_area,
            area::despawn_area_locals,
        ));
        app.add_systems(Update, (
            action::run_action_queues,
            daynight::update_game_time,
            player::move_players,
            camera::control_flycam
                .run_if(in_state(DebugStates::Enabled)),
            mobs::update_fireflies
                .after(daynight::update_game_time),
            player::draw_players
                .after(player::move_players),
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
    pub maps: HashMap<String, Entity>,
}


/// All high-level messages that drive application logic.
pub mod messages {
    pub use crate::player::messages::SpawnPlayer;
    pub use crate::area::messages::InitArea;
    pub use crate::map::messages::SpawnMap;
    pub use crate::map::messages::DespawnMap;
    pub use crate::action::messages::RunAction;
    pub use crate::action::messages::QuitAction;
}