use bevy::prelude::*;


/// Entities with this component will have their translations rounded to the nearest unit.
/// After transform propagation, the original translation will be restored.
#[derive(Component, Copy, Clone, Default, Debug)]
pub struct Round(Vec3);

#[derive(Resource, PartialEq, Debug)]
pub struct RoundUnitSize(pub f32);

impl Default for RoundUnitSize {
    fn default() -> Self {
        Self(1.0)
    }
}

pub fn round_translations(
    round_scale: Res<RoundUnitSize>,
    mut entities: Query<(&mut Transform, &mut Round)>
) {
    let round_scale = round_scale.0;
    let iround_scale = 1.0 / round_scale;
    for (mut transf, mut round) in &mut entities {
        round.0 = transf.translation;
        transf.translation = (transf.translation * iround_scale).round() * round_scale;
    }
}

pub fn restore_translations(mut entities: Query<(&mut Transform, &Round)>) {
    for (mut transf, round) in &mut entities {
        transf.translation = round.0;
    }
}