use std::time::Duration;
use bevy::prelude::*;
use crate::batch::AssetBatch;
use crate::ui::*;
use crate::dsl::*;

pub fn spawn_dialog(message: &str, dialog_id: Entity, text_id: Entity, commands: &mut Commands, assets: &mut AssetBatch) {
    let t = &mut TreeBuilder::root(commands);
    node(c_fullscreen, t); begin(t);
        next(dialog_id, t);
        dialog(message, 0.05, assets, text_id, t);
    end(t);
}

pub fn set_dialog_message(message: &str, text_id: Entity, commands: &mut Commands, assets: &mut AssetBatch) {
    let mut style = TextStyle::default();
    c_dialog_font(assets, &mut style);
    let section_pairs = gen_section_pairs([TextSection::new(message, style)]);
    commands.entity(text_id).insert((
        Text::from_sections(section_pairs),
        TextAdvancer::new(Duration::from_secs_f32(0.05)),
    ));
}

// ---------------- Widgets ---------------------

pub fn dialog(txt: impl Into<String>, char_duration_secs: f32, assets: &mut AssetBatch, text_id: Entity, t: &mut TreeBuilder) {
    fn c_patch(a: &mut AssetBatch, b: &mut ImageBundle) {
        let s = &mut b.style;
        s.justify_content = JustifyContent::Stretch;
        s.align_items = AlignItems::Center;
        s.width = Val::Px(200.0);
        s.height = Val::Px(60.0);
        s.margin = UiRect::all(Val::Px(2.0));
        s.position_type = PositionType::Absolute;
        s.bottom = Val::Px(0.0);
        b.background_color = Color::rgb(0.8, 0.8, 0.8).into();
        b.image = a.load::<Image>("ui/metal_box.png").into();
    }
    fn c_node(b: &mut NodeBundle) {
        b.style.width = Val::Px(40.0);
        b.style.height = Val::Px(40.0);
        b.style.margin = UiRect::all(Val::Px(5.0));
        b.style.flex_grow = 1.0;
    }
    patch(7.0, 7.0, 15.0, 15.0, c_patch, assets, t); begin(t);
        avatar_box(assets, t);
        node(c_node, t); begin(t);
            next(text_id, t);
            advancing_text(txt, char_duration_secs, c_dialog_font, assets, t);
        end(t);
    end(t);
}

fn avatar_box(assets: &mut AssetBatch, t: &mut TreeBuilder) {
    fn class(assets: &mut AssetBatch, b: &mut ImageBundle) {
        b.style.width = Val::Px(40.0);
        b.style.height = Val::Px(40.0);
        b.style.margin = UiRect::all(Val::Px(5.0));
        b.image = assets.load("ui/avatar_box.png").into();
    }
    patch(7.0, 7.0, 12.0, 12.0, class, assets, t);
}

// -------------------- Classes ----------------------
fn c_dialog_font(a: &mut AssetBatch, s: &mut TextStyle) {
    s.font = a.load("ui/yoster.ttf");
    s.font_size = 12.0;
}

fn c_fullscreen(b: &mut NodeBundle) {
    let s = &mut b.style;
    s.width = Val::Percent(100.0);
    s.height = Val::Percent(100.0);
    s.justify_content = JustifyContent::Center;
    s.align_items = AlignItems::Center;
}
