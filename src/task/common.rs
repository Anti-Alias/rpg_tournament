use std::marker::PhantomData;
use std::time::Duration;
use bevy::prelude::*;
use super::{Task, TaskLock, TaskQueue, TaskStatus};

/// Performs an arbitrary task once.
/// Useful for creating inline tasks that execute once then immediately finish.
/// Also useful for creating simple "aggregate" tasks.
pub struct Start<F, R> {
    callback: Option<F>,
    phantom: PhantomData<R>,
}

impl<F, R> Start<F, R>
where
    F: FnOnce(&mut World, &mut TaskQueue) -> R + Send + Sync + 'static,
    R: Send + Sync + 'static,
{
    pub fn new(callback: F) -> Self {
        Self {
            callback: Some(callback),
            phantom: PhantomData,
        }
    }
}

impl<F, R> Task for Start<F, R>
where
    F: FnOnce(&mut World, &mut TaskQueue) -> R + Send + Sync + 'static,
    R: Send + Sync + 'static,
{
    fn start(&mut self, world: &mut World, tq: &mut TaskQueue) {
        let callback = self.callback.take().unwrap();
        callback(world, tq);
    }
}

/// Performs an arbitrary task at least one time.
/// Useful for creating inline tasks.
pub struct Run<F>(F);
impl<F> Run<F>
where
    F: FnMut(&mut World, Duration) -> TaskStatus + Send + Sync + 'static
{
    pub fn new(callback: F) -> Self {
        Self(callback)
    }
}

impl<F> Task for Run<F>
where
    F: FnMut(&mut World, Duration) -> TaskStatus + Send + Sync + 'static
{
    fn run(&mut self, world: &mut World, delta: Duration) -> TaskStatus {
        self.0(world, delta)
    }
}


/// Clears all tasks in the [`TaskQueue`].
/// Acts as an "early return".
/// [`TaskRunner`](crate::task::TaskRunner) will be removed from host.
/// Despawns host if configured.
pub struct Quit { pub despawn_host: bool }
impl Task for Quit {
    fn start(&mut self, world: &mut World, tq: &mut TaskQueue) {
        tq.clear();
        if self.despawn_host {
            world.despawn(tq.host());
        }
    }
}

/// Task that runs an inner task only if not locked.
pub struct Guard<T> {
    task: T,
    lock: TaskLock,
    failed: bool,
}

impl<T> Guard<T> {
    pub fn new(task: T, lock: TaskLock) -> Self {
        Self {
            task,
            lock,
            failed: false,
        }
    }
}

impl<T: Task> Task for Guard<T> {
    fn start(&mut self, world: &mut World, tq: &mut TaskQueue) {
        if self.lock.lock() {
            let lock_cb = self.lock.clone();
            self.task.start(world, tq);
            tq.start(move |_,_| lock_cb.unlock());
        }
        else {
            self.failed = true;
        }
    }
    fn run(&mut self, world: &mut World, delta: Duration) -> TaskStatus {
        if self.failed { return TaskStatus::Finished }
        self.task.run(world, delta)
    }
    fn end(&self, world: &mut World) {
        if self.failed { return }
        self.task.end(world);
    }
}