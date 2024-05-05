use std::time::Duration;
use bevy::prelude::*;
use crate::task::{SetState, Shared, Task, TaskCtx, TaskStatus};
use super::{Despawnable, ScreenState};

const FADE_Z_INDEX: ZIndex = ZIndex::Global(1024);

/// Task that fades the screen to a certain color.
/// Spawns a fullscreen "fade entity".
/// Useful for screen transitions.
pub struct FadeIn {
    color: Color,
    duration: Duration,
    elapsed: Duration,
    data: Shared<FadeData>,
}

impl FadeIn {
    pub fn new(color: Color, duration_secs: f32, data: Shared<FadeData>) -> Self {
        let duration = Duration::from_secs_f32(duration_secs);
        Self {
            color,
            duration,
            elapsed: Duration::ZERO,
            data
        }
    }
}


impl Task for FadeIn {

    // Spawns fullscreen fade entity
    fn start(&mut self, world: &mut World, _ctx: TaskCtx) {
        let mut bundle = NodeBundle::default();
        bundle.background_color = self.color.into();
        bundle.style.width = Val::Percent(100.0);
        bundle.style.height = Val::Percent(100.0);
        bundle.transform.translation.z = -1000.0;
        bundle.z_index = FADE_Z_INDEX;
        let node = world.spawn(bundle).id();
        self.data.set(FadeData { node });
    }

    // Updates fade's alpha over time.
    fn run(&mut self, world: &mut World, delta: Duration) -> TaskStatus {
        let percent_done: f32 = {
            self.elapsed += delta;
            self.elapsed.as_secs_f32() / self.duration.as_secs_f32()
        };
        let mut node_color = {
            let data = self.data.get();
            world.get_mut::<BackgroundColor>(data.node).unwrap()
        };
        node_color.0.set_a(percent_done.min(1.0));
        if self.elapsed < self.duration {
            TaskStatus::NotFinished
        }
        else {
            TaskStatus::FinishedRemaining(self.elapsed - self.duration)
        }
    }
}

/// Task that fades the screen out.
/// Reverse of [`FadeIn`].
pub struct FadeOut {
    duration: Duration,
    elapsed: Duration,
    data: Shared<FadeData>,
}

impl FadeOut {
    pub fn new(duration_secs: f32, data: Shared<FadeData>) -> Self {
        let duration = Duration::from_secs_f32(duration_secs);
        Self {
            duration,
            elapsed: Duration::ZERO,
            data
        }
    }
}

impl Task for FadeOut {
    fn run(&mut self, world: &mut World, delta: Duration) -> TaskStatus {
        let percent_done: f32 = {
            self.elapsed += delta;
            self.elapsed.as_secs_f32() / self.duration.as_secs_f32()
        };
        let mut node_color = {
            let data = self.data.get();
            world.get_mut::<BackgroundColor>(data.node).unwrap()
        };
        node_color.0.set_a(1.0 - percent_done.min(1.0));
        if self.elapsed < self.duration {
            TaskStatus::NotFinished
        }
        else {
            TaskStatus::FinishedRemaining(self.elapsed - self.duration)
        }
    }

    fn end(&self, world: &mut World) {
        let data = self.data.get();
        world.despawn(data.node);
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct FadeData {
    node: Entity,
}

impl FadeData {
    pub const fn new() -> Self {
        Self {
            node: Entity::PLACEHOLDER,
        }
    }
}

pub struct FadeToScreen {
    screen_state: Option<ScreenState>,
    fade_in_secs: f32,
    fade_out_secs: f32,
}

impl FadeToScreen {
    pub fn new(screen: ScreenState, fade_in_secs: f32, fade_out_secs: f32) -> Self {
        Self {
            screen_state: Some(screen),
            fade_in_secs,
            fade_out_secs,
        }
    }
}

impl Task for FadeToScreen {
    fn start(&mut self, _world: &mut World, mut ctx: TaskCtx) {
        let state = Shared::new(FadeData::new());
        let screen_state = self.screen_state.take().unwrap();
        ctx.push(FadeIn::new(Color::BLACK, self.fade_in_secs, state.clone()));
        ctx.push(DespawnAll);
        ctx.push(SetState::new(screen_state));
        ctx.push(FadeOut::new(self.fade_out_secs, state.clone()));
    }
}

/// Despawns all entities marked with a [`Despawnable`] component.
/// Useful for clearing the screen during transitions.
pub struct DespawnAll;
impl Task for DespawnAll {
    fn start(&mut self, world: &mut World, _ctx: TaskCtx) {
        let mut query = world.query_filtered::<Entity, With<Despawnable>>();
        let entities: Vec<Entity> = query.iter(&world).collect();
        for entity in entities {
            despawn_with_children_recursive(world, entity);
        }
    }
}