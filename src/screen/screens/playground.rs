use std::f32::consts;
use bevy::ecs::system::CommandQueue;
use bevy::math::Vec3A;
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy::sprite::Anchor;

use crate::animation::AnimationBundle;
use crate::asset::CommonAssets;
use crate::spawn::AssetBatch;
use crate::ext::CommandsExt;
use crate::screen::ScreenEvent;
use crate::task::Start;


pub fn setup_playground_screen(mut commands: Commands) {
    commands.spawn_task(Start::new(|_, tq| {
        tq.spawn_batch(spawn_playground);
        tq.send_event(ScreenEvent::FinishedLoading);
    }));
}

fn spawn_playground(world: &mut World, commands: &mut CommandQueue, assets: &mut AssetBatch) {

    // Materials
    let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
    let plane_mat = materials.add(StandardMaterial {
        base_color: Color::GRAY,
        ..default()
    });

    // Meshes
    let mut meshes = world.resource_mut::<Assets<Mesh>>();
    let plane_mesh = meshes.add(Mesh::from(Plane3d { normal: Direction3d::Y }));

    // Animations
    let common_assets = world.resource::<CommonAssets>();
    let player_animations = common_assets.player_animations.clone();

    // Bundles
    let mut plane = PbrBundle::default();
    plane.material = plane_mat;
    plane.mesh = plane_mesh;
    plane.transform.scale = Vec3::new(100.0, 1.0, 100.0);
    plane.transform.translation.y = -16.0;

    let mut camera = Camera3dBundle::default();
    camera.transform.translation = Vec3::new(0.0, 100.0, 100.0);
    camera.transform.look_at(Vec3::ZERO, Vec3::Y);

    let mut dir_light = DirectionalLightBundle::default();
    dir_light.directional_light.illuminance /= 2.0;
    dir_light.directional_light.shadows_enabled = true;
    dir_light.transform.rotate_y(-consts::PI / 4.0);
    dir_light.transform.rotate_x(-consts::PI / 4.0);

    let mut player1 = AnimationBundle::default();
    player1.animations = player_animations;
    player1.animation_state.animation_index = 0;
    player1.material = assets.load("human/material.ron.stdmat");

    // Spawn
    let mut commands = Commands::new(commands, world);
    let aabb = Aabb { center: Vec3A::ZERO, half_extents: Vec3A::splat(32.0) };
    commands.spawn((plane, Name::new("Plane")));
    commands.spawn((camera, Name::new("Camera")));
    commands.spawn((dir_light, Name::new("Dir Light")));
    commands.spawn((player1, aabb, Anchor::Center, Name::new("Sprite")));
}


fn spawn_player(commands: &mut Commands, assets: &mut AssetBatch) {

}