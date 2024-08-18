mod dsl;

pub use dsl::*;
use bevy::prelude::*;
use messages::{MenuEvent, ToggleEquipmentMenu};
use crate::equipment::{Equippable, Hair, HairKind, Outfit};

const FONT_SIZE: f32 = 14.0;
const ICON_SIZE: UVec2 = UVec2::new(32, 28);

#[derive(Resource, Reflect, Debug)]
pub struct EquipmentMenu {
    pub entity: Entity,
    pub hair_list: Vec<Equippable>,
    pub hair_selected: usize,
    pub outfit_list: Vec<Equippable>,
    pub outfit_selected: usize,
}


pub fn toggle_equipment_menu(
    _trigger: Trigger<ToggleEquipmentMenu>,
    equipment_menu: Option<Res<EquipmentMenu>>,
    mut commands: Commands,
) {
    match equipment_menu {
        Some(equipment_menu) => {
            commands.entity(equipment_menu.entity).despawn_recursive();
            commands.remove_resource::<EquipmentMenu>();
        },
        None => {
            let fullscreen_node = create_node(c_fullscreen);
            let entity = commands.spawn(fullscreen_node).id();
            commands.insert_resource(EquipmentMenu {
                entity,
                hair_selected: 0,
                outfit_selected: 0,
                hair_list: vec![
                    Hair { kind: HairKind::Spikey, ..default() }.into(),
                    Hair { kind: HairKind::Bob, ..default() }.into(),
                    Hair { kind: HairKind::Ponytail, ..default() }.into(),
                ],
                outfit_list: vec![
                    Outfit::Casual1.into(),
                    Outfit::Casual2.into(),
                    Outfit::Casual3.into(),
                    Outfit::Casual4.into(),
                    Outfit::Casual5.into(),
                ],
            });
        },
    }
}

pub fn handle_menu_events(
    trigger: Trigger<MenuEvent>,
    equipment_menu: Option<ResMut<EquipmentMenu>>,
) {
    let Some(mut menu) = equipment_menu else { return };
    let (hair_sel, hair_len) = (menu.hair_selected, menu.hair_list.len());
    let (outfit_sel, outfit_len) = (menu.outfit_selected, menu.outfit_list.len());
    if hair_len == 0 { return };
    match trigger.event() {
        MenuEvent::NextHair         => menu.hair_selected = (hair_sel + 1).rem_euclid(hair_len),
        MenuEvent::PreviousHair     => menu.hair_selected = (hair_sel + hair_len - 1).rem_euclid(hair_len),
        MenuEvent::NextOutfit       => menu.outfit_selected = (outfit_sel + 1).rem_euclid(outfit_len),
        MenuEvent::PreviousOutfit   => menu.outfit_selected = (outfit_sel + outfit_len - 1).rem_euclid(outfit_len),
    }
}


pub fn render_equipment_menu(
    menu: Res<EquipmentMenu>,
    assets: Res<AssetServer>,
    mut atluses: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands
) {
    commands.entity(menu.entity).despawn_descendants();
    let d = &mut Dsl::new(&mut commands, menu.entity);
    let font = assets.load::<Font>("fonts/Retro Gaming.ttf");
    let frame = assets.load::<Image>("ui/UI_Paper_Frame_01_Standard.png");

    let sel_hair = &menu.hair_list[menu.hair_selected];
    let sel_hair_info = sel_hair.info();
    let sel_hair_name = format!("{}", sel_hair_info.name);
    let sel_hair_image = assets.load::<Image>(sel_hair_info.image);
    let hair_region = icon_region(16, 4);

    let sel_outfit = &menu.outfit_list[menu.outfit_selected];
    let sel_outfit_info = sel_outfit.info();
    let sel_outfit_name = format!("{}", sel_outfit_info.name);
    let sel_outfit_image = assets.load::<Image>(sel_outfit_info.image);
    let outfit_region = icon_region(16, 16);

    SlicedImageW::new(&frame).class(c_frame).begin(d);
        TextW::new("Equipment").class(c_text).class(c_title).font(&font).font_size(FONT_SIZE).spawn(d);
        NodeW::cls(c_body).begin(d);
        
            NodeW::cls(c_row).begin(d);
                TextButtonW::new("<").text_class(c_next_prev).font(&font).font_size(FONT_SIZE).on_press(MenuEvent::PreviousHair).spawn(d);
                SlicedImageW::new(&sel_hair_image).region(hair_region, UVec2::new(512, 512), &mut atluses).class(c_icon).spawn(d);
                TextButtonW::new(">").text_class(c_next_prev).font(&font).font_size(FONT_SIZE).on_press(MenuEvent::NextHair).spawn(d);
                TextW::new(sel_hair_name).class(c_text).font(&font).font_size(FONT_SIZE).spawn(d);
            NodeW::end(d);

            NodeW::cls(c_row).begin(d);
                TextButtonW::new("<").text_class(c_next_prev).font(&font).font_size(FONT_SIZE).on_press(MenuEvent::PreviousOutfit).spawn(d);
                SlicedImageW::new(&sel_outfit_image).region(outfit_region, UVec2::new(512, 512), &mut atluses).class(c_icon).spawn(d);
                TextButtonW::new(">").text_class(c_next_prev).font(&font).font_size(FONT_SIZE).on_press(MenuEvent::NextOutfit).spawn(d);
                TextW::new(sel_outfit_name.clone()).class(c_text).font(&font).font_size(FONT_SIZE).spawn(d);
            NodeW::end(d);

        NodeW::end(d);
    SlicedImageW::end(d);
}

fn c_frame(image: &mut SlicedImageW) {
    let style = &mut image.image.style;
    let slicer = &mut image.slicer;
    style.width = Val::Px(250.0);
    style.height = Val::Px(150.0);
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Stretch;
    style.align_items = AlignItems::Stretch;
    style.padding = UiRect::top(Val::Px(100.0));
    slicer.border = BorderRect { left: 20.0, right: 20.0, top: 20.0, bottom: 20.0 };
    slicer.center_scale_mode = SliceScaleMode::Tile { stretch_value: 1.0 };
    slicer.sides_scale_mode = SliceScaleMode::Tile { stretch_value: 1.0 };    
}

fn c_icon(image: &mut SlicedImageW) {
    image.image.style.width = Val::Px(ICON_SIZE.x as f32);
    image.image.style.height = Val::Px(ICON_SIZE.y as f32);
}

fn c_body(node: &mut NodeW) {
    node.style.flex_direction = FlexDirection::Column;
    node.style.justify_content = JustifyContent::Center;
    node.style.align_items = AlignItems::Center;
    node.style.flex_grow = 1.0;
}

fn c_row(node: &mut NodeW) {
    node.style.justify_content = JustifyContent::Start;
    node.style.align_items = AlignItems::Center;
    node.style.width = Val::Px(170.0);
}

fn c_fullscreen(node: &mut NodeBundle) {
    node.style.width = Val::Percent(100.0);
    node.style.height = Val::Percent(100.0);
    node.style.justify_content = JustifyContent::Center;
    node.style.align_items = AlignItems::Center;
}

fn c_text(text: &mut TextW) {
    text.text.justify = JustifyText::Center;
    text.set_color(Srgba::from_u8_array([64, 32, 0, 255]).into());
}

fn c_next_prev(text: &mut TextW) {
    text.style.margin = UiRect::horizontal(Val::Px(5.0));
    text.set_color(Srgba::from_u8_array([64, 32, 0, 255]).into());
}

fn c_title(text: &mut TextW) {
    text.style.top = Val::Px(10.0);
}


fn create_node(class: impl Fn(&mut NodeBundle)) -> NodeBundle {
    let mut bundle = NodeBundle::default();
    class(&mut bundle);
    bundle
}



const fn icon_region(x: u32, y: u32) -> URect {
    URect {
        min: UVec2::new(x, y),
        max: UVec2::new(x + ICON_SIZE.x, y + ICON_SIZE.y),
    }
}


pub mod messages {

    use bevy::prelude::*;

    /// Shows / hides equipment menu
    #[derive(Event, Copy, Clone, Eq, PartialEq, Default, Debug)]
    pub struct ToggleEquipmentMenu;

    /// Updates equipment menu state, triggering a UI re-render.
    #[derive(Event, Copy, Clone, PartialEq, Debug)]
    pub enum MenuEvent {
        NextHair,
        PreviousHair,
        NextOutfit,
        PreviousOutfit,
    }
}