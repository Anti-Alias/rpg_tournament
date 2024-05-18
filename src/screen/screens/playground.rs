use std::f32::consts;
use bevy::ecs::system::CommandQueue;
use bevy::math::Vec3A;
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy::sprite::Anchor;

use crate::spawn::AssetBatch;
use crate::ext::CommandsExt;
use crate::screen::ScreenEvent;
use crate::sprite::Sprite3DBundle;
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
    let human_mat = materials.add(StandardMaterial {
        base_color_texture: Some(assets.load("human/sheets/a.png")),
        perceptual_roughness: 1.0,
        alpha_mode: AlphaMode::Mask(0.5),
        cull_mode: None,
        double_sided: true,
        ..default()
    });

    // Meshes
    let mut meshes = world.resource_mut::<Assets<Mesh>>();
    let plane_mesh = meshes.add(Mesh::from(Plane3d { normal: Direction3d::Y }));

    // Bundles
    let mut plane = PbrBundle::default();
    plane.material = plane_mat;
    plane.mesh = plane_mesh;
    plane.transform.scale = Vec3::new(100.0, 1.0, 100.0);
    plane.transform.translation.y = -16.0;

    let mut camera = Camera3dBundle::default();
    camera.transform.translation = Vec3::new(0.0, 40.0, 80.0);
    camera.transform.look_at(Vec3::ZERO, Vec3::Y);

    let mut dir_light = DirectionalLightBundle::default();
    dir_light.directional_light.illuminance /= 2.0;
    dir_light.directional_light.shadows_enabled = true;
    dir_light.transform.rotate_y(-consts::PI / 4.0);
    dir_light.transform.rotate_x(-consts::PI / 4.0);

    let mut sprite = Sprite3DBundle::default();
    sprite.sprite.rect = Rect::new(0.0, 0.0, 64.0, 64.0);
    sprite.material = human_mat;

    // Spawn
    let mut commands = Commands::new(commands, world);
    let aabb = Aabb { center: Vec3A::ZERO, half_extents: Vec3A::splat(32.0) };
    commands.spawn((plane, Name::new("Plane")));
    commands.spawn((camera, Name::new("Camera")));
    commands.spawn((dir_light, Name::new("Dir Light")));
    commands.spawn((sprite, aabb, Anchor::Center, Name::new("Sprite")));
}