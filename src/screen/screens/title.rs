use bevy::ecs::system::CommandQueue;
use bevy::prelude::*;
use crate::spawn::AssetBatch;
use crate::ext::{CommandsExt, EntityCommandsExt, WorldExt};
use crate::screen::*;
use crate::ui::*;
use crate::task::*;
use crate::dsl::*;

pub fn setup_title_screen(mut commands: Commands) {
    commands.spawn_task(Start::new(|_, tq| {
        tq.spawn_batch(spawn_title_menu);
        tq.send_event(ScreenEvent::FinishedLoading);
    }));
}

fn spawn_title_menu(
    world: &mut World,
    commands: &mut CommandQueue,
    assets: &mut AssetBatch
) {
    let mut commands = Commands::new(commands, world);
    let t = &mut TreeBuilder::root(&mut commands);
    let new_game: Entity;
    let cont: Entity;
    let options: Entity;
    node(c_root, t); insert(Name::new("Title UI"), t); begin(t);
        menu_button("New Game", assets, t); new_game=last(t);
        menu_button("Continue", assets, t); cont=last(t);
        menu_button("Options", assets, t);  options=last(t);
        menu_button("Exit", assets, t);
    end(t);

    commands.entity(new_game).on_press(|world| {
        let task = FadeToScreen(ScreenState::Playground);
        world.spawn_task(task);
    });
    commands.entity(options).on_press(|world| {
        let task = FadeToScreen(ScreenState::Options);
        world.spawn_task(task);
    });

    let lock = TaskLock::new();
    commands.entity(cont).on_press(move |world| {
        let task = Guard::new(ShowDialog, lock.clone());
        world.spawn_task(task);
    });
}

pub struct ShowDialog;
impl Task for ShowDialog {

    fn start(&mut self, world: &mut World, tq: &mut TaskQueue) {
        const WAIT_TIME: u64 = 1000;
        let diag_container: Entity = world.spawn_empty().id();
        let text: Entity = world.spawn_empty().id();
        tq.spawn_dialog("Basic sprite rendering and animations now work.", diag_container, text);
        tq.wait_on_text(text);
        tq.wait_millis(WAIT_TIME);
        tq.set_dialog_message("Click on \"New Game\" to try it out.", text);
        tq.wait_on_text(text);
        tq.wait_millis(WAIT_TIME);
        tq.despawn(text, true, true);
        tq.despawn(diag_container, true, true);
    }
}

pub fn c_root(b: &mut NodeBundle) {
    let s = &mut b.style;
    s.display = Display::Flex;
    s.flex_direction = FlexDirection::Column;
    s.justify_content = JustifyContent::Center;
    s.align_items = AlignItems::Center;
    s.width = Val::Percent(100.0);
    s.height = Val::Percent(100.0);
    b.background_color = Color::rgb(0.5, 0.5, 0.5).into();
}