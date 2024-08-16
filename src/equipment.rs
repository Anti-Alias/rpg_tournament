use bevy::prelude::*;
use derive_more::*;
use crate::animation::{AnimationBundle, AnimationState, AnimationSync};
use crate::common::CommonAssets;
use crate::item::Item;

/// Component that stores [`Equippable`]s.
#[derive(Component, Default, Debug)]
pub struct Equipment {
    pub hat: Option<Item>,
    pub outfit: Option<Item>,
    pub hands: Option<Equippable>,
    pub feet: Option<Equippable>,
}

/// Type of equippable.
#[derive(From, Clone, PartialEq, Debug)]
pub enum Equippable {
    Outfit(Outfit),
}


#[derive(Clone, PartialEq, Debug)]
pub enum Outfit {
    Casual1,
    Casual2,
    Casual3,
    Casual4,
    Casual5,
}



const OUTFIT_OFFSET: f32 = 0.001;

pub fn spawn_equipment_entities(
    common_assets: Res<CommonAssets>,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut entities_with_equipment: Query<
        (Entity, &Equipment),
        Or<(Added<Equipment>, Changed<Equipment>)>
    >,
    mut commands: Commands,
) {
    for (equip_id, equip) in &mut entities_with_equipment {
        commands.entity(equip_id).despawn_descendants();
        if let Some(ref outfit_item) = equip.outfit {
            let outfit_mat = create_material(&assets, outfit_item.info().image);
            let outfit_id = commands
                .spawn(AnimationBundle {
                    animation_set: common_assets.animations.player.clone(),
                    animation_state: AnimationState { stopped: true, ..default() },
                    material: materials.add(outfit_mat),
                    transform: Transform::from_xyz(0.0, OUTFIT_OFFSET, OUTFIT_OFFSET),
                    ..default()
                })
                .insert(AnimationSync(equip_id))
                .id();
            commands.entity(equip_id).add_child(outfit_id);
        }
    }
}


pub(crate) fn create_material(assets: &AssetServer, path: &'static str) -> StandardMaterial {
    StandardMaterial {
        base_color_texture: Some(assets.load::<Image>(path)),
        reflectance: 0.0,
        perceptual_roughness: 1.0,
        cull_mode: None,
        alpha_mode: AlphaMode::Mask(0.5),
        double_sided: false,
        ..default()
    }
}