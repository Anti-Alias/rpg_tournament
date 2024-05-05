use std::time::Duration;
use bevy::prelude::*;
use crate::task::{Shared, TaskCtx, Task, TaskStatus};

#[derive(States, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ScreenState {
    Title,
    Map { map_file: String, }
}

#[derive(Component, Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Local;

#[derive(Component, Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Carryover;


pub struct FadeIn {
    color: Color,
    duration: Duration,
    elapsed: Duration,
    state: Shared<FadeState>,
}

impl FadeIn {
    pub fn new(color: Color, duration_secs: f32, state: Shared<FadeState>) -> Self {
        let duration = Duration::from_secs_f32(duration_secs);
        Self {
            color,
            duration,
            elapsed: Duration::ZERO,
            state
        }
    }
}


impl Task for FadeIn {

    fn start(&mut self, world: &mut World, _ctx: TaskCtx) {
        const SCREEN_SIZE: Vec2 = Vec2::new(5000.0, 5000.0);
        let mut bundle = SpriteBundle::default();
        bundle.sprite.color = self.color;
        bundle.sprite.custom_size = Some(Vec2::ONE);
        bundle.transform.scale = SCREEN_SIZE.extend(1000.0);
        let entity = world.spawn(bundle).id();
        self.state.set(FadeState { entity });
    }

    fn run(&mut self, world: &mut World, delta: Duration) -> TaskStatus {
        let percent_done: f32 = {
            self.elapsed += delta;
            self.elapsed.as_secs_f32() / self.duration.as_secs_f32()
        };
        let mut sprite = {
            let state = self.state.get();
            world.get_mut::<Sprite>(state.entity).unwrap()
        };
        sprite.color.set_a(percent_done.min(1.0));
        if self.elapsed < self.duration {
            TaskStatus::NotFinished
        }
        else {
            TaskStatus::FinishedRemaining(self.elapsed - self.duration)
        }
    }
}

pub struct FadeOut {
    duration: Duration,
    elapsed: Duration,
    state: Shared<FadeState>,
}

impl FadeOut {
    pub fn new(duration_secs: f32, state: Shared<FadeState>) -> Self {
        let duration = Duration::from_secs_f32(duration_secs);
        Self {
            duration,
            elapsed: Duration::ZERO,
            state
        }
    }
}

impl Task for FadeOut {
    fn run(&mut self, world: &mut World, delta: Duration) -> TaskStatus {
        let percent_done: f32 = {
            self.elapsed += delta;
            self.elapsed.as_secs_f32() / self.duration.as_secs_f32()
        };
        let mut sprite = {
            let state = self.state.get();
            world.get_mut::<Sprite>(state.entity).unwrap()
        };
        sprite.color.set_a(1.0 - percent_done.min(1.0));
        if self.elapsed < self.duration {
            TaskStatus::NotFinished
        }
        else {
            TaskStatus::FinishedRemaining(self.elapsed - self.duration)
        }
    }

    fn end(&self, world: &mut World) {
        let state = self.state.get();
        world.despawn(state.entity);
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct FadeState {
    entity: Entity,
}

impl FadeState {
    pub const fn new() -> Self {
        Self {
            entity: Entity::PLACEHOLDER,
        }
    }
}