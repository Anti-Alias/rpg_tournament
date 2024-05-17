use crate::batch::*;
use crate::ext::*;
use crate::screen::*;
use crate::task::Start;
use crate::ui::*;
use crate::dsl::*;
use bevy::ecs::system::CommandQueue;
use bevy::prelude::*;


pub fn setup_options_screen(mut commands: Commands, mut scale: ResMut<UiScale>) {
    scale.0 = 2.0;      
    commands.spawn(Camera2dBundle::default());
    commands.spawn_task(Start::new(|_, tq| {
        tq.spawn_batch(spawn_options_menu);
        tq.send_event(ScreenEvent::FinishedLoading);
    }));
}

fn spawn_options_menu(world: &mut World, commands: &mut CommandQueue, assets: &mut AssetBatch) {
    let graphics: Entity;
    let sound: Entity;
    let back: Entity;

    let commands = &mut Commands::new(commands, world);
    let t = &mut TreeBuilder::root(commands);
    node(c_options_root, t); begin(t);
        menu_button("Graphics", assets, t); graphics = last(t);
        menu_button("Sound", assets, t);    sound = last(t);
        menu_button("Back", assets, t);     back = last(t);
    end(t);
    commands.entity(graphics).on_press(|_| println!("Graphics pressed!"));
    commands.entity(sound).on_press(|_| println!("Sound pressed!"));
    commands.entity(back).on_press(|w| w.spawn_task(FadeToScreen(ScreenState::Title)));
}



fn c_options_root(b: &mut NodeBundle) {
    let s = &mut b.style;
    s.display = Display::Flex;
    s.flex_direction = FlexDirection::Column;
    s.justify_content = JustifyContent::Center;
    s.align_items = AlignItems::Center;
    s.width = Val::Percent(100.0);
    s.height = Val::Percent(100.0);
    b.background_color = Color::rgb(0.2, 0.7, 0.7).into();
}