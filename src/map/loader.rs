use bevy::prelude::*;
use bevy::asset::{AssetLoader, AsyncReadExt, LoadContext};
use bevy::asset::io::Reader;
use tiled_parser as tp;
use thiserror::*;

use super::{Map, Tileset, TilesetEntry};

/// Loads a [`Map`].
#[derive(Default)]
pub struct MapLoader;
impl AssetLoader for MapLoader {

    type Asset = Map;
    type Settings = ();
    type Error = MapLoadError;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a (),
        load_context: &'a mut LoadContext<'_>,
    ) -> Result<Map, MapLoadError> {

        // Reads map bytes
        let mut bytes = vec![];
        reader.read_to_end(&mut bytes).await?;
        let map: tp::Map = tp::Map::parse(bytes.as_slice())?;

        // Loads tileset dependencies
        let mut tileset_entries: Vec<TilesetEntry> = vec![];
        let map_tileset_entries = map.tileset_entries();
        for tileset_entry in map_tileset_entries {
            match tileset_entry.kind() {
                tiled_parser::TilesetEntryKind::Internal(tileset) => {
                    let image = match tileset.image() {
                        Some(image) => image,
                        None => return Err(MapLoadError::MissingImageError),
                    };
                    let mut tileset_load_context = load_context.begin_labeled_asset();
                    let image_path = match load_context.asset_path().parent() {
                        Some(dir) => format!("{}/{}", dir, image.source()),
                        None => image.source().to_owned(),
                    };
                    let image_handle: Handle<Image> = tileset_load_context.load(image_path);
                    let tileset = Tileset {
                        tileset: tileset.clone(),
                        base_color_texture: image_handle,
                        ..default()
                    };
                    let tileset_handle = load_context.add_loaded_labeled_asset(
                        tileset.tileset.name().to_owned(),
                        tileset_load_context.finish(tileset, None)
                    );
                    tileset_entries.push(TilesetEntry {
                        tileset: tileset_handle,
                    });
                },
                tiled_parser::TilesetEntryKind::External(file) => {
                    let tileset_path = match load_context.asset_path().parent() {
                        Some(map_dir) => format!("{}/{}", map_dir, file),
                        None => file.clone(),
                    };
                    let tileset_handle: Handle<Tileset> = load_context.load(tileset_path);
                    tileset_entries.push(TilesetEntry {
                        tileset: tileset_handle,
                    })
                },
            }
        }
        Ok(Map {
            map,
            tileset_entries,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["tmx"]
    }
}

/// Loads a [`Map`].
#[derive(Default)]
pub struct TilesetLoader;
impl AssetLoader for TilesetLoader {

    type Asset = Tileset;
    type Settings = ();
    type Error = MapLoadError;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a (),
        load_context: &'a mut LoadContext<'_>,
    ) -> Result<Tileset, MapLoadError> {

        // Reads tileset bytes
        let mut bytes = vec![];
        reader.read_to_end(&mut bytes).await?;

        // Adds main image as a dependency
        let tileset: tp::Tileset = tp::Tileset::parse(bytes.as_slice())?;
        let image_source = match tileset.image() {
            Some(image) => image.source(),
            None => return Err(MapLoadError::MissingImageError),
        };
        let image_path = match load_context.asset_path().parent() {
            Some(dir) => format!("{dir}/{image_source}"),
            None => image_source.to_owned(),
        };
        let image_handle = load_context.load(image_path);

        // TODO: Parse / load
        let base_color = Color::WHITE;
        let emissive = LinearRgba::BLACK;
        let emissive_texture = None;
        let normal_texture = None;

        Ok(Tileset {
            tileset,
            base_color_texture: image_handle,
            base_color,
            emissive,
            emissive_texture,
            normal_texture,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["tsx"]
    }
}

#[derive(Error, Debug)]
#[error(transparent)]
pub enum MapLoadError {
    IOError(#[from] std::io::Error),
    MapError(#[from] tp::Error),
    #[error("All tilesets must include an image")]
    MissingImageError,
}