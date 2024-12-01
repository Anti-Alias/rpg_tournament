use bevy::prelude::*;
use crate::common::CommonAssets;
use crate::daynight::GameTime;


pub fn spawn_entity(
    trigger: Trigger<SpawnEntity>,
    common_assets: Res<CommonAssets>,
    assets: Res<AssetServer>,
    game_time: Res<GameTime>,
    mut commands: Commands,
) {
    let message = trigger.event();
    match message.entity_type {
        EntityType::Firefly => crate::mobs::spawn_firefly(&mut commands, message.position, &common_assets, game_time.time_fraction()),
        EntityType::Water   => crate::objects::spawn_water(&mut commands, message.position, message.size, &common_assets, &assets),
    }
}


#[derive(Event, Copy, Clone, PartialEq, Debug)]
pub struct SpawnEntity {
    pub entity_type: EntityType,
    pub position: Vec3,
    pub size: Vec3,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum EntityType { Firefly, Water }

impl EntityType {
    pub fn parse(entity_type: &str) -> Option<Self> {
        match entity_type {
            "firefly"   => Some(Self::Firefly),
            "water"     => Some(Self::Water),
            _           => None,
        }
    }
}
