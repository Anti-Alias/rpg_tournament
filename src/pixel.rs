use bevy::math::Affine3A;
use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct PixelPlugin;
impl Plugin for PixelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, round_translations.after(TransformSystem::TransformPropagate));
    }
}


#[derive(Component, Copy, Clone, Default, Debug)]
pub struct Round;


fn round_translations(mut entities: Query<&mut GlobalTransform, With<Round>>) {
    for mut global_transform in &mut entities {
        let (scale, rotation, translation) = global_transform.affine().to_scale_rotation_translation();
        let rounded_affine = Affine3A::from_scale_rotation_translation(scale, rotation, translation.round());
        *global_transform = GlobalTransform::from(rounded_affine);
    }
}