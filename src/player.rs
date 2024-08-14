use bevy::color::palettes::css::WHITE;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_mod_sprite3d::*;
use messages::SpawnPlayer;
use crate::area::AreaStreamer;
use crate::pixel::Round;
use crate::EntityIndex;

const PLAYER_DRAW_SIZE: Vec3 = Vec3::new(16.0, 16.0, 16.0);

#[derive(Component, Copy, Clone, PartialEq, Debug)]
pub struct Player { pub speed: f32 }

#[derive(Bundle, Default, Debug)]
pub struct PlayerBundle {
    pub player: Player,
    pub sprite_3d_bundle: Sprite3dBundle<StandardMaterial>,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 256.0
        }
    }
}

pub fn spawn_player(
    trigger: Trigger<SpawnPlayer>,
    mut entity_index: ResMut<EntityIndex>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let message = trigger.event();
    let player_tex = assets.load::<Image>("player/base/light_walk.png");
    let player_mat = materials.add(StandardMaterial {
        base_color_texture: Some(player_tex),
        reflectance: 0.0,
        perceptual_roughness: 1.0,
        cull_mode: None,
        //alpha_mode: AlphaMode::Mask(0.5),
        double_sided: true,
        ..default()
    });
    
    // Spawns player
    let mut player_sprite = Sprite3dBundle::<StandardMaterial>::default();
    player_sprite.sprite3d.rect = Some(Rect::new(0.0, 0.0, 64.0, 64.0));
    player_sprite.sprite3d.anchor = Anchor::Custom(Vec2::new(0.0, -0.2));
    player_sprite.transform  = Transform::from_translation(message.position);
    player_sprite.material = player_mat;
    let player_id = commands.spawn((
        Name::new("player"),
        Player::default(),
        AreaStreamer { size: Vec2::splat(32.0 * 40.0) },
        player_sprite,
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

pub fn draw_players(
    mut gizmos: Gizmos,
    players: Query<&Transform, With<Player>>,
) {
    for transf in &players {
        let offset = Vec3::new(0.0, PLAYER_DRAW_SIZE.y / 2.0, 0.0);
        let transform = Transform {
            translation: transf.translation + offset,
            rotation: Quat::IDENTITY,
            scale: PLAYER_DRAW_SIZE,
        };
        gizmos.cuboid(transform, Color::WHITE);
    }
}


pub mod messages {

    use bevy::prelude::*;

    #[derive(Event, Copy, Clone, PartialEq, Default, Debug)]
    pub struct SpawnPlayer {
        pub position: Vec3,
    }
}