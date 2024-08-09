use bevy::prelude::*;
use messages::SpawnPlayer;
use crate::area::AreaStreamer;

#[derive(Component, Copy, Clone, PartialEq, Debug)]
pub struct Player {
    pub speed: f32,
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
    mut commands: Commands,
) {
    let message = trigger.event();
    commands.spawn((
        Player::default(),
        AreaStreamer {
            size: Vec2::new(256.0, 256.0),
        },
        SpatialBundle {
            transform: Transform::from_translation(message.position),
            ..default()
        }
    ));
}

pub fn move_players(
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
    players: Query<(&Transform, &AreaStreamer), With<Player>>,
) {
    for (transf, streamer) in &players {
        let pos = Vec2::new(transf.translation.x, transf.translation.y - transf.translation.z);
        gizmos.rect_2d(pos, 0.0, streamer.size, Color::WHITE);
    }
}


pub mod messages {

    use bevy::prelude::*;

    #[derive(Event, Copy, Clone, PartialEq, Default, Debug)]
    pub struct SpawnPlayer {
        pub position: Vec3,
    }
}