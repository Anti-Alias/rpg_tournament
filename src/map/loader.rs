use bevy::prelude::*;
use bevy::asset::{AssetLoader, AsyncReadExt, LoadContext};
use bevy::asset::io::Reader;
use tiled_parser as tp;
use thiserror::*;

use super::Map;

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
        _load_context: &'a mut LoadContext<'_>,
    ) -> Result<Map, MapLoadError> {
        let mut bytes = vec![];
        reader.read_to_end(&mut bytes).await?;
        let map = tp::Map::parse(bytes.as_slice())?;
        Ok(Map {
            map,
            tileset_entries: vec![],
        })
    }

    fn extensions(&self) -> &[&str] {
        &["tmx"]
    }
}

#[derive(Error, Debug)]
#[error(transparent)]
pub enum MapLoadError {
    IOError(#[from] std::io::Error),
    MapError(#[from] tp::Error),
}