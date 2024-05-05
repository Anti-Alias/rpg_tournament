use bevy::prelude::*;

pub fn c_title_root(b: &mut NodeBundle) {
    let s = &mut b.style;
    s.width = Val::Percent(100.0);
    s.height = Val::Percent(100.0);
    b.background_color = Color::YELLOW.into();
}

pub fn c_options_root(b: &mut NodeBundle) {
    let s = &mut b.style;
    s.width = Val::Percent(100.0);
    s.height = Val::Percent(100.0);
    b.background_color = Color::GREEN.into();
}