use bevy::prelude::*;
use crate::screen::*;
use crate::ui::*;
use crate::task::*;
use crate::dsl::*;

pub fn setup_title_screen(mut commands: Commands, assets: Res<AssetServer>, mut scale: ResMut<UiScale>) {
    commands.spawn(Camera2dBundle::default());
    scale.0 = 2.0;

    let new_game: Entity;
    let cont: Entity;
    let options: Entity;
    let exit: Entity;

    let t = &mut TreeBuilder::root(&mut commands);
    node(c_title_root, t); begin(t);
        menu_button("New Game", &assets, t); new_game = last(t);
        menu_button("Continue", &assets, t); cont = last(t);
        menu_button("Options", &assets, t);  options = last(t);
        menu_button("Exit", &assets, t);     exit = last(t);
    end(t);

    commands.entity(new_game).insert(OnPress::call(|_| println!("New game pressed!")));
    commands.entity(cont).insert(OnPress::task(true, || ShowDialog));
    commands.entity(options).insert(OnPress::task(false, || FadeToScreen(ScreenState::Options)));
    commands.entity(exit).insert(OnPress::call(|_| println!("Exit pressed!")));
}

pub struct ShowDialog;
impl Task for ShowDialog {
    fn start(&mut self, world: &mut World, tq: &mut TaskQueue) {
        let mut tq = ExtTaskQueue(tq);
        let dialog = world.spawn_empty().id();
        let text = world.spawn_empty().id();
        tq.spawn_dialog("Hello, world!", dialog, text);
        tq.wait_on_text(text);
        tq.wait_millis(500);
        tq.set_dialog_message("How are you on this fine day?", text);
        tq.wait_on_text(text);
        tq.wait_millis(500);
        tq.set_dialog_message("I'm fine, but you already knew that, didn't you?", text);
        tq.wait_on_text(text);
        tq.wait_millis(500);
        tq.despawn(dialog, true, true);
        tq.finally(|_| println!("Ran finally block!!!"));
    }
}

pub fn c_title_root(b: &mut NodeBundle) {
    let s = &mut b.style;
    s.display = Display::Flex;
    s.flex_direction = FlexDirection::Column;
    s.justify_content = JustifyContent::Center;
    s.align_items = AlignItems::Center;
    s.width = Val::Percent(100.0);
    s.height = Val::Percent(100.0);
    b.background_color = Color::rgb(0.5, 0.5, 0.5).into();
}