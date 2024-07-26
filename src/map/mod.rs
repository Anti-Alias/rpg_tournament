mod loader;

use bitflags::bitflags;
pub use loader::*;

use std::str::FromStr;
use thiserror::*;
use tiled_parser as tp;
use bevy::prelude::*;
use crate::{EntityIndex, Environment};


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

/// Refers to a map that's either loading, or finished loading and has spawned.
#[derive(Clone, Debug)]
pub enum MapData {
    Loading { handle: Handle<Map>, position: Vec3 },
    Loaded { entity: Entity },
}

/// Monitors loading [`Map`]s and spawns them when they finish loading.
pub fn finish_spawning_maps(
    mut commands: Commands,
    mut entities: ResMut<EntityIndex>,
    assets: Res<AssetServer>,
    maps: Res<Assets<Map>>,
    tilesets: Res<Assets<Tileset>>,
    images: Res<Assets<Image>>,
) {
    for map_data in &mut entities.maps.values_mut() {
        let (map_handle, _position) = match map_data {
            MapData::Loading { handle, position } => (handle, position),
            MapData::Loaded { .. } => continue,
        };
        if !assets.is_loaded_with_dependencies(&*map_handle) { continue };
        let map_entity = spawn_map(&mut commands, &map_handle, &maps, &tilesets, &images);
        *map_data = MapData::Loaded { entity: map_entity };
        println!("Finished spawning map");
    }
}


/// Enqueues the spawning of a [`Map`].
/// Once loaded, map entity will be spawned.
pub fn start_spawning_map(
    name: &'static str,
    file: &'static str,
    position: Vec3,
    env: Environment,
) {
    let (entities, assets) = (env.entities, env.assets);
    if entities.maps.contains_key(name) {
        panic!("Map {name} already spawned");
    }
    entities.maps.insert(name, MapData::Loading { handle: assets.load(file), position });
    println!("Spawning map");
}

fn spawn_map(
    commands: &mut Commands,
    map_handle: &Handle<Map>,
    maps: &Assets<Map>,
    tilesets: &Assets<Tileset>,
    images: &Assets<Image>,
) -> Entity {
    let map = maps.get(map_handle).unwrap();
    for group_layer in map.map.layers() {
        let group_layer = match group_layer.as_group_layer() {
            Some(group_layer) => group_layer,
            None => panic!("Layer '{}'", group_layer.name()),
        };
        
    }
    todo!()
}

/// Despawns a [`Map`].
pub fn despawn_map(name: &str, env: Environment) {
    let (entities, commands) = (env.entities, env.commands);
    if !entities.maps.contains_key(name) {
        panic!("Map {name} not spawned");
    }
    let map_data = entities.maps.remove(name);
    match map_data {
        Some(MapData::Loading { .. })       => {},
        Some(MapData::Loaded { entity })    => commands.entity(entity).despawn_recursive(),
        None                                => panic!("Map {name} not spawned"),
    };
    println!("Despawned map");
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