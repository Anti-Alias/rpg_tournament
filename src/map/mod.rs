mod entities;
mod loader;
mod mesh;

pub use entities::*;
pub use loader::*;

use bevy::math::I16Vec2;
use bevy::math::I16Vec3;
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

pub const TH: i16 = 2;  // Tile height
pub const THH: i16 = 1; // Half tile height


/// Spawns a [`Map`] entity.
/// Map contents load asynchronously.
pub fn spawn_map(
    trigger: Trigger<messages::SpawnMap>,
    mut entities: ResMut<EntityIndex>,
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let message = trigger.event();
    let map_file = &message.file;
    let map_handle: Handle<Map> = assets.load(map_file);
    let map_transf = Transform::from_translation(message.position);
    let map_entity = commands.spawn((
        Name::new(format!("map-chunk-{}", map_file)),
        MapStatus::Loading(map_handle),
        SpatialBundle::from_transform(map_transf)
    )).id();
    log::info!("Spawned map `{map_file}`");
    entities.maps.insert(map_file.clone(), map_entity);
}

/// Despawns a [`Map`].
pub fn despawn_map(
    trigger: Trigger<messages::DespawnMap>,
    mut entities: ResMut<EntityIndex>,
    mut commands: Commands,
) {
    let map_file = &trigger.event().file;
    let map_entity = match entities.maps.remove(map_file) {
        Some(entity) => entity,
        None => panic!("Map '{}' not spawned", map_file),
    };
    commands.entity(map_entity).despawn_recursive();
    log::info!("Despawned map '{map_file}'");
}

/// Monitors loading [`Map`] entities, and finalizes them once they finish loading.
pub fn process_loaded_maps(
    mut commands: Commands,
    mut map_entities: Query<(Entity, &mut MapStatus, &Transform)>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    tileset_assets: Res<Assets<Tileset>>,
    map_assets: Res<Assets<Map>>,
    asset_server: Res<AssetServer>,
) {
    for (map_entity, mut map_status, map_transf) in &mut map_entities {
        let MapStatus::Loading(ref map_handle) = *map_status else {
            continue;
        };
        if asset_server.is_loaded_with_dependencies(map_handle) {
            commands.entity(map_entity).despawn_descendants();
            process_map(
                &mut commands,
                map_entity,
                map_transf.translation,
                &map_handle,
                &map_assets,
                &tileset_assets,
                &mut material_assets,
                &mut mesh_assets,
            );
            *map_status = MapStatus::Loaded;
        }
    }
}

fn process_map(
    commands: &mut Commands,
    map_entity: Entity,
    map_position: Vec3,
    map_handle: &Handle<Map>,
    map_assets: &Assets<Map>,
    tileset_assets: &Assets<Tileset>,
    material_assets: &mut Assets<StandardMaterial>,
    mesh_assets: &mut Assets<Mesh>,
) {
    let mut vert_offset: u16 = 0;
    let map = map_assets.get(map_handle).unwrap();
    for layer in map.map.layers() {
        match layer.kind() {
            tp::LayerKind::GroupLayer(group_layer) => process_group_layer(
                commands,
                group_layer,
                layer.properties(),
                layer.name(),
                map,
                map_entity,
                tileset_assets,
                material_assets,
                mesh_assets,
                &mut vert_offset,
            ),
            tp::LayerKind::ObjectGroupLayer(object_layer) => process_object_layer(
                commands,
                object_layer,
                map.map.tile_height() as f32,
                map.map.tile_height() as f32 * map.map.height() as f32,
                map_position,
            ),
            tp::LayerKind::TileLayer(_) => panic!("Unexpected tile layer"),
            tp::LayerKind::ImageLayer(_) => panic!("Unexpected image layer"),
        }
    }
    log::info!("Finished map");
}

fn process_group_layer(
    commands: &mut Commands,
    group_layer: &tp::GroupLayer,
    group_layer_props: &tp::Properties,
    group_layer_name: &str,
    map: &Map,
    map_entity: Entity,
    tileset_assets: &Assets<Tileset>,
    material_assets: &mut Assets<StandardMaterial>,
    mesh_assets: &mut Assets<Mesh>,
    vert_offset: &mut u16,
) {

    let (regular_layers, group_meta) = parse_group_layer(
        group_layer,
        group_layer_props,
        group_layer_name,
        map,
        &tileset_assets,
    );

    // Forms graphics meshes, parallel with the map's tileset entries.
    let mut cliff_mesh = GraphicsMesh::new();
    let mut gmeshes: Vec<GraphicsMesh> = init_graphics_meshes(map.tileset_entries.len());
    for layer in regular_layers {
        let region = layer.region;
        let (min_x, max_x) = (region.x, region.x + region.width as i32);
        let (min_y, max_y) = (region.y, region.y + region.height as i32);

        // For all columns in regular layer...
        for tile_x in min_x..max_x {
            let tile_x = tile_x as i16;
            let lift = group_meta.lift * TH;

            // Init strip for column
            let mut gstrip = Strip {
                left: I16Vec3::new(tile_x, lift, lift),
                right: I16Vec3::new(tile_x+1, lift, lift),
            };

            // For all tiles in column...
            for tile_y in (min_y..max_y).rev() {
                let tile_y = tile_y as i16;
                let tile_coords = (tile_x, tile_y);

                // Gets tile and tile meta
                let tile = layer.tiles.get(&tile_coords);
                let tile_geom = group_meta.graphics_geoms.get(&tile_coords).copied().unwrap_or_default();
                let tile_quad_info = tile_geom.shape.quad_info();
                let gstrip_next = gstrip.next(tile_geom.shape);

                // Advances strip and generates quad vertices from tile
                if let Some(tile) = tile {
                    let tile_vertices = match tile_geom.shape.is_flipped() {
                        false => GraphicsVertex::quad(
                            [gstrip.left, gstrip.right, gstrip_next.right, gstrip_next.left],
                            [I16Vec2::new(tile.uv1.x, tile.uv2.y), I16Vec2::new(tile.uv2.x, tile.uv2.y), I16Vec2::new(tile.uv2.x, tile.uv1.y), I16Vec2::new(tile.uv1.x, tile.uv1.y)],
                            *vert_offset,
                        ),
                        true => GraphicsVertex::quad(
                            [gstrip.right, gstrip_next.right, gstrip_next.left, gstrip.left],
                            [I16Vec2::new(tile.uv2.x, tile.uv2.y), I16Vec2::new(tile.uv2.x, tile.uv1.y), I16Vec2::new(tile.uv1.x, tile.uv1.y), I16Vec2::new(tile.uv1.x, tile.uv2.y)],
                            *vert_offset,
                        ),
                    };

                    // Pushes quad to relevant mesh
                    let gmesh = &mut gmeshes[tile.tileset_idx];
                    gmesh.push_quad(tile_vertices);
                }

                // Pushes northern cliff vertices
                if tile_geom.cliff.contains(Cliff::NORTH) {
                    let (point_a, point_b) = match tile_quad_info {
                        QuadInfo::Quad | QuadInfo::QuadFlipped  => (gstrip_next.left, gstrip_next.right),
                        QuadInfo::Triangle                      => (gstrip.left, gstrip_next.right),
                        QuadInfo::TriangleFlipped               => (gstrip_next.left, gstrip.right),
                    };
                    let cliff_points = [point_a, point_b, point_b.with_y(lift), point_a.with_y(lift)];
                    let cliff_uvs = [I16Vec2::ZERO; 4];
                    let cliff_verts = GraphicsVertex::quad(cliff_points, cliff_uvs, 0);
                    cliff_mesh.push_quad(cliff_verts);
                }

                // Pushes eastern cliff vertices
                if tile_geom.cliff.contains(Cliff::EAST) {
                    let (point_a, point_b) = (gstrip_next.right, gstrip.right);
                    let cliff_points = [point_b, point_a, point_a.with_y(lift), point_b.with_y(lift)];
                    let cliff_uvs = [I16Vec2::ZERO; 4];
                    let cliff_verts = GraphicsVertex::quad(cliff_points, cliff_uvs, 0);
                    cliff_mesh.push_quad(cliff_verts);
                }

                // Pushes western cliff vertices
                if tile_geom.cliff.contains(Cliff::WEST) {
                    let (point_a, point_b) = (gstrip_next.left, gstrip.left);
                    let cliff_points = [point_a, point_b, point_b.with_y(lift), point_a.with_y(lift)];
                    let cliff_uvs = [I16Vec2::ZERO; 4];
                    let cliff_verts = GraphicsVertex::quad(cliff_points, cliff_uvs, 0);
                    cliff_mesh.push_quad(cliff_verts);
                }

                gstrip = gstrip_next;

                // Resets strip to ground level
                if tile_geom.reset {
                    gstrip.left.z -= gstrip.left.y - lift;
                    gstrip.right.z -= gstrip.right.y - lift;
                    gstrip.left.y = lift;
                    gstrip.right.y = lift;
                }
            }
        }
        *vert_offset += 1;
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
                perceptual_roughness: 1.0,
                reflectance: 0.0,
                alpha_mode: AlphaMode::Mask(0.5),
                double_sided: true,
                cull_mode: None,
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
        commands.entity(map_entity).with_children(|b| {
            b.spawn(PbrBundle {
                mesh: mesh_assets.add(mesh),
                material: mat.material,
                ..default()
            });
        });
    }

    // Spawns cliff mesh
    let cliff_material = material_assets.add(StandardMaterial { base_color: Color::BLACK, unlit: true, ..default() });
    let cliff_mesh = create_bevy_mesh(cliff_mesh, 100.0, 100.0, map.map.tile_width() as f32, map.map.tile_height() as f32);
    commands.entity(map_entity).with_children(|b| {
        b.spawn(PbrBundle {
            mesh: mesh_assets.add(cliff_mesh),
            material: cliff_material,
            ..default()
        });
    });
}


fn process_object_layer(
    commands: &mut Commands,
    object_layer: &tp::ObjectGroupLayer,
    tile_height: f32,
    map_height_px: f32,
    map_position: Vec3,
) {
    for object in object_layer.objects() {
        let props = object.properties();
        for (prop_name, prop_value) in props.iter() {
            match (prop_name, prop_value) {
                ("type", PropertyValue::String(typ)) => {
                    let entity_type = EntityType::parse(typ);
                    spawn_object(
                        commands,
                        object,
                        entity_type,
                        tile_height,
                        map_height_px,
                        map_position
                    );
                }
                ("type", _) => panic!("Property 'type' not a string"),
                _ => {}
            }
        }
    }
}

fn spawn_object(
    commands: &mut Commands,
    object: &tp::Object,
    entity_type: EntityType,
    tile_height: f32,
    map_height_px: f32,
    map_position: Vec3,
) {
    let mut position = map_position + Vec3::new(object.x(), 0.0, object.y() - map_height_px);
    for (prop_name, prop_value) in object.properties() {
        match (prop_name, prop_value) {
            ("lift", PropertyValue::Float(lift))    => { position += Vec3::new(0.0, *lift * tile_height, *lift * tile_height) }
            ("lift", PropertyValue::Int(lift))      => { position += Vec3::new(0.0, *lift as f32 * tile_height, *lift as f32 * tile_height) }
            _ => {}
        }
    }
    commands.trigger(SpawnEntity { entity_type, position });
}

#[derive(Component, Clone, Eq, PartialEq, Debug)]
pub enum MapStatus {
    Loading(Handle<Map>),
    Loaded,
}

// Strip of two points that travels up a vertical column of tiles.
// Two strips define the quad of a tile.
// Used to build collision and graphics meshes.
#[derive(Copy, Clone, Debug)]
struct Strip {
    left: I16Vec3,
    right: I16Vec3,
}

impl Strip {
    fn next(mut self, shape: TileShape) -> Self {
        match shape {
            TileShape::Wall | TileShape::WallNE | TileShape::WallNW     => { self.left.y += TH;     self.right.y += TH; },
            TileShape::WallFloorSE                                      => { self.left.z -= TH;     self.right.y += TH; },
            TileShape::WallFloorSW                                      => { self.left.y += TH;     self.right.z -= TH; },
            TileShape::Floor | TileShape::FloorNE | TileShape::FloorNW  => { self.left.z -= TH;     self.right.z -= TH; },
            TileShape::FloorWallSE                                      => { self.left.y += TH;     self.right.z -= TH; },
            TileShape::FloorWallSW                                      => { self.left.z -= TH;     self.right.y += TH; },
            TileShape::FloorSlopeSE                                     => { self.left.z -= THH;    self.left.y += THH;     self.right.z -= TH; },
            TileShape::FloorSlopeSW                                     => { self.left.z -= TH;     self.right.y += THH;    self.right.z -= THH; },
            TileShape::Slope | TileShape::SlopeNE | TileShape::SlopeNW  => { self.left.z -= THH;    self.left.y += THH;     self.right.z -= THH;    self.right.y += THH; },
            TileShape::SlopeFloorSE                                     => { self.left.z -= THH },
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
            ("lift", PropertyValue::Int(lift)) => group_meta.lift = *lift as i16,
            ("lift", _) => panic!("Property 'lift' not a float"),
            _ => {}
        }
    }
    for layer in group_layer.layers() {
        let tile_layer = match layer.as_tile_layer() {
            Some(tile_layer) => tile_layer,
            None => panic!("Layer '{}/{}' not a tile layer", group_layer_name, layer.name()),
        };
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
            let (x, y) = (x as i16, y as i16);
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

// Material with metadata
struct Mat {
    material: Handle<StandardMaterial>,
    width: u32,
    height: u32,
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
    pub tileset: Handle<Tileset>,
}

/// A set of maps to dynamically load/unload.
#[derive(Asset, TypePath, Deref, Clone, Eq, PartialEq, Debug)]
pub struct Area(pub tiled_parser::World);

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


#[derive(Debug)]
struct RegularTileLayer {
    region: TileLayerRegion,
    tiles: HashMap<(i16, i16), RegularTile>,
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
                let tile_size = I16Vec2::new(tileset.tileset.tile_width() as i16, tileset.tileset.tile_height() as i16);
                let uv1 = I16Vec2::new((tile_id % tileset_columns) as i16, (tile_id / tileset_columns) as i16);
                let uv2 = uv1 + I16Vec2::ONE;
                let (tile_x, tile_y) = (tile_x as i16, tile_y as i16);
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
    uv1: I16Vec2,
    uv2: I16Vec2,
}


/// Metadata about a group layer.
/// Also contains aggregate metadata about all of tiles across all non-meta tile layers in the group.
#[derive(Default)]
struct GroupMeta {
    lift: i16,
    collision_geoms: HashMap<(i16, i16), TileGeom>,
    graphics_geoms: HashMap<(i16, i16), TileGeom>,
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
            Self::FloorNE       => QuadInfo::TriangleFlipped,
            Self::FloorNW       => QuadInfo::Triangle,
            Self::FloorWallSW   => QuadInfo::QuadFlipped,
            Self::FloorSlopeSW  => QuadInfo::QuadFlipped,
            Self::WallFloorSW   => QuadInfo::QuadFlipped,
            Self::WallNE        => QuadInfo::TriangleFlipped,
            Self::WallNW        => QuadInfo::Triangle,
            Self::SlopeNE       => QuadInfo::TriangleFlipped,
            Self::SlopeNW       => QuadInfo::Triangle,
            Self::SlopeFloorSW  => QuadInfo::QuadFlipped,
            _ => QuadInfo::Quad,
        }
    }

    fn is_flipped(self) -> bool {
        match self.quad_info() {
            QuadInfo::QuadFlipped => true,
            QuadInfo::TriangleFlipped => true,
            _ => false,
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

    #[derive(Event, Clone, PartialEq, Debug)]
    pub struct SpawnMap {
        pub file: String,
        pub position: Vec3,
    }

    #[derive(Event, Clone, Eq, PartialEq, Debug)]
    pub struct DespawnMap {
        pub file: String,
    }
}