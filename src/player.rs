use std::time::Duration;

use bevy::input::gamepad::{GamepadConnection, GamepadConnectionEvent, GamepadEvent};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use messages::SpawnPlayer;
use crate::animation::{Animation, AnimationBundle, AnimationSet, AnimationState};
use crate::area::AreaStreamer;
use crate::common::CommonAssets;
use crate::input::{GamepadMapping, KeyboardMapping, StickConfig, StickType, VButtons, VSticks};
use crate::round::Round;
use crate::EntityIndex;

const ANIM_WALK_OFFSET: usize = 4;

#[derive(Component, Copy, Clone, PartialEq, Debug)]
pub struct Player { pub speed: f32 }

#[derive(Bundle, Default, Debug)]
pub struct PlayerBundle {
    pub player: Player,
    pub vbuttons: VButtons,
    pub vsticks: VSticks,
    pub animation_bundle: AnimationBundle<StandardMaterial>,
    pub area_streamer: AreaStreamer,
    pub round: Round,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 2.0,
        }
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
    let player_id = commands
        .spawn(PlayerBundle {
            area_streamer: AreaStreamer { size: Vec2::splat(32.0 * 40.0) },
            vsticks: VSticks::new(2),
            animation_bundle: AnimationBundle {
                animation_set: common_assets.animations.player.clone(),
                animation_state: AnimationState {
                    animation_idx: ANIM_WALK_OFFSET,
                    ..default()
                },
                material: player_mat,
                transform: Transform::from_translation(message.position),
                ..default()
            },
            ..default()
        })
        .insert(KeyboardMapping::from([
            (KeyCode::ArrowLeft,    buttons::LEFT),
            (KeyCode::ArrowRight,   buttons::RIGHT),
            (KeyCode::ArrowUp,      buttons::UP),
            (KeyCode::ArrowDown,    buttons::DOWN),
        ]))
        .insert(Name::new("player"))
        .id();
    entity_index.player = Some(player_id);
}


/// When the first gamepad connects, assign it to a [`Player`] entity.
/// When the first gamepad disconnects, unassign it from the [`Player`] entity.
pub fn assign_gamepad_to_player(
    mut events: EventReader<GamepadEvent>,
    mut players: Query<Entity, With<Player>>,
    gamepads: Res<Gamepads>,
    mut commands: Commands,
) {
    for event in events.read() {
        match event {
            GamepadEvent::Connection(GamepadConnectionEvent { gamepad, connection: GamepadConnection::Connected(_) }) => {
                if gamepads.iter().count() != 1 { break }
                let mapping = GamepadMapping::new(gamepad.clone())
                    .with_button(GamepadButtonType::DPadLeft, buttons::LEFT)
                    .with_button(GamepadButtonType::DPadRight, buttons::RIGHT)
                    .with_button(GamepadButtonType::DPadUp, buttons::UP)
                    .with_button(GamepadButtonType::DPadDown, buttons::DOWN)
                    .with_stick(StickType::Left, StickConfig { vstick_idx: sticks::LEFT, deadzones: Vec2::new(0.15, 0.15) });
                for player_id in &mut players {
                    commands.entity(player_id).insert(mapping.clone());
                }
            },
            GamepadEvent::Connection(GamepadConnectionEvent { connection: GamepadConnection::Disconnected, .. }) => {
                if gamepads.iter().count() != 0 { break }
                for player_id in &mut players {
                    commands.entity(player_id).remove::<GamepadMapping>();
                }   
            },
            _ => {}
        }
    }
}

pub fn update_players(mut players: Query<(&Player, &mut Transform, &VButtons, &VSticks)>) {
    for (player, mut transf, buttons, sticks) in &mut players {
        let lstick = sticks.get(sticks::LEFT).unwrap();
        let lstick = Vec3::new(lstick.x, 0.0, -lstick.y);
        let mut direction = Vec3::ZERO;
        if buttons.pressed(buttons::LEFT) { direction.x -= 1.0; }
        if buttons.pressed(buttons::RIGHT) { direction.x += 1.0; }
        if buttons.pressed(buttons::UP) { direction.z -= 1.0; }
        if buttons.pressed(buttons::DOWN) { direction.z += 1.0; }
        direction += lstick;
        if direction.length_squared() > 1.0 {
            direction = direction.normalize_or_zero();
        }
        transf.translation += direction * player.speed;
    }
}

pub(crate) fn create_player_animations() -> AnimationSet {

    const SIZE: Vec2 = Vec2::new(64.0, 64.0);
    const STRIDE: Vec2 = Vec2::new(64.0, 0.0);
    const DURATION: Duration = Duration::from_millis(100);
    const ANCHOR: Anchor = Anchor::Custom(Vec2::new(0.0, -0.21));

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

/// Virtual player buttons
pub mod buttons {
    pub const LEFT: u32     = 1 << 0;
    pub const RIGHT: u32    = 1 << 1;
    pub const UP: u32       = 1 << 2;
    pub const DOWN: u32     = 1 << 3;
}

/// Stick index
pub mod sticks {
    pub const LEFT: usize = 0;
}

pub mod messages {
    use bevy::prelude::*;

    #[derive(Event, Copy, Clone, PartialEq, Default, Debug)]
    pub struct SpawnPlayer {
        pub position: Vec3,
    }
}