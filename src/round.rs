use bevy::math::Affine3A;
use bevy::prelude::*;


#[derive(Component, Copy, Clone, Default, Debug)]
pub struct Round;

pub fn round_positions(mut entities: Query<&mut GlobalTransform, With<Round>>) {
    for mut global_transform in &mut entities {
        let (scale, rotation, translation) = global_transform.affine().to_scale_rotation_translation();
        let rounded_affine = Affine3A::from_scale_rotation_translation(scale, rotation, translation.round());
        *global_transform = GlobalTransform::from(rounded_affine);
    }
}