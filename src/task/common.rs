use std::time::Duration;
use bevy::prelude::*;
use super::{Task, TaskCtx, TaskStatus};

/// Performs an arbitrary task during start().
pub struct Do<F>(Option<F>);
impl<F> Do<F>
where
    F: FnOnce(&mut World) + Send + Sync + 'static
{
    pub fn new(callback: F) -> Self {
        Self(Some(callback))
    }
}

impl<F> Task for Do<F>
where
    F: FnOnce(&mut World) + Send + Sync + 'static,
{
    fn start(&mut self, world: &mut World, _ctx: TaskCtx) {
        let callback = self.0.take().unwrap();
        callback(world);
    }
}


/// Clears all tasks in the [`TaskRunner`].
/// No more tasks will run.
/// Similar to [`DespawnHost`], except that the host entity does not despawn.
pub struct Quit;
impl Task for Quit {
    fn start(&mut self, _world: &mut World, mut ctx: TaskCtx) {
        ctx.clear();
    }
}

/// Despawns the entity that contains the [TaskRunner].
/// No more tasks will run.
/// Similar to [`Quit`], except that the host entity also despawns.
pub struct DespawnHost;
impl Task for DespawnHost {
    fn start(&mut self, world: &mut World, mut ctx: TaskCtx) {
        ctx.clear();
        world.despawn(ctx.host);
    }
}

pub struct Wait {
    duration: Duration,
    elapsed: Duration,
}

impl From<Duration> for Wait {
    fn from(duration: Duration) -> Self {
        Self {
            duration,
            elapsed: Duration::ZERO,
        }
    }
}

impl Wait {
    pub fn secs(secs: f32) -> Self {
        Self::from(Duration::from_secs_f32(secs))
    }
    pub fn millis(millis: u64) -> Self {
        Self::from(Duration::from_millis(millis))
    }
}

impl Task for Wait {
    fn run(&mut self, _world: &mut World, delta: Duration) -> TaskStatus {
        self.elapsed += delta;
        if self.elapsed < self.duration {
            TaskStatus::NotFinished
        }
        else {
            TaskStatus::FinishedRemaining(self.elapsed - self.duration)
        }
    }
}

/// Task that sets a state.
pub struct SetState<S> {
    state: Option<S>
}

impl<S> SetState<S> {
    pub fn new(state: S) -> Self {
        Self { state: Some(state) }
    }
}

impl<S: States> Task for SetState<S> {
    fn start(&mut self, world: &mut World, _ctx: TaskCtx) {
        let state = self.state.take().unwrap();
        let mut next_state = world.resource_mut::<NextState<S>>();
        next_state.set(state);
    }
}