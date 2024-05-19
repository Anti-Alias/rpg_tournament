use std::time::Duration;

use bevy::asset::AssetPath;
use bevy::prelude::*;

use crate::animation::{Animation, AnimationSet};

pub fn asset_extension_plugin(app: &mut App) {
    app.add_systems(Startup, insert_common_assets);
}


fn insert_common_assets(mut commands: Commands, mut animation_sets: ResMut<Assets<AnimationSet>>) {
    commands.insert_resource(CommonAssets {
        player_animations: animation_sets.add(player_anim_set()),
    })
}

fn player_anim_set() -> AnimationSet {
    let sprite_size = Vec2::new(64.0, 64.0);
    let padding = Vec2::ZERO;
    let frame_duration = Duration::from_secs_f32(1.0 / 12.0);
    let walk_south = Animation::from_row(Vec2::new(0.0, 4.0) * sprite_size, sprite_size, padding, frame_duration, 6, 512);
    let walk_north = Animation::from_row(Vec2::new(0.0, 5.0) * sprite_size, sprite_size, padding, frame_duration, 6, 512);
    let walk_east = Animation::from_row(Vec2::new(0.0, 6.0) * sprite_size, sprite_size, padding, frame_duration, 6, 512);
    let walk_west = Animation::from_row(Vec2::new(0.0, 7.0) * sprite_size, sprite_size, padding, frame_duration, 6, 512);
    AnimationSet(vec![walk_south, walk_north, walk_east, walk_west])
}

#[derive(Resource, Clone, Eq, PartialEq)]
pub struct CommonAssets {
    pub player_animations: Handle<AnimationSet>,
}

/// Extends the functionality of [`AssetServer`] by keeping track of the handles loaded since creation.
/// This is useful for waiting on a set of handles before continuing on to another task.
#[derive(Clone, Deref)]
pub struct AssetBatch {
    #[deref]
    pub assets: AssetServer,
    handles: Vec<UntypedHandle>,
}

impl AssetBatch {
    pub fn new(assets: AssetServer) -> Self {
        Self { assets, handles: vec![] }
    }

    /// Passthrough for [`load`](AssetServer::load).
    /// Keeps track of handle in separate vec.
    pub fn load<'b, A: Asset>(&mut self, path: impl Into<AssetPath<'b>>) -> Handle<A> {
        let handle = self.assets.load(path);
        self.handles.push(handle.clone().untyped());
        handle
    }

    /// Finishes the batch, returning the handles accumulated.
    pub fn finish(self) -> Vec<UntypedHandle> {
        self.handles
    }
}