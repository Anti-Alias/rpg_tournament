use std::f32::consts::PI;
use bevy::prelude::*;

pub fn setup_playground_screen(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut plane_mat = StandardMaterial::from(Color::GRAY);
    plane_mat.perceptual_roughness = 1.0;

    let mut sphere_mat = StandardMaterial::from(Color::RED);
    sphere_mat.perceptual_roughness = 1.0;

    let mut sphere = PbrBundle::default();
    sphere.material = materials.add(sphere_mat);
    sphere.mesh = meshes.add(Mesh::from(Sphere::default()));
    sphere.transform.translation.y = 0.2;
    commands.spawn(sphere);

    let mut plane = PbrBundle::default();
    plane.material = materials.add(plane_mat);
    plane.mesh = meshes.add(Mesh::from(Plane3d { normal: Direction3d::Y }));
    plane.transform.scale = Vec3::new(10.0, 1.0, 10.0);
    plane.transform.translation.y = -0.5;
    commands.spawn(plane);

    let mut camera = Camera3dBundle::default();
    camera.transform.translation = Vec3::new(0.0, 7.0, 7.0);
    camera.transform.look_at(Vec3::ZERO, Vec3::Y);
    commands.spawn(camera);

    let mut dir_light = DirectionalLightBundle::default();
    dir_light.transform = Transform::from_rotation(Quat::from_rotation_x(-PI / 2.0));
    dir_light.directional_light.illuminance /= 2.0;
    dir_light.directional_light.shadows_enabled = true;
    commands.spawn(dir_light);
}
