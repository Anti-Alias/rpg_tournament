use bevy::prelude::*;
use crate::animation::{AnimationSet, AnimationSetData};
use crate::player;

/// Resource that stores simple, common assets used across the application.
/// These assets are generally lightweight.
#[derive(Resource, Debug)]
pub struct CommonAssets {
    pub meshes: CommonMeshes,
    pub materials: CommonMaterials,
    pub animations: CommonAnimations,
}

impl FromWorld for CommonAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>().clone();
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
        let materials = CommonMaterials::new(&assets, &mut materials);
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        let meshes = CommonMeshes::new(&mut meshes);
        let mut animations = world.resource_mut::<Assets<AnimationSetData>>();
        let animations = CommonAnimations::new(&mut animations);
        Self { meshes, materials, animations }
    }
}

#[derive(Debug)]
pub struct CommonMeshes {
    pub sphere: Handle<Mesh>,
    pub plane: Handle<Mesh>,
}

impl CommonMeshes {
    fn new(meshes: &mut Assets<Mesh>) -> Self {
        Self {
            sphere: meshes.add(Sphere::new(1.0)),
            plane: meshes.add(Plane3d::new(Vec3::Y, Vec2::new(0.5, 0.5)))
        }
    }
}

#[derive(Debug)]
pub struct CommonMaterials {
    pub white: Handle<StandardMaterial>,
    pub player: Handle<StandardMaterial>,
}

impl CommonMaterials {
    fn new(assets: &AssetServer, materials: &mut Assets<StandardMaterial>) -> Self {
        Self {
            white: materials.add(StandardMaterial { base_color: Color::WHITE, unlit: true, ..default() }),
            player: materials.add(player::create_material(assets, "player/base/char_a_p1_0bas_humn_v00.png"))
        }
    }
}

#[derive(Debug)]
pub struct CommonAnimations {
    pub player: Handle<AnimationSetData>,
}

impl CommonAnimations {
    fn new(animations: &mut Assets<AnimationSetData>) -> Self {
        Self {
            player: animations.add(crate::player::create_player_animations()),
        }
    }
}
