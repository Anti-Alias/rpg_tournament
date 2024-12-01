use std::time::Duration;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_mod_sprite3d::{SizedMaterial, Sprite3d };


#[derive(Bundle, Debug)]
pub struct AnimationBundle<M: SizedMaterial = StandardMaterial> {
    pub animation_set: Handle<AnimationSet>,
    pub animation_state: AnimationState,
    pub sprite3d: Sprite3d,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub material: Handle<M>,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

impl<M: SizedMaterial> Default for AnimationBundle<M> {
    fn default() -> Self {
         Self {
            animation_set: default(),
            animation_state: default(),
            sprite3d: Sprite3d {
                custom_size: Some(Vec2::ZERO),
                ..default()
            },
            transform: default(),
            global_transform: default(),
            material: default(),
            visibility: default(),
            inherited_visibility: default(),
            view_visibility: default(),
        }
    }
}

/// A set of [`Animation`]s.
#[derive(Asset, TypePath, Clone, Default, Debug)]
pub struct AnimationSet {
    pub animations: Vec<Animation>,
}

impl AnimationSet {
    pub const EMPTY: Self = AnimationSet {
        animations: vec![],
    };

    #[allow(unused)]
    pub fn with_animation(mut self, animation: Animation) -> Self {
        self.animations.push(animation);
        self
    }

    pub fn with_animations(mut self, animations: impl IntoIterator<Item = Animation>) -> Self {
        for anim in animations {
            self = self.with_animation(anim);
        }
        self
    }
}

/// A single animation in an [`AnimationSet`].
#[derive(Clone, Default, Debug)]
pub struct Animation {
    pub frames: Vec<Frame>,
}

impl Animation {

    pub const EMPTY: Animation = Animation { frames: vec![] };

    /// Produces a new [`Animation`] with the frames.
    pub fn with_frames(
        mut self,
        count: u32,         // Number of frames to add
        start: Vec2,        // Top-left of first frame
        size: Vec2,         // Size of each frame
        stride: Vec2,       // Distance between each frame as a vector
        duration: Duration, // Duration of each frame
        anchor: Anchor,     // Anchor of the sprite in each frame
    ) -> Self {
        let mut pos = start;
        for _ in 0..count {
            self.frames.push(Frame {
                sprite: Sprite3d {
                    rect: Some(Rect { min: pos, max: pos + size }),
                    anchor,
                    ..default()
                },
                duration,
            });
            pos += stride;
        }
        self
    }
}

/// Causes entity's animation set to sync with another's.
/// Useful for equipment systems.
#[derive(Component, Copy, Clone, Eq, PartialEq, Debug)]
pub struct AnimationSync(pub Entity);

#[derive(Clone, Default, Debug)]
pub struct Frame<M: SizedMaterial> {
    pub sprite: Sprite3d<M>,
    pub duration: Duration,
}

#[derive(Component, Clone, Debug)]
pub struct AnimationState {
    pub animation_idx: usize,
    pub frame_idx: usize,
    pub mode: AnimationMode,
    pub frame_elapsed: Duration,
    pub stopped: bool,
}

impl Default for AnimationState {
    fn default() -> Self {
        Self {
            animation_idx: 0,
            frame_idx: 0,
            mode: AnimationMode::default(),
            frame_elapsed: Duration::from_millis(200),  // 12 FPS
            stopped: false,
        }
    }
}

/// Current behavior of an [`AnimationState`].
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub enum AnimationMode {
    #[allow(unused)]
    Play,
    #[default]
    Loop,
}

/// Updates sprite entities that have an animation that has changed recently.
pub fn update_animations<M: SizedMaterial>(
//    mut animation_q: Query<(&mut Sprite3d<M>, &mut AnimationState, &Handle<AnimationSet>)>,
    animations: Res<Assets<AnimationSet>>,
    time: Res<Time>,
) {
    // For all animations...
    for (mut sprite, mut anim_state, anim_set) in &mut animation_q {
        let anim_state_changed = anim_state.is_added() || anim_state.is_changed();
        if anim_state.stopped && !anim_state_changed { continue };

        // Consumes time and gets animation set
        let anim_set = animations.get(anim_set).unwrap();
        anim_state.frame_elapsed += time.delta();

        // Gets animation by index
        let Some(anim) = anim_set.animations.get(anim_state.animation_idx) else {
            bevy::log::warn!("Animation index out of bounds");
            anim_state.frame_elapsed = Duration::ZERO;
            continue
        };
        loop {

            // Gets frame by index
            let Some(frame) = anim.frames.get(anim_state.frame_idx) else {
                bevy::log::warn!("Frame index out of bounds");
                anim_state.frame_elapsed = Duration::ZERO;
                break;
            };

            // Updates frame index if the frame completed
            if anim_state.frame_elapsed > frame.duration {
                anim_state.frame_idx = match anim_state.mode {
                    AnimationMode::Play => (anim_state.frame_idx + 1).max(anim.frames.len() - 1),
                    AnimationMode::Loop => (anim_state.frame_idx + 1) % anim.frames.len(),
                };
                anim_state.frame_elapsed -= frame.duration;
                sync_sprite(&mut sprite, &frame.sprite);
            }

            // If not, animation is complete. Move on to the next one.
            else {
                sync_sprite(&mut sprite, &frame.sprite);
                break;
            }
        }
    }
}

/// Synchronizes sprites
pub fn sync_animations<M: SizedMaterial>(
    mut sync_q: Query<(&mut Sprite3d<M>, &AnimationSync)>,
    animation_q: Query<&Sprite3d<M>, Without<AnimationSync>>,
) {
    for (mut dest_sprite, AnimationSync(anim_id)) in &mut sync_q {
        let Ok(anim_sprite) = animation_q.get(*anim_id) else {
            bevy::log::warn!("Failed to look up sprite to sync with");
            continue;
        };
        sync_sprite(&mut *dest_sprite, &anim_sprite);
    }
}


fn sync_sprite(dest: &mut Sprite3d, src: &Sprite3d) {
    dest.flip_x = src.flip_x;
    dest.flip_y = src.flip_y;
    dest.custom_size = src.custom_size;
    dest.rect = src.rect;
    dest.anchor = src.anchor;
}
