use std::str::Utf8Error;
use std::time::Duration;

use bevy::asset::{AssetLoader, AsyncReadExt};
use bevy::prelude::*;
use bevy::utils::BoxedFuture;
use ron::de::SpannedError;
use serde::Deserialize;
use thiserror::*;

use crate::animation::{Animation, AnimationSet};

pub fn asset_extension_plugin(app: &mut App) {
    app.init_asset_loader::<StandardMaterialLoader>();
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

#[derive(Default)]
pub struct StandardMaterialLoader;

impl AssetLoader for StandardMaterialLoader {
    type Asset = StandardMaterial;
    type Settings = ();
    type Error = StandardMaterialError;
    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = vec![];
            reader.read_to_end(&mut bytes).await?;
            let ron_data = std::str::from_utf8(&bytes)?;
            let load_mat: LoadingMaterial = ron::from_str(ron_data)?;
            Ok(StandardMaterial {
                base_color: load_mat.base_color,
                base_color_texture: load_mat.base_color_texture.map(|str| load_context.load(str)),
                perceptual_roughness: load_mat.perceptual_roughness,
                alpha_mode: load_mat.alpha_mode.to_bevy(),
                cull_mode: load_mat.cull_mode.map(|f| f.to_bevy()),
                double_sided: load_mat.double_sided,
                ..default()
            })
        })
    }
    
    fn extensions(&self) -> &[&str] {
        &["ron.stdmat"]
    }
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct LoadingMaterial {
    pub base_color: Color,
    pub base_color_texture: Option<String>,
    pub perceptual_roughness: f32,
    pub alpha_mode: AlphaMode,
    pub cull_mode: Option<Face>,
    pub double_sided: bool,
}

#[derive(Deserialize, Copy, Clone, PartialEq, Default, Debug)]
enum AlphaMode {
    #[default]
    Opaque,
    Mask(f32),
    Blend,
    Premultiplied,
    Add,
    Multiply,
}

impl AlphaMode {
    fn to_bevy(self) -> bevy::prelude::AlphaMode {
        match self {
            AlphaMode::Opaque => bevy::prelude::AlphaMode::Opaque,
            AlphaMode::Mask(val) => bevy::prelude::AlphaMode::Mask(val),
            AlphaMode::Blend => bevy::prelude::AlphaMode::Blend,
            AlphaMode::Premultiplied => bevy::prelude::AlphaMode::Premultiplied,
            AlphaMode::Add => bevy::prelude::AlphaMode::Add,
            AlphaMode::Multiply => bevy::prelude::AlphaMode::Multiply,
        }
    }
}

#[derive(Deserialize, Copy, Clone, PartialEq, Debug)]
pub enum Face {
    Front = 0,
    Back = 1,
}

impl Face {
    fn to_bevy(self) -> bevy::render::render_resource::Face {
        match self {
            Face::Front => bevy::render::render_resource::Face::Front,
            Face::Back => bevy::render::render_resource::Face::Back,
        }
    }
}

#[derive(Error, Debug)]
pub enum StandardMaterialError {
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    Utf8Error(#[from] Utf8Error),
    #[error(transparent)]
    SpannedError(#[from] SpannedError),
}