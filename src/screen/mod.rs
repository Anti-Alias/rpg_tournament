mod tasks;
mod classes;

use classes::*;
use bevy::prelude::*;
pub use tasks::*;
use crate::task::{Task, TaskCtx, TaskRunner};
use crate::tree::*;


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
    let t = &mut TreeBuilder::new(&mut commands);
    node(c_title_root, t); insert(Despawnable, t); begin(t);
        node(c_menu_items, t); begin(t);
            text("New Game", c_font, &assets, t);
            text("Continue", c_font, &assets, t);
            text("Options", c_font, &assets, t);
            text("Exit", c_font, &assets, t);
        end(t);
        text("Super Exit", c_font, &assets, t);
    end(t);
}

fn spawn_options_screen(mut commands: Commands, assets: Res<AssetServer>) {
    let t = &mut TreeBuilder::new(&mut commands);
    node(c_title_root, t); insert(Despawnable, t); begin(t);
        text("Graphics", c_font, &assets, t);
        text("Sound", c_font, &assets, t);
        text("Back", c_font, &assets, t);
    end(t);
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