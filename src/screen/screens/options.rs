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
    node(c_options_root, t); begin(t);
        menu_button("Graphics", &assets, t);    graphics = last(t);
        menu_button("Sound", &assets, t);       sound = last(t);
        menu_button("Back", &assets, t);        back = last(t);
    end(t);

    commands.entity(graphics).insert(OnPress::call(|_| println!("Graphics pressed!")));
    commands.entity(sound).insert(OnPress::call(|_| println!("Sound pressed!")));
    commands.entity(back).insert(OnPress::task(false, || FadeToScreen(ScreenState::Title)));
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