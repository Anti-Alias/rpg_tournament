use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use crate::task::{Task, TaskRunner};
use crate::ui::OnPress;

/// Allows a [`Commands`] to more easily spawn a task entity.
pub trait CommandsExt {
    fn spawn_task(&mut self, task: impl Task) -> EntityCommands<'_>;
}

pub trait EntityCommandsExt {
    fn on_press<C, R>(&mut self, callback: C) -> &mut Self
    where
        C: Fn(&mut World) -> R + Send + Sync + 'static;
}

impl<'a> EntityCommandsExt for EntityCommands<'a> {
    fn on_press<C, R>(&mut self, callback: C) -> &mut Self
    where
        C: Fn(&mut World) -> R + Send + Sync + 'static
    {
        self.insert(OnPress::call(callback))
    }
}

impl<'w, 's> CommandsExt for Commands<'w, 's> {
    fn spawn_task(&mut self, task: impl Task) -> EntityCommands<'_> {
        self.spawn(TaskRunner::new(task))
    }
}

/// Allows a [`World`] to more easily spawn a task entity.
pub trait WorldExt {
    fn spawn_task(&mut self, task: impl Task) -> Entity;
}

impl WorldExt for World {
    fn spawn_task(&mut self, task: impl Task) -> Entity {
        self.spawn(TaskRunner::new(task)).id()
    }
}