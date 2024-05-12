use bevy::prelude::*;
use std::time::Duration;

/// System that advances text on a [`Text`] node.
pub fn advance_text(
    mut text_entities: Query<(&mut TextAdvancer, &mut Text)>,
    time: Res<Time>,
) {
    let delta = time.delta();
    for (mut advancer, mut text) in &mut text_entities {
        if advancer.finished { continue }
        advancer.char_timer.tick(delta);
        for _ in 0..advancer.char_timer.times_finished_this_tick() {
            advancer.finished = advance_section_pairs(&mut text.sections);
        }
    }
}

pub fn gen_section_pairs(sections: impl IntoIterator<Item = TextSection>) -> Vec<TextSection> {
    let mut result = vec![];
    for section in sections {
        let sec_style = section.style.clone();
        let mut pair_right = section;
        pair_right.style.color = Color::NONE;
        let pair_left = TextSection::from_style(sec_style);
        result.extend([pair_left, pair_right]);
    }
    result
}

#[derive(Bundle, Default)]
pub struct TextAdvancerBundle {
    pub text_advancer: TextAdvancer,
    pub text: Text,
}

#[derive(Component, Debug, Default)]
pub struct TextAdvancer {
    char_timer: Timer,
    finished: bool,
}

impl TextAdvancer {
    
    pub fn new(char_duration: Duration) -> Self {
        Self {
            char_timer: Timer::new(char_duration, TimerMode::Repeating),
            finished: false,
        }
    }

    pub fn finished(&self) -> bool {
        self.finished
    }
}

fn advance_section_pairs(pairs: &mut [TextSection]) -> bool {
    for i in 0..pairs.len() / 2 {
        let right = &mut pairs[i*2 + 1];
        if right.value.is_empty() { continue };
        let c = right.value.remove(0);
        let left = &mut pairs[i*2];
        left.value.push(c);
    }
    match pairs.last() {
        Some(section) => section.value.is_empty(),
        None => true,
    }
}