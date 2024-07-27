use std::collections::VecDeque;
use std::time::Duration;
use bevy::prelude::*;
use smallvec::SmallVec;
pub mod common;

/// A queue of actions that will be executed sequentially over the course of 1 or more frames/ticks.
/// Useful for stringing together arbitrary actions in a sequence.
/// Applications include cutscenes, lerping, screen transitions, waiting logic, animated dialog trees, etc.
#[derive(Component)]
pub enum ActionQueue {
    Occupied(InnerActionQueue),
    Empty,
}

impl Default for ActionQueue {
    fn default() -> Self {
        Self::Occupied(InnerActionQueue::default())
    }
}

impl<A: Action> From<A> for ActionQueue {
    fn from(value: A) -> Self {
        let mut actions: VecDeque<Box<dyn Action>> = VecDeque::new();
        actions.push_back(Box::new(value));
        Self::Occupied(InnerActionQueue {
            running_action: None,
            remaining_actions: actions,
            on_finish: OnFinish::default(),
            quit: false,
        })
    }
}

impl ActionQueue {

    #[inline(always)]
    pub fn push(&mut self, action: impl Action) {
        let queue = match self {
            ActionQueue::Occupied(queue) => queue,
            ActionQueue::Empty => panic!("Queue not occupied"),
        };
        queue.remaining_actions.push_back(Box::new(action));
    }

    /// Sets the operation to perform when all [`Action`]s have been consumed.
    pub fn with_on_finish(mut self, on_finish: OnFinish) -> Self {
        match &mut self {
            ActionQueue::Occupied(queue) => queue.on_finish = on_finish,
            ActionQueue::Empty => panic!("Queue not occupied"),
        }
        self
    }

    /// Schedules the queue to quit.
    pub fn quit(&mut self) {
        match self {
            ActionQueue::Occupied(queue) => queue.quit = true,
            ActionQueue::Empty => panic!("Queue not occupied"),
        }
    }

    fn inner(self) -> InnerActionQueue {
        match self {
            ActionQueue::Occupied(inner) => inner,
            ActionQueue::Empty => panic!("ActionQueue not occupied"),
        }
    }
}


#[derive(Default)]
pub struct InnerActionQueue {
    running_action: Option<Box<dyn Action>>,
    remaining_actions: VecDeque<Box<dyn Action>>,
    quit: bool,
    pub on_finish: OnFinish,
}

impl InnerActionQueue {

    fn is_finished(&self) -> bool {
        self.running_action.is_none() && self.remaining_actions.is_empty()
    }

    fn update(&mut self, world: &mut World, entity: Entity, delta: Duration) {
        let mut time_remaining = delta;
        loop {

            if self.quit {
                while let Some(mut action) = self.remaining_actions.pop_front() {
                    action.end(&mut EndEnv { world, entity });
                }
                self.quit = false;
                break;
            }

            // Gets next action to run.
            // Starts it if this is its first time.
            let mut action = match self.running_action.take() {
                Some(action) => action,
                None => {
                    let Some(mut action) = self.remaining_actions.pop_front() else { break };
                    action.start(&mut StartEnv {
                        world,
                        entity,
                        actions: &mut self.remaining_actions,
                        insert_idx: 0,
                    });
                    action
                },
            };
            // Runs action.
            let run_status = action.run(&mut RunEnv { world, entity, delta: time_remaining });
            match run_status {
                // Resumes action next tick
                RunStatus::Running => {
                    self.running_action = Some(action);
                    break;
                },
                // Ends action, and runs the next one if there is one, and there is time remaining.
                RunStatus::Finished { time_consumed } => {
                    let time_consumed = time_consumed.max(time_remaining);
                    time_remaining -= time_consumed;
                    action.end(&mut EndEnv { world, entity });
                    if time_remaining == Duration::ZERO { break }
                },
                // Ends action and every other remaining action.
                RunStatus::Quit => {
                    action.end(&mut EndEnv { world, entity });
                    self.quit = true;
                }
            }
        }
    }
}


/// A single task to run in an [`ActionQueue`].
/// Executes over the course of at least one frame/tick.
/// 
/// Lifecycle example:
/// Frame 1 (run not finished):
///     start(), run() -> Running
/// Frame 2 (run not finished):
///     run() -> Running
/// Frame 3 (run not finished):
///     run() -> Running
/// Frame 4 (run finished, next action in queue will run if there is one):
///     run() -> Finished { ... }
///     end()
pub trait Action: Send + Sync + 'static {
    /// Called on the first frame/tick.
    /// Useful for initialization.
    #[allow(unused)]
    fn start(&mut self, env: &mut StartEnv) {}
    /// Called continuously each frame/tick.
    /// Returns true if finished.
    #[allow(unused)]
    fn run(&mut self, env: &mut RunEnv) -> RunStatus {
        RunStatus::Finished { time_consumed: Duration::ZERO }
    }
    /// Called on same frame/tick that run() returns true.
    /// Useful for cleanup.
    #[allow(unused)]
    fn end(&mut self, env: &mut EndEnv) {}
}

impl<F, R> Action for F
where
    F: FnMut(&mut StartEnv) -> R + Send + Sync + 'static,
    R: Send + Sync + 'static,
{
    fn start(&mut self, env: &mut StartEnv) {
        self(env);
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum RunStatus {
    /// [`Action`] should run again next frame/tick.
    Running,
    /// [`Action`] finished this frame/tick. Next [`Action`] in queue should run.
    Finished { time_consumed: Duration },
    /// [`Action`] forces [`ActionQueue`] to abruptly quit. All remaining [`Action`]s will have their end() methods invoked.
    Quit,
}

pub struct StartEnv<'a> {
    pub world: &'a mut World,
    pub entity: Entity,
    actions: &'a mut VecDeque<Box<dyn Action>>,
    insert_idx: usize,
}

impl<'a> StartEnv<'a> {
    pub fn push(&mut self, action: impl Action) {
        self.actions.insert(self.insert_idx, Box::new(action));
        self.insert_idx += 1;
    }
}

pub struct EndEnv<'a> {
    pub world: &'a mut World,
    pub entity: Entity,
}

pub struct RunEnv<'a> {
    pub world: &'a mut World,
    pub entity: Entity,
    pub delta: Duration,
}

/// Task to perform on the [`Entity`] of an [`ActionQueue`] after all [`Action`]s are exhausted.
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub enum OnFinish {
    Nothing,
    RemoveQueue,
    Despawn,
    #[default]
    DespawnRecursive,
}

#[derive(Event, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct RunAction(pub ActionKind);

#[derive(Event, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct QuitAction(pub ActionKind);

#[derive(Component, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum ActionKind {
    PrintDisclaimer,
    Cutscene,
}

impl ActionKind {
    pub fn to_action_queue(self) -> ActionQueue {
        match self {
            ActionKind::PrintDisclaimer => ActionQueue::from(common::Print("This is a disclaimer")),
            ActionKind::Cutscene        => ActionQueue::from(common::cutscene),
        }
    }
}

pub fn run_action(
    trigger: Trigger<RunAction>,
    mut commands: Commands,
) {
    let action_kind = trigger.event().0;
    let action_queue = action_kind.to_action_queue().with_on_finish(OnFinish::Despawn);
    commands.spawn((action_queue, action_kind));
}

pub fn quit_action(
    trigger: Trigger<QuitAction>,
    mut commands: Commands,
    mut action_queues: Query<(Entity, &mut ActionQueue, &ActionKind)>,
) {
    let action_kind = trigger.event().0;
    for (entity, mut queue, kind) in &mut action_queues {
        if action_kind == *kind {
            queue.quit();
            commands.entity(entity).despawn_recursive();
        }
    }
}

/// Runs all action queues for the current frame/tick.
pub fn run_action_queues(
    world: &mut World,
    action_queues: &mut QueryState<(Entity, &mut ActionQueue)>,
) {
    // Takes out all action queues from their entities.
    // This is done because action queues need exclusive mutable world access when executing.
    let mut inner_action_queues = SmallVec::<[_; 8]>::new();
    for (action_queue_entity, mut action_queue) in action_queues.iter_mut(world) {
        let action_queue = &mut *action_queue;
        let action_queue = std::mem::replace(action_queue, ActionQueue::Empty);
        inner_action_queues.push((action_queue_entity, action_queue.inner()));
    }

    // Runs all action queues for this frame/tick.
    let delta = world.resource::<Time>().delta();
    for (entity, inner_action_queue) in &mut inner_action_queues {
        inner_action_queue.update(world, *entity, delta);
    }

    // Decides where to put action queues after executing this frame/tick.
    for (action_queue_entity, action_queue) in inner_action_queues {
        let (is_finished, on_finish) = (action_queue.is_finished(), action_queue.on_finish);
        match (is_finished, on_finish) {
            (true, OnFinish::Nothing) | (false, _) => {
                let Some(mut action_queue_entity) = world.get_entity_mut(action_queue_entity) else { continue };
                action_queue_entity.insert(ActionQueue::Occupied(action_queue));
            },
            (true, OnFinish::RemoveQueue) => {
                let Some(mut action_queue_entity) = world.get_entity_mut(action_queue_entity) else { continue };
                action_queue_entity.remove::<ActionQueue>();
            },
            (true, OnFinish::Despawn) => {
                let Some(action_queue_entity) = world.get_entity_mut(action_queue_entity) else { continue };
                action_queue_entity.despawn();
            },
            (true, OnFinish::DespawnRecursive) => {
                let Some(action_queue_entity) = world.get_entity_mut(action_queue_entity) else { continue };
                action_queue_entity.despawn_recursive();
            },
        }
    }
}