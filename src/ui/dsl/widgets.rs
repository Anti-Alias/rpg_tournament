use bevy::prelude::*;
use super::Dsl;

#[derive(Clone, Default, Debug, Deref, DerefMut)]
pub struct NodeW(pub NodeBundle);
impl NodeW {
    pub fn new() -> Self {
        Self(NodeBundle::default())
    }
    pub fn cls(class: impl Fn(&mut Self)) -> Self {
        let mut result = Self::default();
        class(&mut result);
        result
    }
    pub fn bgcolor(mut self, color: Color) -> Self {
        self.background_color = color.into();
        self
    }
    pub fn spawn(self, dsl: &mut Dsl) {
        dsl.spawn(self.0);
    }
    pub fn begin(self, dsl: &mut Dsl) {
        dsl.begin(self.0);
    }
    pub fn end(dsl: &mut Dsl) {
        dsl.end()
    }
    pub fn class(mut self, class: impl Fn(&mut Self)) -> Self {
        class(&mut self);
        self
    }
}

#[derive(Default, Debug, Deref, DerefMut)]
pub struct TextW(pub TextBundle);
impl TextW {
    pub fn new(text: impl Into<String>) -> Self {
        Self(TextBundle::from_section(text, TextStyle::default()))
    }
    pub fn spawn(self, dsl: &mut Dsl) {
        dsl.spawn(self.0);
    }
    pub fn font(mut self, font: &Handle<Font>) -> Self {
        self.set_font(font);
        self
    }
    pub fn font_size(mut self, font_size: f32) -> Self {
        self.set_font_size(font_size);
        self
    }
    pub fn end(dsl: &mut Dsl) {
        dsl.end()
    }
    pub fn class(mut self, class: impl Fn(&mut Self)) -> Self {
        class(&mut self);
        self
    }
    pub fn set_color(&mut self, color: Color) {
        for text_section in &mut self.0.text.sections {
            text_section.style.color = color;
        }
    }
    pub fn set_font(&mut self, font: &Handle<Font>) {
        for text_section in &mut self.0.text.sections {
            text_section.style.font = font.clone();
        }
    }
    pub fn set_font_size(&mut self, font_size: f32) {
        for text_section in &mut self.0.text.sections {
            text_section.style.font_size = font_size;
        }
    }
}

#[derive(Clone, Default, Debug, Deref, DerefMut)]
pub struct WButton(pub ButtonBundle);
impl WButton {
    pub fn begin(self, dsl: &mut Dsl) {
        dsl.begin(self.0);
    }
    pub fn end(dsl: &mut Dsl) {
        dsl.end();
    }
    pub fn class(mut self, class: impl Fn(&mut Self)) -> Self {
        class(&mut self);
        self
    }
}

#[derive(Default, Debug, Deref, DerefMut)]
pub struct ImageW(ImageBundle);
impl ImageW {
    pub fn new(image: &Handle<Image>) -> Self {
        Self(ImageBundle {
            image: UiImage { texture: image.clone(), ..default() },
            ..default()
        })
    }
    pub fn spawn(self, dsl: &mut Dsl) {
        dsl.spawn(self.0);
    }
    pub fn begin(self, dsl: &mut Dsl) {
        dsl.begin(self.0);
    }
    pub fn end(dsl: &mut Dsl) {
        dsl.end();
    }
    pub fn class(mut self, class: impl Fn(&mut Self)) -> Self {
        class(&mut self);
        self
    }
}

#[derive(Default, Debug)]
pub struct SlicedImageW {
    pub image: ImageBundle,
    pub slicer: TextureSlicer,
    pub atlas: Option<TextureAtlas>,
}
impl SlicedImageW {
    pub fn new(image: &Handle<Image>) -> Self {
        Self {
            slicer: TextureSlicer::default(),
            image: ImageBundle {
                image: UiImage { texture: image.clone(), ..default() },
                ..default()
            },
            atlas: None,
        }
    }
    pub fn region(mut self, region: URect, image_size: UVec2, atlases: &mut Assets<TextureAtlasLayout>) -> Self {
        self.set_region(region, image_size, atlases);
        self
    }
    pub fn spawn(self, dsl: &mut Dsl) {
        match self.atlas {
            Some(atlas) => dsl.spawn((self.image, atlas, ImageScaleMode::Sliced(self.slicer))),
            None        => dsl.spawn((self.image, ImageScaleMode::Sliced(self.slicer))),
        }
    }
    pub fn begin(self, dsl: &mut Dsl) {
        match self.atlas {
            Some(atlas) => dsl.begin((self.image, atlas, ImageScaleMode::Sliced(self.slicer))),
            None        => dsl.begin((self.image, ImageScaleMode::Sliced(self.slicer))),
        }
    }
    pub fn end(dsl: &mut Dsl) {
        dsl.end();
    }
    pub fn class(mut self, class: impl Fn(&mut Self)) -> Self {
        class(&mut self);
        self
    }
    pub fn set_region(&mut self, region: URect, image_size: UVec2, atlases: &mut Assets<TextureAtlasLayout>) {
        let mut hair_atlas = TextureAtlasLayout::new_empty(image_size);
        hair_atlas.add_texture(region);
        self.atlas = Some(TextureAtlas { layout: atlases.add(hair_atlas), index: 0 });
    }
}

#[derive(Debug)]
pub struct TextButtonW<E: Event + Clone = Nop> {
    pub button: ButtonBundle,
    pub text: TextW,
    pub on_press: Option<E>,
}

impl<E: Event + Clone> Default for TextButtonW<E> {
    fn default() -> Self {
        Self {
            button: ButtonBundle::default(),
            text: TextW::default(),
            on_press: None,
        }
    }
}

impl<E: Event + Clone> TextButtonW<E> {

    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: TextW(TextBundle::from_section(text, TextStyle::default())),
            ..default()
        }
    }
    pub fn on_press(mut self, on_click: E) -> Self {
        self.on_press = Some(on_click);
        self
    }
    pub fn font(mut self, font: &Handle<Font>) -> Self {
        self.set_font(font);
        self
    }
    pub fn font_size(mut self, font_size: f32) -> Self {
        self.set_font_size(font_size);
        self
    }
    pub fn set_font(&mut self, font: &Handle<Font>) {
        for text_section in &mut self.text.text.sections {
            text_section.style.font = font.clone();
        }
    }
    pub fn set_font_size(&mut self, font_size: f32) {
        for text_section in &mut self.text.text.sections {
            text_section.style.font_size = font_size;
        }
    }
    pub fn spawn(self, dsl: &mut Dsl) {
        let id: Entity;
        dsl.begin(self.button); id=dsl.last();
            dsl.spawn(self.text.0);
        dsl.end();
        if let Some(on_press) = self.on_press {
            dsl.commands.entity(id).insert(OnPress(on_press));
        }
    }
    pub fn end(dsl: &mut Dsl) {
        dsl.end()
    }
    pub fn class(mut self, class: impl Fn(&mut Self)) -> Self {
        class(&mut self);
        self
    }
    pub fn text_class(mut self, class: impl Fn(&mut TextW)) -> Self {
        class(&mut self.text);
        self
    }
}

pub fn handle_interactions<E: Event + Clone>(
    interactions: Query<
        (&Interaction, Option<&OnPress<E>>),
        (Changed<Interaction>, With<Button>),
    >,
    mut commands: Commands,
) {
    for (interaction, on_press) in &interactions {
        match *interaction {
            Interaction::Pressed => if let Some(on_press) = on_press {
                commands.trigger(on_press.0.clone());
            },
            Interaction::Hovered => {},
            Interaction::None => {},
        }
    }
}

#[derive(Component, Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct OnPress<E: Event + Clone>(pub E);

#[derive(Event, Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Nop;