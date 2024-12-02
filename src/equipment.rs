use bevy::prelude::*;
use bevy_mod_sprite3d::Sprite3d;
use derive_more::*;
use crate::animation::{AnimationBundle, AnimationSet, AnimationState, AnimationSync};
use crate::common::CommonAssets;

/// Component that stores [`Equippable`]s.
#[derive(Component, Reflect, Default, Debug)]
#[reflect(Default)]
pub struct Equipment {
    pub hair: Option<Equippable>,
    pub hat: Option<Equippable>,
    pub outfit: Option<Equippable>,
    pub hands: Option<Equippable>,
    pub feet: Option<Equippable>,
}

/// Type of equippable.
#[derive(Reflect, From, Clone, PartialEq, Debug)]
#[reflect(Default)]
pub enum Equippable {
    Hair(Hair),
    Hat(Hat),
    Outfit(Outfit),
}

impl Default for Equippable {
    fn default() -> Self {
        Self::Hair(Hair::default())
    }
}

impl Equippable {
    pub fn info(&self) -> EquippableInfo {
        match *self {
            Self::Hair(Hair { kind: HairKind::Spikey, color, brightness })      => EquippableInfo { name: "Spikey", image: "player/hair/char_a_p1_4har_spk2_v00.png", color, brightness },
            Self::Hair(Hair { kind: HairKind::Bob, color, brightness })         => EquippableInfo { name: "Bob Cut", image: "player/hair/char_a_p1_4har_bob2_v00.png", color, brightness },
            Self::Hair(Hair { kind: HairKind::Ponytail, color, brightness })    => EquippableInfo { name: "Ponytail", image: "player/hair/char_a_p1_4har_pon1_v00.png", color, brightness },
            Self::Hat(Hat::Pointy)                                              => EquippableInfo { name: "Pointy Hat", image: "player/hat/char_a_p1_5hat_pnty_v01.png", ..default() },
            Self::Outfit(Outfit::Casual1)                                       => EquippableInfo { name: "Casual 1", image: "player/outfit/casual_1.png", ..default() },
            Self::Outfit(Outfit::Casual2)                                       => EquippableInfo { name: "Casual 2", image: "player/outfit/casual_2.png", ..default() },
            Self::Outfit(Outfit::Casual3)                                       => EquippableInfo { name: "Casual 3", image: "player/outfit/casual_3.png", ..default() },
            Self::Outfit(Outfit::Casual4)                                       => EquippableInfo { name: "Casual 4", image: "player/outfit/casual_4.png", ..default() },
            Self::Outfit(Outfit::Casual5)                                       => EquippableInfo { name: "Casual 5", image: "player/outfit/casual_5.png", ..default() },
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct EquippableInfo {
    pub name: &'static str,
    pub image: &'static str,
    pub color: Color,
    pub brightness: f32,
}

impl Default for EquippableInfo {
    fn default() -> Self {
        Self {
            name: "",
            image: "",
            color: Color::WHITE,
            brightness: 1.0,
        }
    }
}

#[derive(Reflect, Clone, PartialEq, Debug)]
#[reflect(Default)]
pub struct Hair {
    pub kind: HairKind,
    pub color: Color,
    pub brightness: f32,
}

impl Default for Hair {
    fn default() -> Self {
        Self {
            kind: HairKind::default(),
            color: Color::WHITE,
            brightness: 1.0,
        }
    }
}

impl From<HairKind> for Hair {
    fn from(kind: HairKind) -> Self {
        Self { kind, ..default() }
    }
}


#[derive(Reflect, Clone, PartialEq, Debug, Default)]
#[reflect(Default)]
pub enum HairKind {
    #[default]
    Spikey,
    Bob,
    Ponytail,
}

#[derive(Reflect, Clone, PartialEq, Debug, Default)]
#[reflect(Default)]
pub enum Hat {
    #[default]
    Pointy,
}


#[derive(Reflect, Clone, PartialEq, Debug, Default)]
#[reflect(Default)]
pub enum Outfit {
    #[default]
    Casual1,
    Casual2,
    Casual3,
    Casual4,
    Casual5,
}


const OUTFIT_OFFSET: f32 = 0.001;
const HAIR_OFFSET: f32 = 0.002;
const HAT_OFFSET: f32 = 0.003;

/// Spawns child equipment entities whenever an entity's equipment changes.
pub fn spawn_equipment_entities(
    common_assets: Res<CommonAssets>,
    assets: Res<AssetServer>,
    mut entities_with_equipment: Query<(Entity, &Equipment), Or<(Added<Equipment>, Changed<Equipment>)>>,
    mut commands: Commands,
) {

    // Logic that spawns an item as the child of another entity.
    let spawn_equippable = |parent: Entity, equippable: &Equippable, offset: f32, commands: &mut Commands| {
        let item_info = equippable.info();
        let item_color = item_info.color.to_linear() * (item_info.brightness);
        let item_mat = create_material(&assets, item_info.image);
        let item_id = commands
            .spawn(AnimationBundle {
                sprite3d: Sprite3d { color: item_color.into(), ..default() },
                animation_set: AnimationSet(common_assets.animations.player.clone()),
                animation_state: AnimationState { stopped: true, ..default() },
                material: MeshMaterial3d(assets.add(item_mat)),
                transform: Transform::from_xyz(0.0, offset, offset),
                ..default()
            })
            .insert((Name::new("hair"), AnimationSync(parent)))
            .id();
            commands.entity(parent).add_child(item_id);
    };

    // Handles spawning / despawning descendants for all equippable things.
    for (entity, equip) in &mut entities_with_equipment {
        commands.entity(entity).despawn_descendants();
        if let Some(ref item) = equip.hat {
            spawn_equippable(entity, item, HAT_OFFSET, &mut commands);
        }
        if let Some(ref item) = equip.hair {
            spawn_equippable(entity, item, HAIR_OFFSET, &mut commands);
        }
        if let Some(ref item) = equip.outfit {
            spawn_equippable(entity, item, OUTFIT_OFFSET, &mut commands);
        }
    }
}


pub(crate) fn create_material(assets: &AssetServer, image: &'static str) -> StandardMaterial {
    StandardMaterial {
        base_color_texture: Some(assets.load::<Image>(image)),
        reflectance: 0.0,
        perceptual_roughness: 1.0,
        cull_mode: None,
        alpha_mode: AlphaMode::Mask(0.5),
        double_sided: false,
        ..default()
    }
}
