use bevy::prelude::*;
use crate::area::AreaLocal;
use crate::common::CommonAssets;

pub fn spawn_water(
    commands: &mut Commands,
    position: Vec3,
    size: Vec3,
    common_assets: &CommonAssets,
    assets: &AssetServer,
) {
    let material = StandardMaterial {
        base_color: LinearRgba::from_u8_array([0, 200, 200, 50]).into(),
        alpha_mode: AlphaMode::Blend,
        perceptual_roughness: 0.0,
        reflectance: 1.0,
        ior: 1.33,
        ..default()
    };
    commands.spawn((
        Name::new("Water"),
        PbrBundle {
            mesh: common_assets.meshes.plane.clone(),
            material: assets.add(material),
            transform: Transform {
                translation: position,
                scale: size,
                ..default()
            },
            ..default()
        },
        AreaLocal { size: Vec2::new(size.x, size.y + size.z) },
    ));
}