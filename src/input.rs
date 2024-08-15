use bevy::prelude::*;
use smallvec::SmallVec;


/// Abstraction on virtual buttons on a controller, or the keys on a keyboard.
#[derive(Component, Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct VButtons {
    pressed: u32,
    pressed_prev: u32,
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

    /// True if at least one of the buttons specified was just pressed this frame.
    pub fn just_pressed(&self, button_bits: u32) -> bool {
        let pressed = self.pressed & button_bits != 0;
        let prev_pressed = self.pressed_prev & button_bits != 0;
        pressed && !prev_pressed
    }

    /// True if at least one of the buttons specified was just released this frame.
    pub fn just_released(&self, button_bits: u32) -> bool {
        let pressed = self.pressed & button_bits != 0;
        let prev_pressed = self.pressed_prev & button_bits != 0;
        !pressed && prev_pressed
    }
}

/// Virtual sticks.
#[derive(Component, Clone, Default, Debug)]
pub struct VSticks(SmallVec<[Vec2; 2]>);
impl VSticks {
    
    pub fn new(count: usize) -> Self {
        let mut sticks = SmallVec::with_capacity(count);
        for _ in 0..count {
            sticks.push(Vec2::ZERO);
        }
        Self(sticks)
    }

    /// Gets a stick by index
    pub fn get(&self, stick_idx: usize) -> Option<Vec2> {
        self.0.get(stick_idx).copied()
    }

    /// Sets a stick at the specified index
    pub fn set(&mut self, stick_idx: usize, stick: Vec2) {
        self.0[stick_idx] = stick;
    }
}


/// Maps key presses to virtual button presses on an entity.
#[derive(Component, Clone, Default, Debug)]
pub struct KeyboardMapping {
    key_mappings: SmallVec<[(KeyCode, u32); 8]>,
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
    pub gamepad: Gamepad,
    button_mappings: SmallVec<[(GamepadButtonType, u32); 16]>,
    stick_mappings: SmallVec<[(StickType, usize); 2]>,
}

impl GamepadMapping {
    pub fn new(gamepad: Gamepad) -> Self {
        Self {
            gamepad,
            button_mappings: SmallVec::default(),
            stick_mappings: SmallVec::default(),
        }
    }

    pub fn with_button(mut self, button: GamepadButtonType, vbutton: u32) -> Self {
        self.button_mappings.push((button, vbutton));
        self
    }

    pub fn with_stick(mut self, stick_type: StickType, vstick: usize) -> Self {
        self.stick_mappings.push((stick_type, vstick));
        self
    }
}

/// Identifies a stick on a gamepad.
#[derive(Copy, Clone, Eq, PartialEq,Debug)]
#[allow(unused)]
pub enum StickType {
    Left,
    Right
}

/// Maps keyboard for virtual buttons.
pub fn map_keyboard_to_vbuttons(
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

/// Maps gamepads to virtual buttons.
pub fn map_gamepads_to_vbuttons(
    input: Res<ButtonInput<GamepadButton>>,
    mut mappables: Query<(&GamepadMapping, &mut VButtons)>,
) {
    for (gamepad_mapping, mut vbuttons) in &mut mappables {
        for (button_type, vbutton) in gamepad_mapping.button_mappings.iter().copied() {
            let button = GamepadButton::new(gamepad_mapping.gamepad, button_type);
            if input.pressed(button) {
                vbuttons.press(vbutton);
            }
        }
    }
}

/// Maps gamepads to virtual stick.
pub fn map_gamepads_to_vsticks(
    axes: Res<Axis<GamepadAxis>>,
    mut mappables: Query<(&GamepadMapping, &mut VSticks)>,
) {
    for (gamepad_mapping, mut vsticks) in &mut mappables {
        for (stick_type, stick_idx) in gamepad_mapping.stick_mappings.iter().copied() {
            let gamepad = gamepad_mapping.gamepad;
            let (axis_x, axis_y) = match stick_type {
                StickType::Left => (
                    GamepadAxis { gamepad, axis_type: GamepadAxisType::LeftStickX },
                    GamepadAxis { gamepad, axis_type: GamepadAxisType::LeftStickY },
                ),
                StickType::Right => (
                    GamepadAxis { gamepad, axis_type: GamepadAxisType::RightStickX },
                    GamepadAxis { gamepad, axis_type: GamepadAxisType::RightStickY },
                ),
            };
            let Some(x) = axes.get(axis_x) else { continue };
            let Some(y) = axes.get(axis_y) else { continue };
            vsticks.set(stick_idx, Vec2::new(x, y));
        }
    }
}


/// Syncs previous vbutton state with current.
pub fn sync_vbuttons(mut vbuttons_q: Query<&mut VButtons>) {
    for mut vbuttons in &mut vbuttons_q {
        vbuttons.pressed_prev = vbuttons.pressed;
        vbuttons.pressed = 0;
    }
}