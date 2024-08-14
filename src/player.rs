use std::time::Duration;

use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_mod_sprite3d::*;
use messages::SpawnPlayer;
use crate::animation::{Animation, AnimationBundle, AnimationSet, AnimationState};
use crate::area::AreaStreamer;
use crate::common::CommonAssets;
use crate::round::Round;
use crate::EntityIndex;

const ANIM_WALK_OFFSET: usize = 4;

#[derive(Component, Copy, Clone, PartialEq, Debug)]
pub struct Player { pub speed: f32 }

#[derive(Bundle, Default, Debug)]
pub struct PlayerBundle {
    pub player: Player,
    pub sprite_3d_bundle: Sprite3dBundle<StandardMaterial>,
}

impl Default for Player {
    fn default() -> Self {
        Self { speed: 256.0 }
    }
}

pub fn spawn_player(
    trigger: Trigger<SpawnPlayer>,
    mut entity_index: ResMut<EntityIndex>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    common_assets: Res<CommonAssets>,
) {
    let message = trigger.event();
    let player_tex = asset_server.load::<Image>("player/base/light_walk.png");
    let player_mat = materials.add(StandardMaterial {
        base_color_texture: Some(player_tex),
        reflectance: 0.0,
        perceptual_roughness: 1.0,
        cull_mode: None,
        alpha_mode: AlphaMode::Mask(0.5),
        double_sided: true,
        ..default()
    });
    
    // Spawns player
    let player_id = commands.spawn((
        Name::new("player"),
        Player::default(),
        AnimationBundle::<StandardMaterial> {
            animation_set: common_assets.animations.player.clone(),
            animation_state: AnimationState {
                animation_idx: ANIM_WALK_OFFSET,
                ..default()
            },
            material: player_mat,
            transform: Transform::from_translation(message.position),
            ..default()
        },
        Sprite::default(),
        AreaStreamer { size: Vec2::splat(32.0 * 40.0) },
        Round,
    )).id();
    entity_index.player = Some(player_id);
}

pub fn update_players(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut players: Query<(&Player, &mut Transform)>,
    time: Res<Time>,
) {
    for (player, mut transf) in &mut players {
        let mut direction = Vec3::ZERO;
        if keyboard.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }
        if keyboard.pressed(KeyCode::ArrowUp) {
            direction.z -= 1.0;
        }
        if keyboard.pressed(KeyCode::ArrowDown) {
            direction.z += 1.0;
        }
        let direction = direction.normalize_or_zero();
        transf.translation += direction * player.speed * time.delta_seconds();
    }
}

pub(crate) fn create_player_animations() -> AnimationSet {
    const SIZE: Vec2 = Vec2::new(64.0, 64.0);
    const STRIDE: Vec2 = Vec2::new(64.0, 0.0);
    const DURATION: Duration = Duration::from_millis(100);
    const ANCHOR: Anchor = Anchor::Custom(Vec2::new(0.0, -0.19));

    let idle_s = Animation::EMPTY.with_frames(1, Vec2::new(0.0, 0.0)*SIZE, SIZE, STRIDE, DURATION, ANCHOR);
    let idle_n = Animation::EMPTY.with_frames(1, Vec2::new(0.0, 1.0)*SIZE, SIZE, STRIDE, DURATION, ANCHOR);
    let idle_e = Animation::EMPTY.with_frames(1, Vec2::new(0.0, 2.0)*SIZE, SIZE, STRIDE, DURATION, ANCHOR);
    let idle_w = Animation::EMPTY.with_frames(1, Vec2::new(0.0, 3.0)*SIZE, SIZE, STRIDE, DURATION, ANCHOR);

    let walk_s = Animation::EMPTY.with_frames(6, Vec2::new(0.0, 4.0)*SIZE, SIZE, STRIDE, DURATION, ANCHOR);
    let walk_n = Animation::EMPTY.with_frames(6, Vec2::new(0.0, 5.0)*SIZE, SIZE, STRIDE, DURATION, ANCHOR);
    let walk_e = Animation::EMPTY.with_frames(6, Vec2::new(0.0, 6.0)*SIZE, SIZE, STRIDE, DURATION, ANCHOR);
    let walk_w = Animation::EMPTY.with_frames(6, Vec2::new(0.0, 7.0)*SIZE, SIZE, STRIDE, DURATION, ANCHOR);

    AnimationSet::EMPTY.with_animations([
        idle_s, idle_n, idle_e, idle_w,
        walk_s, walk_n, walk_e, walk_w,
    ])
}

pub mod messages {
    use bevy::prelude::*;

    #[derive(Event, Copy, Clone, PartialEq, Default, Debug)]
    pub struct SpawnPlayer {
        pub position: Vec3,
    }
}