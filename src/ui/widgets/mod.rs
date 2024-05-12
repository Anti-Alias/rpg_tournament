mod components;
pub use components::*;

use crate::dsl::{TreeBuilder, Class, AssetClass};
use std::time::Duration;
use bevy::prelude::*;

/// Node widget.
pub fn node(class: impl Class<NodeBundle>, t: &mut TreeBuilder) {
    let mut bundle = NodeBundle::default();
    class.apply(&mut bundle);
    t.spawn(bundle);
}

/// Text widget.
pub fn text(
    value: impl Into<String>,
    class: impl AssetClass<TextStyle>,
    assets: &AssetServer,
    t: &mut TreeBuilder
) {
    let mut text_style = TextStyle::default();
    class.apply(&assets, &mut text_style);
    let section = TextSection::new(value, text_style);
    let mut bundle = TextBundle::default();
    bundle.text.sections.push(section);
    t.spawn(bundle);
}

/// Advancing text widget.
pub fn advancing_text(
    value: impl Into<String>,
    char_duration_secs: f32,
    class: impl AssetClass<TextStyle>,
    assets: &AssetServer,
    t: &mut TreeBuilder
) {
    let mut text_style = TextStyle::default();
    class.apply(&assets, &mut text_style);
    let section_pairs = gen_section_pairs([TextSection::new(value, text_style)]);
    let mut bundle = TextBundle::default();
    bundle.text.sections = section_pairs;
    let advancer = TextAdvancer::new(Duration::from_secs_f32(char_duration_secs));
    t.spawn((bundle, advancer));
}

/// Image widget.
pub fn image(
    class: impl AssetClass<ImageBundle>,
    assets: &AssetServer,
    t: &mut TreeBuilder
) {
    let mut bundle = ImageBundle::default();
    class.apply(assets, &mut bundle);
    t.spawn(bundle);
}

/// Nine patch widget.
pub fn patch(
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    class: impl AssetClass<ImageBundle>,
    assets: &AssetServer,
    t: &mut TreeBuilder
) {
    let scale_mode = ImageScaleMode::Sliced(TextureSlicer {
        border: BorderRect { left, right, top, bottom },
        ..default()
    });
    let mut bundle = ImageBundle::default();
    class.apply(assets, &mut bundle);
    t.spawn((bundle, scale_mode));
}

pub fn menu_button(
    txt: impl Into<String>,
    assets: &AssetServer,
    t: &mut TreeBuilder,
) {

    // Spawns button
    let mut bundle = ButtonBundle::default();
    c_wood(assets, &mut bundle);
    t.spawn((bundle, nine_patch_scale_mode(7.0, 7.0, 15.0, 15.0)));

    // Spawns text
    t.begin();
    text(txt, c_title_font, assets, t);
    t.end();
}


// ------------------ Utils ------------------
fn nine_patch_scale_mode(left: f32, right: f32, top: f32, bottom: f32) -> ImageScaleMode {
    ImageScaleMode::Sliced(TextureSlicer {
        border: BorderRect { left, right, top, bottom },
        ..default()
    })
}


// ------------------ Classes ------------------
pub fn c_wood(a: &AssetServer, b: &mut ButtonBundle) {
    b.style.justify_content = JustifyContent::Center;
    b.style.align_items = AlignItems::Center;
    b.style.width = Val::Px(100.0);
    b.style.height = Val::Px(40.0);
    b.style.margin = UiRect::all(Val::Px(2.0));
    b.background_color = Color::rgb(0.8, 0.8, 0.8).into();
    b.image = a.load::<Image>("ui/wood_button.png").into();
}

fn c_title_font(a: &AssetServer, s: &mut TextStyle) {
    s.font = a.load("ui/yoster.ttf");
    s.font_size = 12.0;
}