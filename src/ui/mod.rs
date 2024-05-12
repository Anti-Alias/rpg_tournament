mod widgets;
mod dialog;

pub use widgets::*;
pub use dialog::*;

use crate::task::*;
use bevy::prelude::*;
use std::sync::Arc;

pub fn ui_plugin(app: &mut App) {
    app.add_systems(Update, (handle_ui_interactions, advance_text));
}

fn handle_ui_interactions(
    world: &mut World,
    interactions: &mut QueryState<
        (&Interaction, &OnPress),
        Changed<Interaction>
    >,
) {
    let mut callbacks = vec![];
    for (interaction, on_press) in interactions.iter(world) {
        if *interaction == Interaction::Pressed {
            callbacks.push(on_press.0.clone());
        }
    }
    for callback in callbacks {
        callback.invoke(world);
    }
}

/// A simple callback function responding to something that happened to an entity.
pub trait Callback: Send + Sync + 'static {
    fn invoke(&self, world: &mut World);
}

impl<F> Callback for F
where
    F: Fn(&mut World) + Send + Sync + 'static
{
    fn invoke(&self, world: &mut World) {
        self(world);
    }
}

/// Component that gets fires when an entity gets pressed.
#[derive(Component)]
pub struct OnPress(Arc<dyn Callback>);
impl OnPress {

    /// Invokes a callback function.
    pub fn call(callback: impl Fn(&mut World) + Send + Sync + 'static) -> Self {
        Self(Arc::new(callback))
    }

    /// Spawns an entity with a task created by the producer.
    pub fn task<P, T>(guard: bool, producer: P) -> Self
    where
        P: Fn() -> T + Send + Sync + 'static,
        T: Task,
    {
        if guard {
            let lock = TaskLock::new();
            let new_producer = move || {
                let task = producer();
                let lock = lock.clone();
                let task = Guard::new(task, lock);
                task
            };
            Self::call(move |world| {
                let task = new_producer();
                let runner = TaskRunner::new(task);
                world.spawn(runner);
            })
        }
        else {
            Self::call(move |world| {
                world.spawn(TaskRunner::from(producer()));
            })
        }
    }
}