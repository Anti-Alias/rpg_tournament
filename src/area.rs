use std::f32::consts::PI;
use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::camera::GameCameraBundle;
use crate::daynight::Sunlight;
use crate::map::Area;
use crate::messages::{DespawnMap, SpawnMap};


/// Keeps track of the current map world.
#[derive(Resource, Debug)]
pub struct CurrentArea {
    name: &'static str,
    area: Handle<Area>,
    loaded_maps: HashMap<String, Rect>,
}

impl CurrentArea {

    fn is_touching_rect(&self, rect: Rect) -> bool {
        for map_rect in self.loaded_maps.values() {
            if rects_touching(*map_rect, rect) {
                return true;
            }
        }
        false
    }
}

fn rects_touching(a: Rect, b: Rect) -> bool {
    a.min.x <= b.max.x && a.max.x >= b.min.x &&
    a.min.y <= b.max.y && a.max.y >= b.min.y
}

/// Sets up environment for dynamically loading/unloading maps from a map world.
/// Unloads existing world if already set.
pub fn init_area(
    trigger: Trigger<messages::InitArea>,
    current_area: Option<ResMut<CurrentArea>>,
    assets: Res<AssetServer>,
    mut commands: Commands,
) {
    let message = trigger.event();
    match current_area {
        Some(mut current_area) if message.name != current_area.name => {
            current_area.name = message.name;
            current_area.area = assets.load(message.file);
        },
        None => {

            // Spawns sun
            let mut sun = DirectionalLightBundle::default();
            sun.directional_light.shadows_enabled = true;
            sun.directional_light.illuminance *= 0.5;
            sun.transform.rotate(Quat::from_euler(EulerRot::YXZ, PI/4.0, -PI/4.0, 0.0));
            commands.spawn((Name::new("sun"), sun, Sunlight::default()));
    
            // Spawns camera
            commands.spawn((Name::new("camera"), GameCameraBundle::default()));

            // Configures area, which will stream in maps into the world
            let area = assets.load::<Area>(message.file);
            commands.insert_resource(CurrentArea {
                name: message.name,
                area,
                loaded_maps: HashMap::new(),
            })
        },
        _ => {},
    }
}

/// Forces all spawned maps to reload when the user hits a key combo.
pub fn reload_area(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut current_area: ResMut<CurrentArea>,
    mut commands: Commands,
) {
    if keyboard.just_pressed(KeyCode::F5) {
        let loaded_maps = std::mem::take(&mut current_area.loaded_maps);
        for map_path in loaded_maps.into_keys() {
            commands.trigger(DespawnMap { file: map_path });
        }
    }
}

/// Dynamically loads and unloads world maps based on the current position
/// of all streamers.
pub fn stream_current_area(
    streamers: Query<(&AreaStreamer, &Transform)>,
    current_area: Option<ResMut<CurrentArea>>,
    mut areas: ResMut<Assets<Area>>,
    mut commands: Commands,
) {
    let Some(mut current_area) = current_area else { return };
    let Some(area) = areas.get_mut(&current_area.area) else { return };
    for map_ref in &area.maps {
        for (streamer, transf) in &streamers {

            // Computes streamer bounds
            let stream_pos = Vec2::new(transf.translation.x, transf.translation.y - transf.translation.z);
            let stream_hsize = streamer.size / 2.0;
            let stream_left = stream_pos.x - stream_hsize.x;
            let stream_right = stream_pos.x + stream_hsize.x;
            let stream_top = stream_pos.y + stream_hsize.y;
            let stream_bottom = stream_pos.y - stream_hsize.y;
            
            // Computes map bounds
            let map_left = map_ref.x as f32;
            let map_right = map_left + map_ref.width as f32;
            let map_bottom = -map_ref.y as f32;
            let map_top = map_bottom + map_ref.height as f32;
            let map_path = format!("worlds/{}", map_ref.file_name);

            // Loads / unloads maps
            let streamer_touching_map = stream_left <= map_right && stream_right >= map_left && stream_bottom <= map_top && stream_top >= map_bottom;
            let map_is_loaded = current_area.loaded_maps.contains_key(&map_path);
            match (streamer_touching_map, map_is_loaded) {
                (true, false) => {
                    let map_rect = Rect::new(map_left, map_top, map_right, map_bottom);                    
                    current_area.loaded_maps.insert(map_path.clone(), map_rect);
                    commands.trigger(SpawnMap {
                        file: map_path,
                        position: Vec3::new(map_ref.x as f32, 0.0, map_ref.y as f32)
                    });
                },
                (false, true) => {
                    current_area.loaded_maps.remove(&map_path);
                    commands.trigger(DespawnMap { file: map_path });
                },
                _ => {}
            }
        }
    }
}

/// Despawns entities that belong to no maps in the current area.
/// Generally, ccts as a cleanup system for maps that unload.
pub fn despawn_area_locals(
    current_area: Option<ResMut<CurrentArea>>,
    locals: Query<(Entity, &AreaLocal, &Transform)>,
    mut commands: Commands,
) {
    match current_area {
        Some(current_area) => {
            for (local_e, local, local_transf) in &locals {
                let local_pos = Vec2::new(local_transf.translation.x, local_transf.translation.y - local_transf.translation.z);
                let local_hsize = local.size / 2.0;
                let local_rect = Rect { min: local_pos - local_hsize, max: local_pos + local_hsize};
                if !current_area.is_touching_rect(local_rect) {
                    commands.entity(local_e).despawn_recursive();
                }
            }
        },
        None => {
            for (local_e, _, _) in &locals {
                commands.entity(local_e).despawn_recursive();
            }
        },
    }
}


/// An [`Entity`] that keeps the map it touches loaded.
/// An AABB surrounds a streamer.
/// If this AABB touches a particular map, it will load or remain loaded.
/// If it stops touching a map, it will unload.
#[derive(Component, Copy, Clone, PartialEq, Default, Debug)]
pub struct AreaStreamer {
    pub size: Vec2,
}

/// Any [`Entity`] that should automatically despawned if they are not touching any
/// maps in an area.
#[derive(Component, Copy, Clone, PartialEq, Default, Debug)]
pub struct AreaLocal {
    pub size: Vec2,
}


pub mod messages {

    use bevy::prelude::*;

    #[derive(Event, Copy, Clone, Eq, PartialEq, Default, Debug)]
    pub struct InitArea {
        pub name: &'static str,
        pub file: &'static str,
    }
}