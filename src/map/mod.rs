mod loader;

use bitflags::bitflags;
pub use loader::*;
use std::str::FromStr;
use thiserror::*;
use tiled_parser as tp;
use bevy::prelude::*;
use bevy::log;
use crate::EntityIndex;


/// A tiled map.
#[derive(Asset, TypePath, Debug)]
pub struct Map {
    pub map: tp::Map,
    pub tileset_entries: Vec<TilesetEntry>,
}

/// A tileset entry in a [`Map`].
#[derive(TypePath, Debug)]
pub struct TilesetEntry {
    pub first_gid: u32,
    pub tileset: Handle<Tileset>,
}

/// A tileset referenced by a [`TilesetEntry`].
#[derive(Asset, TypePath, Debug)]
pub struct Tileset {    
    pub tileset: tp::Tileset,
    pub image: Handle<Image>,
}

/// Spawns a [`Map`] entity.
/// Map contents load asynchronously.
pub fn spawn_map(
    trigger: Trigger<SpawnMapMsg>,
    mut entities: ResMut<EntityIndex>,
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let event = trigger.event();
    let (map_name, map_file) = (event.name, event.file);
    if entities.maps.contains_key(map_name) {
        panic!("Map '{}' already spawned", map_name);
    }
    let map_handle: Handle<Map> = assets.load(map_file);
    let map_entity = commands.spawn(map_handle).id();
    entities.maps.insert(map_name, map_entity);
    log::info!("Spawned map `{map_name}`, file: `{map_file}`");
}

/// Despawns a [`Map`].
pub fn despawn_map(
    trigger: Trigger<DespawnMapMsg>,
    mut entities: ResMut<EntityIndex>,
    mut commands: Commands,
) {
    let map_name = trigger.event().name;
    let map_entity = match entities.maps.remove(map_name) {
        Some(entity) => entity,
        None => panic!("Map '{}' not spawned", map_name),
    };
    commands.entity(map_entity).despawn_recursive();
    log::info!("Despawned map '{map_name}'");
}

/// Monitors loading [`Map`] entities, and finalizes them once they finish loading.
pub fn finish_maps(
    mut commands: Commands,
    mut map_entities: Query<(Entity, &Handle<Map>)>,
    map_assets: Res<Assets<Map>>,
    tileset_assets: Res<Assets<Tileset>>,
    image_assets: Res<Assets<Image>>,
    asset_server: Res<AssetServer>,
) {
    for (map_entity, map_handle) in &mut map_entities {
        if asset_server.is_loaded_with_dependencies(map_handle) {
            finish_map(&mut commands, map_entity, map_handle, &map_assets, &tileset_assets, &image_assets);
        }
    }
}

fn finish_map(
    commands: &mut Commands,
    map_entity: Entity,
    map_handle: &Handle<Map>,
    map_assets: &Assets<Map>,
    tileset_assets: &Assets<Tileset>,
    image_assets: &Assets<Image>,
) {
    let map = map_assets.get(map_handle).unwrap();
    for group_layer in map.map.layers() {
        let group_layer = match group_layer.as_group_layer() {
            Some(group_layer) => group_layer,
            None => panic!("Layer '{}'", group_layer.name()),
        };
        
    }
    commands.entity(map_entity).remove::<Handle<Map>>();
    log::info!("Finished map");
}


struct TileMeta {
    collision_geom: TileGeom,
    graphics_geom: TileGeom,
}


#[derive(Default, Debug)]
struct TileGeom {
    shape: TileShape,
    reset: bool,
    cliffs: Cliffs,
}


#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
enum TileShape {
    Wall,
    WallSW,
    WallSE,
    WallFloorSE,
    WallFloorSW,
    #[default]
    Floor,
    FloorWallSE,
    FloorWallSW,
    FloorSlopeSE,
    FloorSlopeSW,
    Slope,
    SlopeFloorSE,
    SlopeFloorSW,
}

impl FromStr for TileShape {
    type Err = MapSpawningError;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str {
            "wall"              => Ok(Self::Wall),
            "wall-se"           => Ok(Self::WallSE),
            "wall-sw"           => Ok(Self::WallSW),
            "wall-floor-se"     => Ok(Self::WallFloorSE),
            "wall-floor-sw"     => Ok(Self::WallFloorSW),
            "floor"             => Ok(Self::Floor),
            "floor-wall-se"     => Ok(Self::FloorWallSE),
            "floor-wall-sw"     => Ok(Self::FloorWallSW),
            "floor-slope-se"    => Ok(Self::FloorSlopeSE),
            "floor-slope-sw"    => Ok(Self::FloorSlopeSW),
            "slope"             => Ok(Self::Slope),
            "slope-floor-se"    => Ok(Self::SlopeFloorSE),
            "slope-floor-sw"    => Ok(Self::SlopeFloorSW),
            _ => Err(MapSpawningError::InvalidTileShape),
        }
    }
}

bitflags! {
    #[derive(Debug, Default)]
    struct Cliffs: u8 {
        const NONE  = 0b00000000;
        const NORTH = 0b00000001;
        const EAST  = 0b00000010;
        const WEST  = 0b00000100;
    }
}


#[derive(Error, Debug)]
pub enum MapSpawningError {
    #[error("Invalid tile shape")]
    InvalidTileShape,
}

#[derive(Event, Copy, Clone, PartialEq, Debug)]
pub struct SpawnMapMsg {
    pub name: &'static str,
    pub file: &'static str,
    pub position: Vec3,
}

#[derive(Event, Copy, Clone, Eq, PartialEq, Debug)]
pub struct DespawnMapMsg {
    pub name: &'static str,
}