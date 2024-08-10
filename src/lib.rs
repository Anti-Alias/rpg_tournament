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
mod debug;

use bevy::pbr::PbrProjectionPlugin;
use bevy::render::camera::CameraProjectionPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use camera::DualProjection;
use bevy::prelude::*;
use bevy::utils::HashMap;

pub use action::ActionKind;
use debug::DebugStates;


/// Game engine plugin.
pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::Off);
        app.add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            CameraProjectionPlugin::<DualProjection>::default(),
            PbrProjectionPlugin::<DualProjection>::default(),
            WorldInspectorPlugin::default().run_if(in_state(DebugStates::Enabled)),
        ));

        // States and resources
        app.init_state::<ScreenStates>();
        app.init_state::<debug::DebugStates>();
        app.init_resource::<EntityIndex>();

        // Observers
        app.observe(action::run_action);
        app.observe(action::quit_action);
        app.observe(map::spawn_map);
        app.observe(map::despawn_map);
        app.observe(map::spawn_entity);
        app.observe(player::spawn_player);
        app.observe(area::init_area);

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

        // System sets
        app.configure_sets(Update, (
            GameSets::Flush.after(GameSets::PreLogic),
            GameSets::Logic.after(GameSets::Flush),
            GameSets::PostLogic.after(GameSets::Logic),
        ));

        // Systems
        app.add_systems(Update, (

            /////////////// PreLogic ///////////////
            (
                action::run_action_queues,
                map::process_loaded_maps,
                area::stream_current_area,
                area::despawn_area_locals,
                daynight::update_game_time,
                area::reload_area
                    .run_if(in_state(DebugStates::Enabled)),
            ).in_set(GameSets::PreLogic),

            /////////////// Flush ///////////////
            apply_deferred.in_set(GameSets::Flush),

            /////////////// Logic ///////////////
            (
                player::update_players,
                mobs::update_fireflies,
                debug::toggle_debug,
                camera::update_flycam,
                (
                    camera::toggle_projection,
                    camera::toggle_flycam,
                ).run_if(in_state(DebugStates::Enabled)),
                player::draw_players
                    .after(player::update_players),
            ).in_set(GameSets::Logic),

            /////////////// PostLogic ///////////////
            camera::update_game_camera.in_set(GameSets::PostLogic),
        ));

        app.add_systems(OnEnter(DebugStates::Disabled), camera::handle_disable_debug);

        // Misc systems
        app.add_systems(
            PostUpdate,
            pixel::round_positions.after(TransformSystem::TransformPropagate)
        );
    }
}

#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameSets {
    PreLogic,
    Flush,
    Logic,
    PostLogic,
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