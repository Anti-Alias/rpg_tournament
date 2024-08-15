use bevy::math::Affine3A;
use bevy::prelude::*;


#[derive(Component, Copy, Clone, Default, Debug)]
pub struct Round;

#[derive(Resource, PartialEq, Debug)]
pub struct RoundScale(pub f32);

impl Default for RoundScale {
    fn default() -> Self {
        Self(0.5)
    }
}

pub fn round_positions(
    round_scale: Res<RoundScale>,
    mut entities: Query<&mut GlobalTransform, With<Round>>
) {
    let round_scale = round_scale.0;
    let iround_scale = 1.0 / round_scale;
    for mut global_transform in &mut entities {
        let (scale, rotation, translation) = global_transform.affine().to_scale_rotation_translation();
        let translation = (translation * iround_scale).round() * round_scale;
        let rounded_affine = Affine3A::from_scale_rotation_translation(scale, rotation, translation);
        *global_transform = GlobalTransform::from(rounded_affine);
    }
}