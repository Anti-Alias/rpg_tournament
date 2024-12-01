use std::f32::consts::{FRAC_1_SQRT_2, TAU};
use std::time::Duration;

use bevy::input::gamepad::{GamepadConnection, GamepadConnectionEvent, GamepadEvent};
use bevy::prelude::*;
use bevy::sprite::Anchor;


use messages::SpawnPlayer;
use crate::animation::{Animation, AnimationBundle, AnimationSet, AnimationState};
use crate::area::AreaStreamer;
use crate::common::CommonAssets;
use crate::input::{GamepadMapping, KeyboardMapping, StickConfig, StickType, VButtons, VSticks};
use crate::equipment::{Equipment, Hair, HairKind, Outfit};
use crate::messages::ToggleEquipmentMenu;
use crate::round::Round;
use crate::EntityIndex;


#[derive(Bundle, Default, Debug)]
pub struct PlayerBundle {
    pub player: Player,
    pub equipment: Equipment,
    pub character_controller: CharacterController,
    pub vbuttons: VButtons,
    pub vsticks: VSticks,
    pub animation_bundle: AnimationBundle<StandardMaterial>,
    pub area_streamer: AreaStreamer,
    pub round: Round,
}

#[derive(Component, Copy, Clone, PartialEq, Debug)]
pub struct Player {
    /// Direction player visibly faces
    pub card_dir: CardinalDirection,
    /// What the player is currently doing
    pub behavior: PlayerBehavior,
    /// What the player was doing last frame
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

    pub fn speed_friction(&self) -> (f32, f32) {
        let friction = match self.on_ground {
            true => self.ground_friction,
            false => self.air_friction,
        };
        let speed = self.top_speed/friction - self.top_speed;
        (speed, friction)
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

impl Default for Player {
    fn default() -> Self {
        Self {
            behavior: PlayerBehavior::default(),
            behavior_prev: PlayerBehavior::default(),
            card_dir: CardinalDirection::default(),
        }
    }
}

pub fn spawn_player(
    trigger: Trigger<SpawnPlayer>,
    common_assets: Res<CommonAssets>,
//    gamepads: Res<Gamepads>,
    mut entity_index: ResMut<EntityIndex>,
    mut commands: Commands,
) {

    let player_hair = Hair {
        kind: HairKind::Ponytail,
        color: Color::linear_rgb(1.0, 1.0, 0.0),
        brightness: 1.0
    }.into();

    let mut player_bundle = PlayerBundle::default();
    player_bundle.equipment.hair = Some(player_hair);
    player_bundle.equipment.outfit = Some(Outfit::Casual1.into());
    player_bundle.area_streamer =  AreaStreamer { size: Vec2::splat(32.0 * 40.0) };
    player_bundle.vsticks = VSticks::new(2);
    player_bundle.animation_bundle.animation_set = common_assets.animations.player.clone();
    player_bundle.animation_bundle.animation_state = AnimationState { animation_idx: animations::WALK_BASE, ..default() };
    player_bundle.animation_bundle.material = common_assets.materials.player.clone();

    let player_id = commands
        .spawn(player_bundle)
        .insert((
            Name::new("player"),
            Transform::from_translation(trigger.event().position),
            KeyboardMapping::from([
                (KeyCode::ArrowLeft,    buttons::LEFT),
                (KeyCode::ArrowRight,   buttons::RIGHT),
                (KeyCode::ArrowUp,      buttons::UP),
                (KeyCode::ArrowDown,    buttons::DOWN),
                (KeyCode::Enter,        buttons::START),
            ]),
        ))
        .id();
    if let Some(first_gamepad) = gamepads.iter().next() {
        let mapping = create_gamepad_mapping(first_gamepad);
        commands.entity(player_id).insert(mapping);
    }
    entity_index.player = Some(player_id);
}


/// Inserts / removes a gamepad mapping to the player whenever a gamepad connects disconnects.
/// Does nothing if already inserted.
pub fn assign_gamepad_to_player(
    mut events: EventReader<GamepadEvent>,
    mut players: Query<Entity, With<Player>>,
    //gamepads: Res<Gamepads>,
    mut commands: Commands,
) {
    for event in events.read() {
        match event {
            GamepadEvent::Connection(GamepadConnectionEvent { gamepad, connection: GamepadConnection::Connected(_) }) => {
                let mapping = create_gamepad_mapping(gamepad.clone());
                for player_id in &mut players {
                    commands.entity(player_id).insert(mapping.clone());
                }
            },
            GamepadEvent::Connection(GamepadConnectionEvent { connection: GamepadConnection::Disconnected, .. }) => {
                if gamepads.iter().count() != 0 { continue }    // Only remove if it was the last gamepad to disconnect
                for player_id in &mut players {
                    commands.entity(player_id).remove::<GamepadMapping>();
                }   
            },
            _ => {}
        }
    }
}

pub fn update_players(
    mut commands: Commands,
    mut players: Query<(
        &mut Player,
        &mut CharacterController,
        &VButtons,
        &VSticks
    )>
) {
    for (mut player, mut cc, buttons, sticks) in &mut players {

        // Determines player's travel direction.
        // Determines player's cardinal direction.
        // Uses either dpad or stick.
        let mut direction = Vec3::ZERO;
        let using_dpad = buttons.pressed(buttons::LEFT | buttons::RIGHT | buttons::UP | buttons::DOWN);
        if using_dpad {
            const DIAG: f32 = FRAC_1_SQRT_2;
            let (x, y) = xy_from_dpad(buttons.pressed);
            let (prev_x, prev_y) = xy_from_dpad(buttons.pressed_prev);
            direction = match (x, y) {
                (1, 0) => Vec3::new(1.0, 0.0, 0.0),
                (1, 1) => Vec3::new(DIAG, 0.0, -DIAG),
                (0, 1) => Vec3::new(0.0, 0.0, -1.0),
                (-1, 1) => Vec3::new(-DIAG, 0.0, -DIAG),
                (-1, 0) => Vec3::new(-1.0, 0.0, 0.0),
                (-1, -1) => Vec3::new(-DIAG, 0.0, DIAG),
                (0, -1) => Vec3::new(0.0, 0.0, 1.0),
                (1, -1) => Vec3::new(DIAG, 0.0, DIAG),
                _ => Vec3::ZERO,
            };
            if let Some(card_dir) = card_dir_from_xy(x, y, prev_x, prev_y) {
                player.card_dir = card_dir;
            }
        }
        else {
            let stick = sticks.get(sticks::LEFT).unwrap();
            let stick = Vec3::new(stick.x, 0.0, -stick.y);
            direction = (direction + stick).clamp_length_max(1.0);
            if let Some(card_dir) = CardinalDirection::from_vec2(Vec2::new(direction.x, -direction.z)) {
                player.card_dir = card_dir;
            }
        }

        // Applies direction to velocity
        let (cc_speed, cc_friction) = cc.speed_friction();
        cc.velocity += direction * cc_speed;
        cc.velocity *= cc_friction;
        let mut is_moving = true;
        if cc.velocity.length_squared() < 0.01 {
            cc.velocity = Vec3::ZERO;
            is_moving = false;
        } // Updates behavior
        player.behavior = match player.behavior {
            PlayerBehavior::Idle => {
                match is_moving {
                    false => PlayerBehavior::Idle,
                    true => PlayerBehavior::Walking,
                }
            },
            PlayerBehavior::Walking => {
                match is_moving {
                    false => PlayerBehavior::Idle,
                    true => PlayerBehavior::Walking,
                }
            },
        };
2
        // Opens equipment menu
        if buttons.just_pressed(buttons::START) {
            commands.trigger(ToggleEquipmentMenu);
        }
    }
}

fn xy_from_dpad(button_bits: u32) -> (i32, i32) {
    let (mut x, mut y) = (0, 0);
    if button_bits & buttons::LEFT != 0   { x -= 1; }
    if button_bits & buttons::RIGHT != 0  { x += 1; }
    if button_bits & buttons::UP != 0     { y += 1; }
    if button_bits & buttons::DOWN != 0   { y -= 1; }
    (x, y)
}

fn card_dir_from_xy(x: i32, y: i32, prev_x: i32, prev_y: i32) -> Option<CardinalDirection> {
    if x == 0 && y == 0 { return None }
    match (x, y) {
        (0, 1)  => return Some(CardinalDirection::North),
        (0, -1) => return Some(CardinalDirection::South),
        (-1, 0) => return Some(CardinalDirection::West),
        (1, 0)  => return Some(CardinalDirection::East),
        _ => {}
    }
    match (x, prev_y, y) {
        (1, 0, 1)   => return Some(CardinalDirection::North),   // RIGHT, recent UP
        (1, 0, -1)  => return Some(CardinalDirection::South),   // RIGHT, recent DOWN
        (-1, 0, 1)  => return Some(CardinalDirection::North),   // LEFT, recent UP
        (-1, 0, -1) => return Some(CardinalDirection::South),   // LEFT, recent DOWN
        _ => {}
    }
    match (y, prev_x, x) {
        (1, 0, -1)  => return Some(CardinalDirection::West),   // UP, recent LEFT
        (1, 0, 1)   => return Some(CardinalDirection::East),   // UP, recent RIGHT
        (-1, 0, -1) => return Some(CardinalDirection::West),   // DOWN, recent LEFT
        (-1, 0, 1)  => return Some(CardinalDirection::East),   // DOWN, recent RIGHT
        _ => {}
    }
    None
}

pub fn update_character_controllers(mut controllers: Query<(&CharacterController, &mut Transform)>) {
    for (cc, mut transf) in &mut controllers {
        transf.translation += cc.velocity;
    }
}

pub fn update_player_animations(mut players: Query<(&Player, &mut AnimationState)>) {
    for (player, mut player_anims) in &mut players {
        
        // Handle behavior / direction change
        match (player.behavior_prev, player.behavior, player.card_dir) {
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

fn create_gamepad_mapping(gamepad: Gamepad) -> GamepadMapping {
    GamepadMapping::new(gamepad)
        .with_button(GamepadButtonType::DPadLeft, buttons::LEFT)
        .with_button(GamepadButtonType::DPadRight, buttons::RIGHT)
        .with_button(GamepadButtonType::DPadUp, buttons::UP)
        .with_button(GamepadButtonType::DPadDown, buttons::DOWN)
        .with_button(GamepadButtonType::Start, buttons::START)
        .with_stick(StickType::Left, StickConfig { vstick_idx: sticks::LEFT, deadzones: Vec2::new(0.125, 0.125) })
}

/// Utility function to create the animations a player uses.
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

/// Base indices of player animations.
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
    pub const START: u32    = 1 << 4;
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
