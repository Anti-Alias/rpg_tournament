use std::time::Duration;
use bevy::prelude::*;

use crate::sprite::{batch_sprites, Sprite3D};

pub fn animation_plugin(app: &mut App) {
    app.init_asset::<AnimationSet>();
    app.add_systems(PostUpdate, update_animations.before(batch_sprites));
}

fn update_animations(
    mut anim_states: Query<(&mut AnimationState, &Handle<AnimationSet>, &mut Sprite3D)>,
    animations: Res<Assets<AnimationSet>>,
    time: Res<Time>,
) {
    for (mut anim_state, anim_set_handle, mut sprite) in &mut anim_states {
        
        // Selects animation
        let Some(anim_set) = animations.get(anim_set_handle) else { continue };
        let anim_state = &mut *anim_state;
        let anim_idx = anim_state.animation_index;
        let Some(animation) = anim_set.get(anim_idx) else { continue };

        // Advances animation frame time
        let Some(frame) = animation.frames.get(anim_state.frame_index) else { continue };
        let frame_duration = frame.duration.div_f32(anim_state.speed);
        anim_state.frame_time_elapsed += time.delta();
        loop {
            if anim_state.frame_time_elapsed < frame_duration { break };
            anim_state.frame_time_elapsed -= frame_duration;
            match animation.mode {
                AnimationMode::Forward => anim_state.frame_index = (anim_state.frame_index + 1).min(animation.frames.len() - 1),
                AnimationMode::ForwardLoop { loop_frame } => {
                    anim_state.frame_index += 1;
                    if anim_state.frame_index >= animation.frames.len() {
                        anim_state.frame_index = loop_frame;
                    }
                },
            }
        }

        // Syncs frame data
        let frame = &animation.frames[anim_state.frame_index];
        sprite.rect = frame.region;
    }
}


/// Bundle that represents an entity that can have 0 or more sprite animations
/// manipulating a single [`Sprite3D`].
#[derive(Bundle, Clone, Default, Debug)]
pub struct AnimationBundle {
    pub animations: Handle<AnimationSet>,
    pub animation_state: AnimationState,
    pub sprite: Sprite3D,
    pub material: Handle<StandardMaterial>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

#[derive(Asset, TypePath, Deref, Clone, PartialEq, Default, Debug)]
pub struct AnimationSet(pub Vec<Animation>);

/// A set of animations.
#[derive(Component, Clone, Debug)]
pub struct AnimationState {
    pub animation_index: usize,
    pub frame_index: usize,
    pub frame_time_elapsed: Duration,
    pub speed: f32,
}

impl Default for AnimationState {
    fn default() -> Self {
        Self {
            animation_index: 0,
            frame_index: 0,
            frame_time_elapsed: Duration::ZERO,
            speed: 1.0,
        }
    }
}

/// A single animation in an [`AnimationSet`].
#[derive(Clone, PartialEq, Default, Debug)]
pub struct Animation {
    pub frames: Vec<Frame>,
    pub mode: AnimationMode,
}

impl Animation {

    pub fn from_row(
        start: Vec2,
        sprite_size: Vec2,
        padding: Vec2,
        frame_duration: Duration,
        frame_count: u32,
        image_width: u32,
    ) -> Self {
        let mut frames = vec![];
        let mut pos = start;
        for _ in 0..frame_count {
            frames.push(Frame {
                region: Rect { min: pos, max: pos + sprite_size },
                duration: frame_duration,
            });
            pos.x += sprite_size.x + padding.x;
            if pos.x + sprite_size.x > image_width as f32 {
                pos.x = start.x;
                pos.y += sprite_size.y + padding.y;
            }
        }
        Self { frames, mode: AnimationMode::default() }
    }
}

/// A single frame in an animation.
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Frame {
    pub region: Rect,
    pub duration: Duration,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum AnimationMode {
    Forward,
    ForwardLoop { loop_frame: usize },
}

impl Default for AnimationMode {
    fn default() -> Self {
        Self::ForwardLoop { loop_frame: 0 }
    }
}