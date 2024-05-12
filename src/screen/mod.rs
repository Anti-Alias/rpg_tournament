mod tasks;
mod screens;

pub use tasks::*;

use crate::task::ext::ExtTaskQueue;
use crate::task::{Task, TaskQueue};
use crate::GameState;
use bevy::prelude::*;



pub fn screen_plugin(app: &mut App) {
    app.insert_state(ScreenState::Title);
    app.add_systems(OnEnter(ScreenState::Title),    screens::title::setup_title_screen);
    app.add_systems(OnEnter(ScreenState::Options),  screens::options::setup_options_screen);
    app.add_systems(OnExit(ScreenState::Title),     despawn_all);
    app.add_systems(OnExit(ScreenState::Options),   despawn_all);
}

/// Despawns root entities without a [`Keep`] component.
fn despawn_all(
    mut commands: Commands,
    despawnables: Query<Entity, (Without<Window>, Without<Keep>, Without<Parent>)>
) {
    for entity in &despawnables {
        commands.entity(entity).despawn_recursive();
    }
}


/// State that controls what screen is being displayed.
#[derive(States, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ScreenState {
    Title,
    Options,
    //Map { map_file: String, }
}

/// Keeps this entity across screen transitions.
#[derive(Component)]
pub struct Keep;

pub struct FadeToScreen(pub ScreenState);
impl Task for FadeToScreen {
    fn start(&mut self, _world: &mut World, tq: &mut TaskQueue) {
        let mut tq = ExtTaskQueue(tq);
        let screen_state = self.0.clone();
        tq.quit_if_state(GameState::Transitioning, true);
        tq.set_state(GameState::Transitioning);
        tq.start(move |world, tq| {
            let mut tq = ExtTaskQueue(tq);
            let fade_id = world.spawn(Keep).id();
            tq.insert_host(Keep);
            tq.fade_in(fade_id, Color::BLACK, 0.25);
            tq.set_state(screen_state);
            tq.fade_out(fade_id, 0.25);
            tq.set_state(GameState::Running);
            tq.quit(true);
        });
    }
}