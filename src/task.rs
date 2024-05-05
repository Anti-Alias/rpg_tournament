use std::collections::VecDeque;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;
use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::*;

/**
 * Allows for a set arbitrary tasks to be run one after another.
 */
pub struct TaskPlugin<S> {
    schedule: S,
}

impl <S: ScheduleLabel> TaskPlugin<S> {
    pub fn new(schedule: S) -> Self {
        Self { schedule }
    }
}


impl<S: ScheduleLabel + Clone> Plugin for TaskPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(self.schedule.clone(), run_task_runners::<S>);
    }
}

fn run_task_runners<S: ScheduleLabel>(world: &mut World, runners: &mut QueryState<(Entity, &mut TaskRunner<S>)>) {
    let delta = world.resource_mut::<Time>().delta();
    let runners: Vec<_> = runners
        .iter_mut(world)
        .map(|(host, runner)| (host, runner.inner.clone()))
        .collect();
    for (host, runner_inner) in runners {
        let mut runner_inner = runner_inner.lock().unwrap();
        if !runner_inner.started {
            runner_inner.start(world, host);
            runner_inner.started = true;
        }
        let finished = runner_inner.run(world, host, delta);
        if finished {
            let Some(mut host_ref) = world.get_entity_mut(host) else { continue };  // Command may despawn the host.
            host_ref.remove::<TaskRunner<S>>();
        }
    }
}


/// Component that runs a sequence of tasks one after another.
#[derive(Component)]
pub struct TaskRunner<S: ScheduleLabel = Update> {
    inner: Arc<Mutex<TaskRunnerInner>>,
    phantom: PhantomData<S>,
}
impl<S: ScheduleLabel> TaskRunner<S> {

    pub fn new(task: impl Task) -> Self {
        let mut tasks: VecDeque<Box<dyn Task>> = VecDeque::new();
        tasks.push_front(Box::new(task));
        Self {
            inner: Arc::new(Mutex::new(TaskRunnerInner {
                started: false,
                tasks,
            })),
            phantom: PhantomData,
        }
    }
}

impl TaskRunner<Update> {
    pub fn update(task: impl Task) -> Self {
        Self::new(task)
    }
}

impl TaskRunner<FixedUpdate> {
    pub fn fixed_update(task: impl Task) -> Self {
        Self::new(task)
    }
}

impl TaskRunner<PreUpdate> {
    pub fn pre_update(task: impl Task) -> Self {
        Self::new(task)
    }
}

impl TaskRunner<PostUpdate> {
    pub fn post_update(task: impl Task) -> Self {
        Self::new(task)
    }
}

struct TaskRunnerInner {
    started: bool,
    tasks: VecDeque<Box<dyn Task>>,
}

impl TaskRunnerInner {

    fn start(&mut self, world: &mut World, host: Entity,) {
        let mut task = self.tasks.pop_front().unwrap();
        let ctx = TaskCtx { host, insert_index: 0, tasks: &mut self.tasks };
        task.start(world, ctx);
        self.tasks.push_front(task);
    }

    fn run(&mut self, world: &mut World, host: Entity, mut delta: Duration) -> bool {
        loop {
            let mut task = self.tasks.pop_front().unwrap();
            let status = task.run(world, delta);
            match status {
                TaskStatus::NotFinished => {
                    self.tasks.push_front(task);
                    return false;
                },
                TaskStatus::Finished => {
                    task.end(world);
                    let Some(mut next_task) = self.tasks.pop_front() else { return true };
                    let ctx = TaskCtx { host, insert_index: 0, tasks: &mut self.tasks };
                    next_task.start(world, ctx);
                    self.tasks.push_front(next_task);
                },
                TaskStatus::FinishedRemaining(delta_remaining) => {
                    delta = delta_remaining;
                    task.end(world);
                    let Some(mut next_task) = self.tasks.pop_front() else { return true };
                    let ctx = TaskCtx { host, insert_index: 0, tasks: &mut self.tasks };
                    next_task.start(world, ctx);
                    self.tasks.push_front(next_task);
                }
            }
        };
    }
}

pub struct TaskCtx<'a> {
    host: Entity,
    insert_index: usize,
    tasks: &'a mut VecDeque<Box<dyn Task>>,
}

impl<'a> TaskCtx<'a> {

    /// Adds a task to the queue immediately after the current task.
    /// Subsequent invocations will be placed after the last task added.
    /// Useful for creating "aggregate tasks".
    pub fn push(&mut self, task: impl Task) {
        self.tasks.insert(self.insert_index, Box::new(task));
        self.insert_index += 1;
    }

    /// Helper method that pushes a [Do] task.
    pub fn then<F>(&mut self, callback: F)
    where
        F: FnOnce(&mut World) + Send + Sync + 'static
    {
        self.push(Do::new(callback));
    }

    /// Helper method that pushes a [Wait] task.
    pub fn wait(&mut self, secs: f32) {
        self.push(Wait::secs(secs));
    }

    /// Helper method that pushes a [Quit] or [DespawnHost] task.
    pub fn quit(&mut self, despawn_host: bool) {
        if despawn_host {
            self.push(DespawnHost);
        }
        else {
            self.push(Quit);
        }
    }

    /// Entity that contains the [TaskRunner].
    pub fn host(&self) -> Entity { self.host }

    /// Clears all tasks in the task queue.
    pub fn clear(&mut self) {
        self.tasks.clear();
        self.insert_index = 0;
    }
}

pub struct RunCtx {
    delta_ratio: f32
}

impl RunCtx {
    pub fn delta(&self, world: &mut World) -> Duration {
        world
            .resource::<Time>()
            .delta()
            .mul_f32(self.delta_ratio)
    }
}


/// An single task that executes some action.
/// A task may run for a single tick / frame, or multiple ticks / frames.
/// It may even run during the same tick as another task in the same [TaskRunner].
pub trait Task: Send + Sync + 'static {
    /// Sets up the task. Invoked a single time right before 1 or more invocations of run().
    #[allow(unused)]
    fn start(&mut self, world: &mut World, ctx: TaskCtx) {}
    /// Invoked at least once after start().
    #[allow(unused)]
    fn run(&mut self, world: &mut World, delta: Duration) -> TaskStatus { TaskStatus::Finished }
    /// Tears down the task. Invoked immediately after run() returns true.
    #[allow(unused)]
    fn end(&self, world: &mut World) {}
}


#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub enum TaskStatus {
    #[default]
    NotFinished,
    Finished,
    FinishedRemaining(Duration),
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

/// Shareable mutable state between multiple tasks.
#[derive(Clone, Default, Debug)]
pub struct Shared<T>(Arc<Mutex<T>>);
impl<T> Shared<T> {

    pub fn new(value: T) -> Self {
        Self(Arc::new(Mutex::new(value)))
    }

    /// Gets a mutable reference to the value of the state.
    pub fn lock(&self) -> MutexGuard<'_, T> {
        self.0.lock().unwrap()
    }

    /// Sets the value of the state.
    pub fn set(&self, value: T) {
        let mut current_value = self.0.lock().unwrap();
        *current_value = value;
    }
}

impl<T: Clone> Shared<T> {
    /// Gets a cloned copy of the state.
    pub fn get(&self) -> T {
        self.0.lock().unwrap().clone()
    }
}