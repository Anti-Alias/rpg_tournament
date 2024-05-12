use std::collections::VecDeque;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;
use bevy::ecs::system::CommandQueue;
use bevy::prelude::*;
mod common;
pub mod ext;
use bevy::transform::TransformSystem;
pub use common::*;

/**
 * Allows for a set arbitrary tasks to be run one after another.
 */
pub struct TaskPlugin;


impl Plugin for TaskPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, run_task_runners.before(TransformSystem::TransformPropagate));
    }
}

fn run_task_runners(
    world: &mut World,
    runners: &mut QueryState<(Entity, &mut TaskRunner)>,
    mut command_queue: Local<CommandQueue>,
) {
    let delta = world.resource_mut::<Time>().delta();
    let runners: Vec<_> = runners
        .iter_mut(world)
        .map(|(host, runner)| (host, runner.inner.clone()))
        .collect();
    for (host, runner_inner) in runners {
        let mut runner_inner = runner_inner.lock().unwrap();
        if !runner_inner.started {
            runner_inner.start(world, &mut command_queue, host);
            runner_inner.started = true;
        }
        let finished = runner_inner.run(world, &mut command_queue, host, delta);
        if finished {
            let Some(mut host_ref) = world.get_entity_mut(host) else { continue };  // Command may despawn the host.
            host_ref.remove::<TaskRunner>();
        }
    }
}


/// Component that runs a sequence of tasks one after another.
#[derive(Component)]
pub struct TaskRunner {
    inner: Arc<Mutex<TaskRunnerInner>>,
}

impl TaskRunner {

    /// Creates runner with exactly one task.
    pub fn new(task: impl Task) -> Self {
        Self::from(task)
    }

    /// Pushes one more task and returns self.
    pub fn push(self, task: impl Task) -> Self {
        let task = Box::new(task);
        self.inner.lock().unwrap().tasks.push_back(task);
        self
    }
}

impl<T: Task> From<T> for TaskRunner {
    fn from(task: T) -> Self {
        let mut tasks: VecDeque<Box<dyn Task>> = VecDeque::new();
        tasks.push_front(Box::new(task));
        Self {
            inner: Arc::new(Mutex::new(TaskRunnerInner {
                started: false,
                tasks,
            })),
        }
    }
}

impl From<Box<dyn Task>> for TaskRunner {
    fn from(task: Box<dyn Task>) -> Self {
        let mut tasks: VecDeque<Box<dyn Task>> = VecDeque::new();
        tasks.push_front(task);
        Self {
            inner: Arc::new(Mutex::new(TaskRunnerInner {
                started: false,
                tasks,
            })),
        }
    }
}

struct TaskRunnerInner {
    started: bool,
    tasks: VecDeque<Box<dyn Task>>,
}

impl TaskRunnerInner {

    fn start(&mut self, world: &mut World, command_queue: &mut CommandQueue, host: Entity,) {
        let mut task = self.tasks.pop_front().unwrap();
        let tq = &mut TaskQueue { host, insert_index: 0, tasks: &mut self.tasks, command_queue };
        task.start(world, tq);
        command_queue.apply(world);
        self.tasks.push_front(task);
    }

    fn run(&mut self, world: &mut World, command_queue: &mut CommandQueue, host: Entity, mut delta: Duration) -> bool {
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
                    let tq = &mut TaskQueue { host, insert_index: 0, tasks: &mut self.tasks, command_queue };
                    next_task.start(world, tq);
                    command_queue.apply(world);
                    self.tasks.push_front(next_task);
                },
                TaskStatus::FinishedRemaining(delta_remaining) => {
                    delta = delta_remaining;
                    task.end(world);
                    let Some(mut next_task) = self.tasks.pop_front() else { return true };
                    let tq = &mut TaskQueue { host, insert_index: 0, tasks: &mut self.tasks, command_queue };
                    next_task.start(world, tq);
                    command_queue.apply(world);
                    self.tasks.push_front(next_task);
                }
            }
        };
    }
}

/// Queue of tasks that will be run in order of submission.
pub struct TaskQueue<'a> {
    host: Entity,
    insert_index: usize,
    tasks: &'a mut VecDeque<Box<dyn Task>>,
    pub command_queue: &'a mut CommandQueue,
}

impl<'a> TaskQueue<'a> {

    /// Adds a task to the queue immediately after the current task.
    /// Subsequent invocations will be placed after the last task added.
    /// Useful for creating "aggregate tasks".
    pub fn push(&mut self, task: impl Task) {
        self.tasks.insert(self.insert_index, Box::new(task));
        self.insert_index += 1;
    }

    /// Helper method that pushes a [Start] task.
    pub fn start<F, R>(&mut self, callback: F)
    where
        F: FnOnce(&mut World, &mut TaskQueue) -> R + Send + Sync + 'static,
        R: Send + Sync + 'static,
    {
        self.push(Start::new(callback));
    }

    /// Helper method that pushes a [Run] task.
    pub fn run<F>(&mut self, callback: F)
    where
        F: FnMut(&mut World, Duration) -> TaskStatus + Send + Sync + 'static,
    {
        self.push(Run::new(callback));
    }

    /// Pushes a task that waits.
    pub fn wait(&mut self, duration: Duration) {
        let mut elapsed = Duration::ZERO;
        self.run(move |_, delta| {
            elapsed += delta;
            if elapsed > duration {
                TaskStatus::FinishedRemaining(elapsed - duration)
            }
            else {
                TaskStatus::NotFinished
            }
        });
    }

    /// Pushes a task that waits.
    pub fn wait_secs(&mut self, secs: f32) {
        self.wait(Duration::from_secs_f32(secs))
    }

    /// Pushes a task that waits.
    pub fn wait_ms(&mut self, millis: u64) {
        self.wait(Duration::from_millis(millis))
    }

    /// Pushes a task that sets a state.
    pub fn set_state<S: States>(&mut self, state: S) {
        self.start(|world, _| {
            let mut next_state = world.resource_mut::<NextState<S>>();
            next_state.set(state);
        });
    }

    /// Pushes a task that fires an event.
    pub fn fire(&mut self, event: impl Event) {
        self.start(move |world, _| {
            world.send_event(event);
        });
    }

    /// Pushes a task that despawns an entity.
    pub fn despawn(&mut self, entity: Entity, recursive: bool) {
        self.start(move |world, _| {
            let e = world.entity_mut(entity);
            if recursive {
                e.despawn_recursive();
            }
            else {
                e.despawn();
            }
        });
    }

    /// Pushes a task that clears the task queue.
    pub fn quit(&mut self, despawn_host: bool) {
        self.push(Quit { despawn_host } )
    }

    /// Pushes a task that waits exactly one frame / tick.
    /// Useful if the output of a previous task is delayed by a frame / tick.
    pub fn skip(&mut self) {
        let mut skipped = false;
        self.run(move |_, _| {
            if !skipped {
                skipped = true;
                TaskStatus::NotFinished
            }
            else {
                TaskStatus::Finished
            }
        });
    }

    /// Entity that contains the [TaskRunner].
    pub fn host(&self) -> Entity { self.host }

    /// Clears all tasks in the task queue.
    /// Does not push a task.
    pub fn clear(&mut self) {
        self.tasks.clear();
        self.insert_index = 0;
    }
}

/// An single task that executes some action.
/// A task may run for a single tick / frame, or multiple ticks / frames.
/// It may even run during the same tick as another task in the same [TaskRunner].
pub trait Task: Send + Sync + 'static {
    /// Sets up the task. Invoked a single time right before 1 or more invocations of run().
    /// Typically, users implement this if either:
    /// 1) Their task needs setup logic.
    /// 2) Their task is noting more than an "aggretate task" which pushes more tasks into the queue.
    #[allow(unused)]
    fn start(&mut self, world: &mut World, tq: &mut TaskQueue) {}
    /// Invoked immediately after start.
    /// Invoked as long as it returns [TaskStatus::NotFinished].
    /// Returns [TaskStatus::Finished] by default.
    #[allow(unused)]
    fn run(&mut self, world: &mut World, delta: Duration) -> TaskStatus { TaskStatus::Finished }
    /// Tears down the task. Invoked immediately after run() finishes.
    #[allow(unused)]
    fn end(&self, world: &mut World) {}
}


#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub enum TaskStatus {
    /// Task is not finished. run() will be invoked again next frame.
    #[default]
    NotFinished,
    /// Task is finished. Next task, if any, will start(), then run(). Delta assumed to be [Duration::ZERO].
    Finished,
    /// Task is finished. Next task, if any, will start(), then run(), using the delta returned.
    FinishedRemaining(Duration),
}

impl From<bool> for TaskStatus {
    fn from(value: bool) -> Self {
        match value {
            true => Self::Finished,
            false => Self::NotFinished,
        }
    }
}

/// Shareable mutable state across tasks.
/// Useful if the output of one task needs to be used in a later one.
/// For instance, a dialog tree.
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

/// Used to block tasks from running.
#[derive(Clone, Default, Debug)]
pub struct TaskLock(Shared<bool>);
impl TaskLock {

    pub fn new() -> Self {
        Self(Shared::new(false))
    }

    pub fn is_locked(&self) -> bool {
        self.0.get()
    }

    /// Acquires lock.
    /// Returns true if successful.
    pub fn lock(&self) -> bool {
        let is_locked: &mut bool = &mut self.0.lock();
        if *is_locked { return false };
        *is_locked = true;
        true
    }

    /// Releases lock.
    /// Returns true if successful.
    pub fn unlock(&self) -> bool {
        let is_locked: &mut bool = &mut self.0.lock();
        if !*is_locked { return false };
        *is_locked = false;
        true
    }
}