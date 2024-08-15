use std::f32::consts::TAU;
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
pub struct Player {
    pub direction: CardinalDirection,
    pub behavior: PlayerBehavior,
    pub behavior_prev: PlayerBehavior,
}

#[derive(Component, Copy, Clone, PartialEq, Debug)]
pub struct CharacterController {
    pub top_speed: f32,
    pub velocity: Vec3,
    pub ground_friction: f32,
    pub air_friction: f32,
    pub on_ground: bool,
}

impl CharacterController {
    pub fn speed(&self) -> f32 {
        let friction = match self.on_ground {
            true => self.ground_friction,
            false => self.air_friction,
        };
        self.top_speed/friction - self.top_speed
    }
}

impl Default for CharacterController {
    fn default() -> Self {
         Self {
            top_speed: 2.0,
            velocity: Vec3::ZERO,
            ground_friction: 0.5,
            air_friction: 0.95,
            on_ground: true,
        }
    }
}

/// Current "thing" player is doing.
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub enum PlayerBehavior {
    #[default]
    Idle,
    Walking,
}

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub enum CardinalDirection {
    #[default]
    South = 0,
    North = 1,
    East = 2,
    West = 3
}

impl CardinalDirection {
    
    pub fn from_vec2(vec: Vec2) -> Option<Self> {
        const SLICE: f32 = TAU / 8.0;
        if vec.length_squared() < 0.01 { return None }
        let dir = if vec.y > 0.0 {
            let angle = vec.to_angle();
            if angle <= 1.0*SLICE       { Self::East }
            else if angle <= 3.0*SLICE  { Self::North }
            else if angle <= 5.0*SLICE  { Self::West }
            else if angle <= 7.0*SLICE  { Self::South }
            else                        { Self::East }
        }
        else {
            let angle = Vec2::new(vec.x, -vec.y).to_angle();
            if angle <= 1.0*SLICE       { Self::East }
            else if angle <= 3.0*SLICE  { Self::South }
            else if angle <= 5.0*SLICE  { Self::West }
            else if angle <= 7.0*SLICE  { Self::North }
            else                        { Self::East }
        };
        Some(dir)
    }
}

#[derive(Bundle, Default, Debug)]
pub struct PlayerBundle {
    pub player: Player,
    pub character_controller: CharacterController,
    pub vbuttons: VButtons,
    pub vsticks: VSticks,
    pub animation_bundle: AnimationBundle<StandardMaterial>,
    pub area_streamer: AreaStreamer,
    pub round: Round,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            behavior: PlayerBehavior::default(),
            behavior_prev: PlayerBehavior::default(),
            direction: CardinalDirection::default(),
        }
    }
}

pub fn spawn_player(
    trigger: Trigger<SpawnPlayer>,
    common_assets: Res<CommonAssets>,
    mut entity_index: ResMut<EntityIndex>,
    mut commands: Commands,
) {

    let mut bundle = PlayerBundle::default();
    bundle.area_streamer =  AreaStreamer { size: Vec2::splat(32.0 * 40.0) };
    bundle.vsticks = VSticks::new(2);
    bundle.animation_bundle.animation_set = common_assets.animations.player.clone();
    bundle.animation_bundle.animation_state = AnimationState { animation_idx: ANIM_WALK_OFFSET, ..default() };
    bundle.animation_bundle.material = common_assets.materials.player.clone();

    let player_id = commands
        .spawn(bundle)
        .insert(Name::new("player"))
        .insert(Transform::from_translation(trigger.event().position))
        .insert(KeyboardMapping::from([
            (KeyCode::ArrowLeft,    buttons::LEFT),
            (KeyCode::ArrowRight,   buttons::RIGHT),
            (KeyCode::ArrowUp,      buttons::UP),
            (KeyCode::ArrowDown,    buttons::DOWN),
        ]))
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
                    .with_stick(StickType::Left, StickConfig { vstick_idx: sticks::LEFT, deadzones: Vec2::new(0.125, 0.125) });
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

pub fn update_players(mut players: Query<(
    &mut Player,
    &mut CharacterController,
    &VButtons,
    &VSticks)>
) {
    for (mut player, mut cc, buttons, sticks) in &mut players {

        // Controls running direction
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

        // Applies direction to velocity
        let cc = &mut *cc;
        cc.velocity += direction * cc.speed();
        cc.velocity *= cc.ground_friction;

        // Sets direction baed on velocity
        if let Some(direction) = CardinalDirection::from_vec2(Vec2::new(direction.x, -direction.z)) {
            player.direction = direction;
        }

        // Updates behavior
        let requests_movement = direction.length_squared() > 0.0;
        player.behavior = match player.behavior {
            PlayerBehavior::Idle => {
                match requests_movement {
                    false => PlayerBehavior::Idle,
                    true => PlayerBehavior::Walking,
                }
            },
            PlayerBehavior::Walking => {
                let is_moving = cc.velocity.length_squared() > 0.1;
                match is_moving || requests_movement {
                    false => PlayerBehavior::Idle,
                    true => PlayerBehavior::Walking,
                }
            },
        };
    }
}

pub fn update_character_controllers(mut controllers: Query<(&CharacterController, &mut Transform)>) {
    for (cc, mut transf) in &mut controllers {
        transf.translation += cc.velocity;
    }
}

pub fn update_player_animations(mut players: Query<(&Player, &mut AnimationState)>) {
    for (player, mut player_anims) in &mut players {
        
        // Handle behavior / direction change
        match (player.behavior_prev, player.behavior, player.direction) {
            (PlayerBehavior::Idle, PlayerBehavior::Walking, direction) => {
                player_anims.animation_idx = animations::WALK_BASE + direction as usize;
                player_anims.frame_idx = 0;
                player_anims.frame_elapsed = Duration::ZERO;
            },
            (PlayerBehavior::Walking, PlayerBehavior::Idle, direction) => {
                player_anims.animation_idx = animations::IDLE_BASE + direction as usize;
                player_anims.frame_idx = 0;
                player_anims.frame_elapsed = Duration::ZERO;
            },
            (_, PlayerBehavior::Idle, direction) => {
                player_anims.animation_idx = animations::IDLE_BASE + direction as usize;
            },
            (_, PlayerBehavior::Walking, direction) => {
                player_anims.animation_idx = animations::WALK_BASE + direction as usize;
            },
        }
    }
}

pub fn sync_players(mut players: Query<&mut Player>) {
    for mut player in &mut players {
        player.behavior_prev = player.behavior;
    }
}


pub(crate) fn create_material(assets: &AssetServer, path: &'static str) -> StandardMaterial {
    StandardMaterial {
        base_color_texture: Some(assets.load::<Image>(path)),
        reflectance: 0.0,
        perceptual_roughness: 1.0,
        cull_mode: None,
        alpha_mode: AlphaMode::Mask(0.5),
        double_sided: false,
        ..default()
    }
}

/// Utility function to create the animations a player uses
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

pub mod animations {
    pub const IDLE_BASE: usize = 0;
    pub const WALK_BASE: usize = 4;
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
    pub struct SpawnPlayer { pub position: Vec3 }
}