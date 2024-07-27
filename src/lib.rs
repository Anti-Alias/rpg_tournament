mod map;
mod action;
mod act;

pub use map::*;
pub use action::*;
use bevy::prelude::*;
use bevy::utils::HashMap;


/// Game engine plugin.
pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {

        app.init_state::<Screen>();
        app.init_asset::<Map>();
        app.init_asset::<Tileset>();
        app.init_asset_loader::<MapLoader>();
        app.init_resource::<EntityIndex>();
        app.observe(run_action);
        app.observe(quit_action);
        app.observe(spawn_map);
        app.observe(despawn_map);
        app.add_systems(Update, (
            run_action_queues,
            finish_maps.after(run_action_queues),
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
