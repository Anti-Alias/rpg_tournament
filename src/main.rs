mod screen;
mod task;
use bevy::prelude::*;
use screen::{FadeIn, FadeOut, ScreenState};
use task::{Task, TaskCtx, TaskPlugin, TaskRunner};
use crate::screen::FadeState;
use crate::task::Shared;


fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TaskPlugin::new(Update)))
        .insert_state(ScreenState::Title)
        .add_systems(Startup, startup)
        .run();
}


fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(TaskRunner::update(MyTask));

}

struct MyTask;
impl Task for MyTask {
    fn start(&mut self, _world: &mut World, mut ctx: TaskCtx) {
        let state = Shared::new(FadeState::new());
        ctx.push(FadeIn::new(Color::RED, 1.5, state.clone()));
        ctx.push(FadeOut::new(1.5, state.clone()));
        ctx.quit(true);
    }
}