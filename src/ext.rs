use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use crate::task::*;
use crate::ui::*;

/// Allows a [`Commands`] to more easily spawn a task entity.
pub trait CommandsExt {
    fn spawn_task(&mut self, task: impl Task) -> EntityCommands<'_>;
}

pub trait EntityCommandsExt {
    fn on_press<F, R>(&mut self, callback: F) -> &mut Self
    where
        F: Fn(&mut World) -> R + Send + Sync + 'static;
}

impl<'a> EntityCommandsExt for EntityCommands<'a> {
    fn on_press<F, R>(&mut self, callback: F) -> &mut Self
    where
        F: Fn(&mut World) -> R + Send + Sync + 'static
    {
        self.insert(OnPress::new(callback))
    }
}

impl<'w, 's> CommandsExt for Commands<'w, 's> {
    fn spawn_task(&mut self, task: impl Task) -> EntityCommands<'_> {
        let mut runner = TaskRunner::new(task);
        runner.push(Quit { despawn_host: true });
        self.spawn(runner)
    }
}

/// Allows a [`World`] to more easily spawn a task entity.
pub trait WorldExt {
    fn spawn_task(&mut self, task: impl Task) -> Entity;
}

impl WorldExt for World {
    fn spawn_task(&mut self, task: impl Task) -> Entity {
        let mut runner = TaskRunner::new(task);
        runner.push(Quit { despawn_host: true });
        self.spawn(runner).id()
    }
}