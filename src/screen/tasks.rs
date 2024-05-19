use std::time::Duration;
use bevy::prelude::*;
use crate::task::{Task, TaskQueue, TaskStatus};
use crate::GameState;

use super::{Keep, ScreenEvent, ScreenState};

const FADE_Z_INDEX: ZIndex = ZIndex::Global(1024);

/// Task that fades the screen to a certain color.
/// Spawns a fullscreen "fade entity".
/// Useful for screen transitions.
pub struct FadeIn {
    color: Color,
    duration: Duration,
    elapsed: Duration,
    fade_id: Entity,
}

impl FadeIn {
    pub fn new(fade_id: Entity, color: Color, duration_secs: f32) -> Self {
        let duration = Duration::from_secs_f32(duration_secs);
        Self {
            color,
            duration,
            elapsed: Duration::ZERO,
            fade_id,
        }
    }
}


impl Task for FadeIn {

    // Spawns fullscreen fade entity
    fn start(&mut self, world: &mut World, _tq: &mut TaskQueue) {
        let mut node = NodeBundle::default();
        node.background_color = self.color.into();
        node.style.width = Val::Percent(100.0);
        node.style.height = Val::Percent(100.0);
        node.transform.translation.z = -1000.0;
        node.z_index = FADE_Z_INDEX;
        world.entity_mut(self.fade_id).insert(node);
    }

    // Updates fade's alpha over time.
    fn run(&mut self, world: &mut World, delta: Duration) -> TaskStatus {
        let percent_done: f32 = {
            self.elapsed += delta;
            self.elapsed.as_secs_f32() / self.duration.as_secs_f32()
        };
        let percent_done = percent_done.min(1.0);
        let mut node_color = {
            world.get_mut::<BackgroundColor>(self.fade_id).unwrap()
        };
        node_color.0.set_a(percent_done * percent_done);
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
    fade_id: Entity,
    duration: Duration,
    elapsed: Duration,
}

impl FadeOut {
    pub fn new(fade_id: Entity, duration_secs: f32) -> Self {
        let duration = Duration::from_secs_f32(duration_secs);
        Self {
            duration,
            elapsed: Duration::ZERO,
            fade_id,
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
            world.get_mut::<BackgroundColor>(self.fade_id).unwrap()
        };
        node_color.0.set_a(1.0 - percent_done.min(1.0));
        if self.elapsed < self.duration {
            TaskStatus::NotFinished
        }
        else {
            TaskStatus::FinishedRemaining(self.elapsed - self.duration)
        }
    }
}


pub struct FadeToScreen(pub ScreenState);
impl Task for FadeToScreen {
    fn start(&mut self, _world: &mut World, tq: &mut TaskQueue) {
        let screen_state = self.0.clone();
        tq.quit_if_state(GameState::Transitioning, true);
        tq.set_state(GameState::Transitioning);
        tq.start(move |world, tq| {
            let fade_id = world.spawn(Keep).id();
            tq.insert_host(Keep);
            tq.fade_in(fade_id, Color::BLACK, 0.25);
            tq.set_state(screen_state);
            tq.wait_for_event(ScreenEvent::FinishedLoading);
            tq.fade_out(fade_id, 0.25);
            tq.despawn(fade_id, false, true);
            tq.set_state(GameState::Running);
            tq.quit(true);
        });
    }
}


/// Blankets the screen in a fullscreen fade object, then fades out when screen loading is done.
pub struct FadeInitial;
impl Task for FadeInitial {
    fn start(&mut self, _world: &mut World, tq: &mut TaskQueue) {
        tq.quit_if_state(GameState::Transitioning, true);
        tq.start(move |world, tq| {
            let fade_id = world.spawn_empty().id();
            tq.fade_in(fade_id, Color::BLACK, 0.0);
            tq.set_state(GameState::Transitioning);
            tq.wait_for_event(ScreenEvent::FinishedLoading);
            tq.wait_secs(0.25);
            tq.fade_out(fade_id, 0.25);
            tq.despawn(fade_id, true, true);
            tq.set_state(GameState::Running);
        });
    }
}