mod map;
mod action;
mod act;
mod area;
mod camera;
mod round;
mod animation;
mod daynight;
mod player;
mod mobs;
mod common;
mod debug;

use bevy::prelude::*;
use bevy::pbr::PbrProjectionPlugin;
use bevy::render::camera::CameraProjectionPlugin;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::utils::HashMap;
use bevy_mod_sprite3d::Sprite3dPlugin;

use camera::DualProjection;
pub use action::ActionKind;
use debug::DebugStates;


/// Game engine plugin.
pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::Off);
        app.add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),                         // Built-in bevy plugins with configuratio0n.
            Sprite3dPlugin::<StandardMaterial>::default(),                              // Adds 3D sprite batch rendering.
            CameraProjectionPlugin::<DualProjection>::default(),                        // Custom camera projection (switch between ortho and perspective).
            PbrProjectionPlugin::<DualProjection>::default(),                           // Custom camera projection (switch between ortho and perspective).
            WorldInspectorPlugin::default().run_if(in_state(DebugStates::Enabled)),     // Debug menu for inspecting entities and resources.
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
        app.init_asset::<animation::AnimationSet>();
        app.init_asset_loader::<map::MapLoader>();
        app.init_asset_loader::<map::TilesetLoader>();
        app.init_asset_loader::<map::AreaLoader>();
        
        // Common
        app.init_resource::<common::CommonAssets>();

        // System sets
        app.configure_sets(Update, (
            GameSystems::Flush.after(GameSystems::Prepare),
            GameSystems::Logic.after(GameSystems::Flush),
            GameSystems::PostLogic.after(GameSystems::Logic),
        ));

        // Systems
        app.add_systems(Update, (

            /////////////// Prepare ///////////////
            (
                action::run_action_queues,
                map::process_loaded_maps,
                area::stream_current_area,
                area::despawn_area_locals,
                daynight::update_game_time,
                area::reload_area
                    .run_if(in_state(DebugStates::Enabled)),
            ).in_set(GameSystems::Prepare),

            /////////////// Flush ///////////////
            apply_deferred.in_set(GameSystems::Flush),

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
            ).in_set(GameSystems::Logic),

            /////////////// PostLogic ///////////////
            (
                camera::follow_target,
                round::round_positions.after(camera::follow_target),
                animation::update_animations,
            ).in_set(GameSystems::PostLogic),
        ));

        app.add_systems(OnEnter(DebugStates::Disabled), camera::handle_disable_debug);
    }
}

#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameSystems {
    /// Update indexes, area streaming, and other low-level tasks.
    Prepare,
    /// Flush commands from [`Prepare`](GameSystems::Prepare).
    Flush,
    /// Decision logic.
    /// Input   -> Action mapping.
    /// AI      -> Action mapping.
    PreLogic,
    /// Main logic.
    /// Executions actions enqueued on entities.
    Logic,
    /// Reactions to main logic that should happen on the same frame.
    /// IE: Reaction to hitboxes, animations etc.
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