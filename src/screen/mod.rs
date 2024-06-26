mod tasks;
mod screens;

pub use tasks::*;

use crate::task::{collect_children_recursive, Task, TaskQueue, TaskRunner};
use crate::GameState;
use bevy::prelude::*;


/// State that controls what screen is being displayed.
#[derive(States, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ScreenState { Title, Options, Playground }

pub fn screen_plugin(app: &mut App) {
    app.insert_state(ScreenState::Playground);
    app.add_systems(OnEnter(ScreenState::Title),        screens::title::setup_title_screen);
    app.add_systems(OnEnter(ScreenState::Options),      screens::options::setup_options_screen);
    app.add_systems(OnEnter(ScreenState::Playground),   screens::playground::setup_playground_screen);
    app.add_systems(OnExit(ScreenState::Title),         despawn_all);
    app.add_systems(OnExit(ScreenState::Options),       despawn_all);
    app.add_systems(OnExit(ScreenState::Playground),    despawn_all);
    app.add_event::<ScreenEvent>();
}

/// Despawns all entities that don't have a [`Keep`], and don't have an ancestor with a [`Keep`].
/// Also, does not despawn important internal bevy entities.
fn despawn_all(
    world: &mut World,
    despawnables: &mut QueryState<Entity, (Without<Window>, Without<Keep>, Without<Parent>)>
) {
    let mut to_despawn = vec![];
    for entity in despawnables.iter(world) {
        to_despawn.push(entity);
        collect_children_recursive(world, entity, &mut to_despawn);
    }
    for entity in to_despawn {
        let Some(mut w_entity) = world.get_entity_mut(entity) else { continue };
        if let Some(mut task_runner) = w_entity.take::<TaskRunner>() {
            task_runner.clear(world);
        };
        world.despawn(entity);
    }
}


/// Keeps this entity across screen transitions.
#[derive(Component)]
pub struct Keep;

/// Fired when a screen finishes loading.
#[derive(Event, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum ScreenEvent {
    FinishedLoading,
}