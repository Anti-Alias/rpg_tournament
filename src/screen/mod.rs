mod tasks;
mod classes;
mod tree;

use bevy::prelude::*;
//use bevy_ui_dsl::*;

pub use tasks::*;
use classes::*;
use crate::task::{Task, TaskCtx, TaskRunner};

use self::tree::TreeBuilder;

pub fn screen_plugin(app: &mut App) {
    app.insert_state(ScreenState::Title);
    app.add_systems(OnEnter(ScreenState::Title), spawn_title_screen);
    app.add_systems(OnEnter(ScreenState::Options), spawn_options_screen);
    app.add_systems(Startup, on_startup);
}

/// State that controls what screen is being displayed.
#[derive(States, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ScreenState {
    Title,
    Options,
    Map { map_file: String, }
}

/// Marks an entity as despawnable during screen transitions.
#[derive(Component, Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Despawnable;

fn spawn_title_screen(mut commands: Commands, assets: Res<AssetServer>) {
    // rooti(c_title_root, &assets, &mut commands, Despawnable, |p| {
    //     text("New Game", (), c_font, p);
    //     text("Continue", (), c_font, p);
    //     text("Options", (), c_font, p);
    //     text("Exit", (), c_font, p);
    // });
    let t = &mut TreeBuilder::new(&mut commands);
    
}

fn spawn_options_screen(mut commands: Commands, assets: Res<AssetServer>) {
    // rooti(c_options_root, &assets, &mut commands, Despawnable, |p| {
    //     text("Graphics", (), c_font, p);
    //     text("Sound", (), c_font, p);
    //     text("Back", (), c_font, p);
    // });
}

fn on_startup(mut commands: Commands) {
    commands.spawn(TaskRunner::update(MyTask));
}

struct MyTask;
impl Task for MyTask {
    fn start(&mut self, _world: &mut World, mut ctx: TaskCtx) {
        ctx.wait(1.5);
        ctx.push(FadeToScreen::new(ScreenState::Options, 1.5, 1.5));
        ctx.wait(1.5);
        ctx.push(FadeToScreen::new(ScreenState::Title, 1.5, 1.5));
        ctx.quit(true);
    }
}