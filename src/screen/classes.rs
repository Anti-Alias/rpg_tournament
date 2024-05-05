use bevy::prelude::*;

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

pub fn c_options_root(b: &mut NodeBundle) {
    let s = &mut b.style;
    s.display = Display::Flex;
    s.flex_direction = FlexDirection::Column;
    s.justify_content = JustifyContent::Center;
    s.align_items = AlignItems::Center;
    s.width = Val::Percent(100.0);
    s.height = Val::Percent(100.0);
    b.background_color = Color::rgb(0.7, 0.7, 0.7).into();
}

pub fn c_font(_a: &AssetServer, s: &mut TextStyle) {
    s.font_size = 64.0;
}
