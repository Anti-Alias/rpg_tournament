use std::f32::consts::PI;
use bevy::ecs::system::CommandQueue;
use bevy::prelude::*;

use crate::batch::AssetBatch;
use crate::ext::CommandsExt;
use crate::screen::ScreenEvent;
use crate::task::Start;

pub fn setup_playground_screen(mut commands: Commands) {
    commands.spawn_task(Start::new(|_, tq| {
        tq.spawn_batch(spawn_playground);
        tq.send_event(ScreenEvent::FinishedLoading);
    }));
}

fn spawn_playground(world: &mut World, commands: &mut CommandQueue, _assets: &mut AssetBatch) {

    // Materials
    let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
    let plane_mat = materials.add(StandardMaterial {
        base_color: Color::GRAY,
        perceptual_roughness: 1.0,
        ..default()
    });
    let sphere_mat = materials.add(StandardMaterial {
        base_color: Color::RED,
        perceptual_roughness: 1.0,
        ..default()
    });

    // Meshes
    let mut meshes = world.resource_mut::<Assets<Mesh>>();
    let sphere_mesh = meshes.add(Mesh::from(Sphere::default()));
    let plane_mesh = meshes.add(Mesh::from(Plane3d { normal: Direction3d::Y }));

    // Bundles
    let mut sphere = PbrBundle::default();
    sphere.material = sphere_mat;
    sphere.mesh = sphere_mesh;
    sphere.transform.translation.y = 0.2;

    let mut plane = PbrBundle::default();
    plane.material = plane_mat;
    plane.mesh = plane_mesh;
    plane.transform.scale = Vec3::new(10.0, 1.0, 10.0);
    plane.transform.translation.y = -0.5;

    let mut camera = Camera3dBundle::default();
    camera.transform.translation = Vec3::new(0.0, 7.0, 7.0);
    camera.transform.look_at(Vec3::ZERO, Vec3::Y);

    let mut dir_light = DirectionalLightBundle::default();
    dir_light.transform = Transform::from_rotation(Quat::from_rotation_x(-PI / 2.0));
    dir_light.directional_light.illuminance /= 2.0;
    dir_light.directional_light.shadows_enabled = true;

    // Spawn
    let mut commands = Commands::new(commands, world);
    commands.spawn(sphere);
    commands.spawn(plane);
    commands.spawn(camera);
    commands.spawn(dir_light);
}
