use bevy::prelude::*;
use smallvec::SmallVec;


/// Abstraction on virtual buttons on a controller, or the keys on a keyboard.
#[derive(Component, Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct VButtons {
    /// Bits of buttons being pressed this frame.
    pub pressed: u32,
    /// Bits of buttons being pressed the previous frame.
    pub pressed_prev: u32,
}

impl VButtons {

    /// Presses all of the virtual buttons specified.
    pub fn press(&mut self, button_bits: u32) {
        self.pressed |= button_bits;
    }

    /// True if at least once of the buttons specified is pressed.
    pub fn pressed(&self, button_bits: u32) -> bool {
        self.pressed & button_bits != 0
    }

    /// True if at least one of the buttons is pressed this frame, but none in the previous frame.
    pub fn just_pressed(&self, button_bits: u32) -> bool {
        let any_pressed = self.pressed & button_bits != 0;
        let any_pressed_prev = self.pressed_prev & button_bits != 0;
        any_pressed && !any_pressed_prev
    }

    /// True if none of the buttons are pressed this frame, but at least one was the previous frame.
    #[allow(unused)]
    pub fn just_released(&self, button_bits: u32) -> bool {
        let any_pressed = self.pressed & button_bits != 0;
        let any_pressed_prev = self.pressed_prev & button_bits != 0;
        !any_pressed && any_pressed_prev
    }
}

/// Virtual sticks.
#[derive(Component, Clone, Default, Debug)]
pub struct VSticks {
    sticks: SmallVec<[Vec2; 2]>
}
impl VSticks {
    
    pub fn new(count: usize) -> Self {
        let mut sticks = SmallVec::with_capacity(count);
        for _ in 0..count {
            sticks.push(Vec2::ZERO);
        }
        Self { sticks }
    }

    /// Gets a stick by index
    pub fn get(&self, stick_idx: usize) -> Option<Vec2> {
        self.sticks.get(stick_idx).copied()
    }

    /// Sets a stick at the specified index
    pub fn set(&mut self, stick_idx: usize, stick: Vec2) {
        self.sticks[stick_idx] = stick;
    }

    fn reset(&mut self) {
        for stick in &mut self.sticks {
            *stick = Vec2::ZERO;
        }
    }
}


/// Maps key presses to virtual button presses on an entity.
#[derive(Component, Clone, Default, Debug)]
pub struct KeyboardMapping {
    key_mappings: Vec<(KeyCode, u32)>,
}

impl<I> From<I> for KeyboardMapping
where
    I: IntoIterator<Item = (KeyCode, u32)>,
{
    fn from(iter: I) -> Self {
        Self {
            key_mappings: iter.into_iter().collect(),
        }
    }
}

/// Maps gamepad inputs to various fields on an entity.
#[derive(Component, Clone, Debug)]
pub struct GamepadMapping {
    pub gamepad_entity: Entity,
    button_mappings: SmallVec<[(GamepadButton, u32); 16]>,
    stick_mappings: SmallVec<[(StickType, StickConfig); 2]>,
}

impl GamepadMapping {
    pub fn new(gamepad_entity: Entity) -> Self {
        Self {
            gamepad_entity,
            button_mappings: SmallVec::default(),
            stick_mappings: SmallVec::default(),
        }
    }

    pub fn with_button(mut self, button: GamepadButton, vbutton: u32) -> Self {
        self.button_mappings.push((button, vbutton));
        self
    }

    pub fn with_stick(mut self, stick_type: StickType, stick_cfg: StickConfig) -> Self {
        self.stick_mappings.push((stick_type, stick_cfg));
        self
    }
}

#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct StickConfig {
    pub vstick_idx: usize,
    pub deadzones: Vec2,
}

/// Identifies a stick on a gamepad.
#[derive(Copy, Clone, Eq, PartialEq,Debug)]
#[allow(unused)]
pub enum StickType { Left, Right }

/// Maps keyboard for virtual buttons.
pub fn map_keyboard(
    input: Res<ButtonInput<KeyCode>>,
    mut mappables: Query<(&KeyboardMapping, &mut VButtons)>,
) {
    for (keyboard_mapping, mut vbuttons) in &mut mappables {
        for (key, vbutton) in keyboard_mapping.key_mappings.iter().copied() {
            if input.pressed(key) {
                vbuttons.press(vbutton);
            }
        }
    }
}

pub fn map_gamepads(
    gamepad_q: Query<&Gamepad>,
    mut button_mappings: Query<(&GamepadMapping, &mut VButtons)>,
    mut stick_mappings: Query<(&GamepadMapping, &mut VSticks)>,
) {
    // Maps buttons
    for (gamepad_mapping, mut vbuttons) in &mut button_mappings {
        let Ok(gamepad) = gamepad_q.get(gamepad_mapping.gamepad_entity) else { continue };
        for (button, vbutton) in gamepad_mapping.button_mappings.iter().copied() {
            if gamepad.pressed(button) {
               vbuttons.press(vbutton);
            }
        }
    }

    // Maps sticks
    for (gamepad_mapping, mut vsticks) in &mut stick_mappings {
        let Ok(gamepad) = gamepad_q.get(gamepad_mapping.gamepad_entity) else { continue };
        for (stick_type, stick_cfg) in gamepad_mapping.stick_mappings.iter().copied() {
            let (axis_x, axis_y) = match stick_type {
                StickType::Left => (
                    GamepadAxis::LeftStickX,
                    GamepadAxis::LeftStickY,
                ),
                StickType::Right => (
                    GamepadAxis::RightStickX,
                    GamepadAxis::RightStickY,
                ),
            };
            let Some(mut x) = gamepad.get(axis_x) else { continue };
            let Some(mut y) = gamepad.get(axis_y) else { continue };
            if x.abs() < stick_cfg.deadzones.x { x = 0.0 }
            if y.abs() < stick_cfg.deadzones.y { y = 0.0 }
            let Some(vstick) = vsticks.get(stick_cfg.vstick_idx) else { continue };
            let vstick = vstick + Vec2::new(x, y);
            vsticks.set(stick_cfg.vstick_idx, vstick);
        }
    }
}

/// Syncs previous vbutton state with current.
pub fn reset_virtual_inputs(
    mut vbuttons_q: Query<&mut VButtons>,
    mut vsticks_q: Query<&mut VSticks>,
) {
    for mut vbuttons in &mut vbuttons_q {
        vbuttons.pressed_prev = vbuttons.pressed;
        vbuttons.pressed = 0;
    }
    for mut vsticks in &mut vsticks_q {
        vsticks.reset();
    }
}
