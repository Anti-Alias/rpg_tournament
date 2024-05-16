use crate::batch::AssetBatch;
use crate::ext::EntityCommandsExt;
use crate::ext::WorldExt;
use crate::screen::*;
use crate::ui::*;
use crate::dsl::*;
use bevy::prelude::*;


pub fn setup_options_screen(mut commands: Commands, assets: Res<AssetServer>, mut scale: ResMut<UiScale>) {
    commands.spawn(Camera2dBundle::default());
    scale.0 = 2.0;      
    
    let graphics: Entity;
    let sound: Entity;
    let back: Entity;

    let t = &mut TreeBuilder::root(&mut commands);
    let assets = &mut AssetBatch::new(assets.clone());
    node(c_options_root, t); begin(t);
        menu_button("Graphics", assets, t); graphics = last(t);
        menu_button("Sound", assets, t);    sound = last(t);
        menu_button("Back", assets, t);     back = last(t);
    end(t);

    commands.entity(graphics).on_press(|_| println!("Graphics pressed!"));
    commands.entity(sound).on_press(|_| println!("Sound pressed!"));
    commands.entity(back).on_press(|w| w.spawn_task(FadeToScreen(ScreenState::Title)));
}

pub fn c_options_root(b: &mut NodeBundle) {
    let s = &mut b.style;
    s.display = Display::Flex;
    s.flex_direction = FlexDirection::Column;
    s.justify_content = JustifyContent::Center;
    s.align_items = AlignItems::Center;
    s.width = Val::Percent(100.0);
    s.height = Val::Percent(100.0);
    b.background_color = Color::rgb(0.2, 0.7, 0.7).into();
}