mod loader;
mod mesh;
pub use loader::*;

use bevy::utils::HashMap;
use bitflags::bitflags;
use mesh::create_bevy_mesh;
use mesh::GraphicsMesh;
use mesh::GraphicsVertex;
use tiled_parser::PropertyValue;
use tiled_parser::TileLayer;
use tiled_parser::TileLayerRegion;
use tiled_parser as tp;
use bevy::prelude::*;
use bevy::log;
use crate::EntityIndex;

pub const TH: i32 = 2;  // Tile height
pub const THH: i32 = 1; // Half tile height


/// Spawns a [`Map`] entity.
/// Map contents load asynchronously.
pub fn spawn_map(
    trigger: Trigger<messages::SpawnMap>,
    mut entities: ResMut<EntityIndex>,
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let message = trigger.event();
    let (map_name, map_file) = (message.name, message.file);
    if entities.maps.contains_key(map_name) {
        panic!("Map '{}' already spawned", map_name);
    }
    let map_handle: Handle<Map> = assets.load(map_file);
    let map_transf = Transform::from_translation(message.position);
    let map_entity = commands.spawn((
        map_handle,
        SpatialBundle::from_transform(map_transf),
    )).id();
    entities.maps.insert(map_name, map_entity);
    log::info!("Spawned map `{map_name}`, file: `{map_file}`");
}

/// Despawns a [`Map`].
pub fn despawn_map(
    trigger: Trigger<messages::DespawnMap>,
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
    mut material_assets: ResMut<Assets<StandardMaterial>>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    for (map_entity, map_handle) in &mut map_entities {
        if asset_server.is_loaded_with_dependencies(map_handle) {
            finish_map(
                &mut commands,
                map_entity,
                map_handle,
                &map_assets,
                &tileset_assets,
                &mut material_assets,
                &mut mesh_assets,
            );
        }
    }
}

fn finish_map(
    commands: &mut Commands,
    map_entity: Entity,
    map_handle: &Handle<Map>,
    map_assets: &Assets<Map>,
    tileset_assets: &Assets<Tileset>,
    material_assets: &mut Assets<StandardMaterial>,
    mesh_assets: &mut Assets<Mesh>,
) {
    let map = map_assets.get(map_handle).unwrap();
    for layer in map.map.layers() {

        // Parses group layer
        let group_layer = match layer.as_group_layer() {
            Some(group_layer) => group_layer,
            None => panic!("Layer '{}' not a group layer", layer.name()),
        };
        let (regular_layers, group_meta) = parse_group_layer(
            group_layer,
            layer.properties(),
            layer.name(),
            map,
            &tileset_assets
        );
        let lift = (group_meta.lift * map.map.tile_height() as f32) as i32;

        // Forms graphics meshes, parallel with the map's tileset entries.
        let mut gmeshes: Vec<GraphicsMesh> = init_graphics_meshes(map.tileset_entries.len());
        for layer in regular_layers {
            let region = layer.region;
            let (min_x, max_x) = (region.x, region.x + region.width as i32);
            let (min_y, max_y) = (region.y, region.y + region.height as i32);

            // For all columns in regular layer...
            for tile_x in min_x..max_x {
                // Inchwork up the tile column with the graphics strip, appending to the graphics mesh as it goes.
                let mut gstrip = Strip {
                    left: IVec3::new(min_x, lift, 0),
                    right: IVec3::new(min_x+1, lift, 0),
                };
                for tile_y in (min_y..max_y).rev() {
                    let tile_coords = (tile_x, tile_y);
                    let Some(tile) = layer.tiles.get(&tile_coords) else {
                        gstrip.left.z += TH;
                        gstrip.right.z += TH;
                        continue;
                    };
                    let tile_geom = group_meta.graphics_geoms.get(&tile_coords).copied().unwrap_or_default();
                    let gstrip_next = gstrip.next(tile_geom.shape);
                    let (a, b, c, d, e, f) = match tile_geom.shape.quad_info() {
                        QuadInfo::Quad | QuadInfo::Triangle => GraphicsVertex::quad(
                            [
                                gstrip.left,
                                gstrip.right,
                                gstrip_next.right,
                                gstrip_next.left,
                            ],
                            [
                                IVec2::new(tile.uv1.x, tile.uv2.y),
                                IVec2::new(tile.uv2.x, tile.uv2.y),
                                IVec2::new(tile.uv2.x, tile.uv1.y),
                                IVec2::new(tile.uv1
                                    
                                    .x, tile.uv1.y),
                            ],
                        ),
                        QuadInfo::QuadFlipped | QuadInfo::TriangleFlipped => GraphicsVertex::quad(
                            [
                                gstrip.right,
                                gstrip_next.right,
                                gstrip_next.left,
                                gstrip.left,
                            ],
                            [
                                IVec2::new(tile.uv2.x, tile.uv2.y),
                                IVec2::new(tile.uv2.x, tile.uv1.y),
                                IVec2::new(tile.uv1.x, tile.uv1.y),
                                IVec2::new(tile.uv1.x, tile.uv2.y),
                            ],
                        ),
                    };
                    let gmesh = &mut gmeshes[tile.tileset_idx];
                    gmesh.push_quad(a, b, c, d, e, f);
                    gstrip = gstrip_next;
                }
            }
        }

        // Creates materials, parallel with the map's tileset entries
        let materials: Vec<Mat> = map.tileset_entries.iter()
            .map(|entry| tileset_assets.get(&entry.tileset).unwrap())
            .map(|tileset| {
                let image = tileset.tileset.image().expect("Multi image tilesets not supported");
                let width = image.width().expect("Image did not include a width");
                let height = image.height().expect("Image did not include a height");
                let material = material_assets.add(StandardMaterial {
                    base_color: tileset.base_color,
                    base_color_texture: Some(tileset.base_color_texture.clone()),
                    emissive: tileset.emissive,
                    emissive_texture: tileset.emissive_texture.clone(),
                    normal_map_texture: tileset.normal_texture.clone(),
                    ..default()
                });
                Mat { material, width, height }
            })
            .collect();

        // Spawns material/meshes
        for (gmesh, mat) in gmeshes.into_iter().zip(materials) {
            let mesh = create_bevy_mesh(
                gmesh,
                mat.width as f32,
                mat.height as f32,
                map.map.tile_width() as f32,
                map.map.tile_height() as f32,
            );
            commands
                .entity(map_entity)
                .with_children(|b| {
                    b.spawn(PbrBundle {
                        mesh: mesh_assets.add(mesh),
                        material: mat.material,
                        ..default()
                    });
                });
        }
    }
    commands
        .entity(map_entity)
        .remove::<Handle<Map>>();
    log::info!("Finished map");
}

// Material with metadata
struct Mat {
    material: Handle<StandardMaterial>,
    width: u32,
    height: u32,
}

// Strip of two points that travels up a vertical column of tiles.
// Two strips define the quad of a tile.
// Used to build collision and graphics meshes.
#[derive(Copy, Clone, Debug)]
struct Strip {
    left: IVec3,
    right: IVec3,
}

impl Strip {
    fn next(mut self, shape: TileShape) -> Self {
        match shape {
            TileShape::Wall | TileShape::WallNE | TileShape::WallNW     => { self.left.y += TH;     self.right.y += TH; },
            TileShape::WallFloorSE                                      => { self.left.z += TH;     self.right.y += TH; },
            TileShape::WallFloorSW                                      => { self.left.y += TH;     self.right.z += TH; },
            TileShape::Floor | TileShape::FloorNE | TileShape::FloorNW  => { self.left.z += TH;     self.right.z += TH; },
            TileShape::FloorWallSE                                      => { self.left.y += TH;     self.right.z += TH; },
            TileShape::FloorWallSW                                      => { self.left.z += TH;     self.right.y += TH; },
            TileShape::FloorSlopeSE                                     => { self.left.z += THH;    self.left.y += THH;     self.right.z += TH; },
            TileShape::FloorSlopeSW                                     => { self.left.z += TH;     self.right.y += THH;    self.right.z += THH; },
            TileShape::Slope | TileShape::SlopeNE | TileShape::SlopeNW  => { self.left.z += THH;    self.left.y += THH;     self.right.z += THH;    self.right.y += THH; },
            TileShape::SlopeFloorSE                                     => { self.left.z += THH },
            TileShape::SlopeFloorSW                                     => todo!(),
        }
        self
    }
}


fn parse_group_layer(
    group_layer: &tp::GroupLayer,
    group_layer_props: &tp::Properties,
    group_layer_name: &str,
    map: &Map,
    tileset_assets: &Assets<Tileset>,
) -> (Vec<RegularTileLayer>, GroupMeta) {
    
    let mut regular_layers = vec![];
    let mut group_meta = GroupMeta::default();
    for prop in group_layer_props {
        match prop {
            ("lift", PropertyValue::Float(lift)) => group_meta.lift = *lift,
            ("lift", _) => panic!("Property 'lift' not a float"),
            _ => {}
        }
    }
    for layer in group_layer.layers() {
        let tile_layer = match layer.as_tile_layer() {
            Some(tile_layer) => tile_layer,
            None => panic!("Layer '{}/{}' not a tile layer", group_layer_name, layer.name()),
        };
        println!("Parsing {}/{}", group_layer_name, layer.name());
        let layer_type = TileLayerType::from_layer_name(layer.name());
        match layer_type {
            TileLayerType::Regular => regular_layers.push(RegularTileLayer::parse(
                tile_layer, 
                map,
                tileset_assets)
            ),
            TileLayerType::Meta(meta_layer_type) => parse_meta_layer(
                &mut group_meta,
                &tile_layer,
                meta_layer_type,
                map,
                tileset_assets,
            ),
        }
    }
    (regular_layers, group_meta)
}

fn parse_meta_layer(
    group_meta: &mut GroupMeta,
    meta_layer: &TileLayer,
    meta_layer_type: MetaLayerType,
    map: &Map,
    tileset_assets: &Assets<Tileset>,
) {
    let region = meta_layer.region();
    let (min_x, max_x) = (region.x, region.x + region.width as i32);
    let (min_y, max_y) = (region.y, region.y + region.height as i32);
    for x in min_x..max_x {
        for y in (min_y..max_y).rev() {
            let tile_gid = meta_layer.gid_at(x, y);
            let (tileset_idx, tile_id) = match map.map.tile_location_of(tile_gid) {
                Some(result) => result,
                None => continue,
            };
            let tileset_entry = &map.tileset_entries[tileset_idx];
            let tileset = tileset_assets.get(&tileset_entry.tileset).expect("A tileset was not fully loaded");
            let tile = tileset.tileset.tile(tile_id).unwrap();
            let geom = TileGeom::from_tile(tile);
            match meta_layer_type {
                MetaLayerType::Mesh => {
                    group_meta.graphics_geoms.insert((x, y), geom);
                    group_meta.collision_geoms.insert((x, y), geom);
                },
                MetaLayerType::CollisionMesh => {
                    group_meta.collision_geoms.insert((x, y), geom);
                },
                MetaLayerType::GraphicsMesh => {
                    group_meta.graphics_geoms.insert((x, y), geom);
                },
            }
        }
    }
}

fn init_graphics_meshes(count: usize) -> Vec<GraphicsMesh> {
    let mut result = Vec::with_capacity(count);
    for _ in 0..count {
        result.push(GraphicsMesh::new());
    }
    result
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum TileLayerType {
    Regular,
    Meta(MetaLayerType),
}

impl TileLayerType {
    fn from_layer_name(layer_name: &str) -> Self {
        match MetaLayerType::parse(layer_name) {
            Some(typ) => Self::Meta(typ),
            None => Self::Regular,
        }
    }
}


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum MetaLayerType {
    Mesh,           // Tiles affect both the collision and graphics mesh
    CollisionMesh,  // Tiles affect the collision mesh
    GraphicsMesh,   // Tiles affect the graphics mesh
}

impl MetaLayerType {
    fn parse(str: &str) -> Option<Self> {
        match str {
            ":mesh:"        => Some(Self::Mesh),
            ":cmesh:"       => Some(Self::CollisionMesh),
            ":gmesh:"       => Some(Self::GraphicsMesh),
            _               => None,
        }
    }
}

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
#[derive(Asset, TypePath, Debug, Default)]
pub struct Tileset {    
    pub tileset: tp::Tileset,
    pub base_color: Color,
    pub base_color_texture: Handle<Image>,
    pub emissive: LinearRgba,
    pub emissive_texture: Option<Handle<Image>>,
    pub normal_texture: Option<Handle<Image>>,
}


struct RegularTileLayer {
    region: TileLayerRegion,
    tiles: HashMap<(i32, i32), RegularTile>,
}

impl RegularTileLayer {
    fn parse(
        tile_layer: &tp::TileLayer,
        map: &Map,
        tileset_assets: &Assets<Tileset>,
    ) -> Self {
        let region = tile_layer.region();
        let mut result = Self {
            region,
            tiles: HashMap::new(),
        };
        let (min_x, max_x) = (region.x, region.x + region.width as i32);
        let (min_y, max_y) = (region.y, region.y + region.height as i32);
        for tile_x in min_x..max_x {
            for tile_y in min_y..max_y {
                let tile_gid = tile_layer.gid_at(tile_x, tile_y);
                let Some((tileset_idx, tile_id)) = map.map.tile_location_of(tile_gid) else { continue };
                let tileset_entry = &map.tileset_entries[tileset_idx];
                let tileset = tileset_assets.get(&tileset_entry.tileset).expect("A tileset was not fully loaded");
                let tileset_columns = tileset.tileset.columns();
                let tile_size = IVec2::new(tileset.tileset.tile_width() as i32, tileset.tileset.tile_height() as i32);
                let uv1 = IVec2::new((tile_id % tileset_columns) as i32, (tile_id / tileset_columns) as i32);
                let uv2 = uv1 + IVec2::ONE;
                result.tiles.insert((tile_x, tile_y), RegularTile {
                    tileset_idx,
                    uv1: uv1 * tile_size,
                    uv2: uv2 * tile_size,
                });
            }
        }
        result
    }
}

#[derive(Debug)]
struct RegularTile {
    tileset_idx: usize,
    uv1: IVec2,
    uv2: IVec2,
}


/// Metadata about a group layer.
/// Also contains aggregate metadata about all of tiles across all non-meta tile layers in the group.
#[derive(Default)]
struct GroupMeta {
    lift: f32,
    collision_geoms: HashMap<(i32, i32), TileGeom>,
    graphics_geoms: HashMap<(i32, i32), TileGeom>,
}

/// Information about the 3D geometry of a tile.
/// Used when building the graphics and collision meshes.
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
struct TileGeom {
    shape: TileShape,
    reset: bool,        // Resets strip to ground level. Typically used when the north side is a cliff.
    cliff: Cliff,       // Which sides, if any, should emit cliff geometry.
}

impl TileGeom {
    fn from_tile(tile: &tp::Tile) -> Self {
        let mut result = Self::default();
        for prop in tile.properties() {
            match prop {
                ("shape", PropertyValue::String(shape)) => result.shape = TileShape::parse(shape),
                ("reset", PropertyValue::Bool(reset))   => result.reset = *reset,
                ("cliff", PropertyValue::String(cliff)) => result.cliff = Cliff::parse(cliff),
                ("shape", _) => panic!("Property 'shape' not a string"),
                ("reset", _) => panic!("Property 'reset' not a bool"),
                ("cliff", _) => panic!("Property 'cliff' not a string"),
                _ => {}
            }
        }
        result
    }
}


#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
enum TileShape {
    Wall,
    WallNW,
    WallNE,
    WallFloorSE,
    WallFloorSW,
    #[default]
    Floor,
    FloorNE,
    FloorNW,
    FloorWallSE,
    FloorWallSW,
    FloorSlopeSE,
    FloorSlopeSW,
    Slope,
    SlopeNE,
    SlopeNW,
    SlopeFloorSE,
    SlopeFloorSW,
}

impl TileShape {

    fn quad_info(self) -> QuadInfo {
        match self {
            Self::FloorNE | Self::WallNE | Self::SlopeNE    => QuadInfo::TriangleFlipped,
            Self::FloorNW | Self::WallNW | Self::SlopeNW    => QuadInfo::Triangle,
            _ => QuadInfo::Quad,
        }
    }

    fn parse(shape: &str) -> Self {
        match shape {
            "wall"              => Self::Wall,
            "wall-ne"           => Self::WallNE,
            "wall-nw"           => Self::WallNW,
            "wall-floor-se"     => Self::WallFloorSE,
            "wall-floor-sw"     => Self::WallFloorSW,
            "floor"             => Self::Floor,
            "floor-ne"          => Self::FloorNE,
            "floor-nw"          => Self::FloorNW,
            "floor-wall-se"     => Self::FloorWallSE,
            "floor-wall-sw"     => Self::FloorWallSW,
            "floor-slope-se"    => Self::FloorSlopeSE,
            "floor-slope-sw"    => Self::FloorSlopeSW,
            "slope"             => Self::Slope,
            "slope-ne"          => Self::SlopeNE,
            "slope-nw"          => Self::SlopeNW,
            "slope-floor-se"    => Self::SlopeFloorSE,
            "slope-floor-sw"    => Self::SlopeFloorSW,
            _ => panic!("Invalid tile shape '{shape}'"),
        }
    }
}

/// Which triangles in a tile to include when building a mesh.
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub enum QuadInfo {
    #[default]
    Quad,
    QuadFlipped,
    Triangle,
    TriangleFlipped,
}

bitflags! {
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
    struct Cliff: u8 {
        const NORTH = 0b00000001;
        const EAST  = 0b00000010;
        const WEST  = 0b00000100;
    }
}

impl Cliff {
    fn parse(str: &str) -> Self {
        let mut result = Self::empty();
        if str.contains("n") {
            result |= Self::NORTH;
        }
        if str.contains("e") {
            result |= Self::EAST;
        }
        if str.contains("w") {
            result |= Self::WEST;
        }
        result
    }
}

pub mod messages {

    use bevy::prelude::*;

    #[derive(Event, Copy, Clone, PartialEq, Debug)]
    pub struct SpawnMap {
        pub name: &'static str,
        pub file: &'static str,
        pub position: Vec3,
    }

    #[derive(Event, Copy, Clone, Eq, PartialEq, Debug)]
    pub struct DespawnMap {
        pub name: &'static str,
    }
}