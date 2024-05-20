use std::time::Duration;
use bevy::prelude::*;
use crate::animation::{Animation, AnimationSet};


/// Simple common assets that are used across the game.
#[derive(Resource)]
pub struct CommonAssets {
    pub plane_mesh: Handle<Mesh>,
    pub sphere_mesh: Handle<Mesh>,
    pub cuboid_mesh: Handle<Mesh>,
    pub player_animations: Handle<AnimationSet>,
}

impl FromWorld for CommonAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            plane_mesh: assets.add(Mesh::from(Plane3d { normal: Direction3d::Y })),
            sphere_mesh: assets.add(Mesh::from(Sphere { radius: 0.5 })),
            cuboid_mesh: assets.add(Mesh::from(Cuboid { half_size: Vec3::new(0.5, 0.5, 0.5) })),
            player_animations: assets.add(player_anim_set()),
        }
    }
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