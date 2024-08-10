use bevy::prelude::*;

#[derive(States, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub enum DebugStates {
    Enabled,
    #[default]
    Disabled,
}

pub fn toggle_debug(
    keyboard: Res<ButtonInput<KeyCode>>,
    state: Res<State<DebugStates>>,
    mut next_state: ResMut<NextState<DebugStates>>,
) {
    if keyboard.pressed(KeyCode::ControlLeft) && keyboard.just_pressed(KeyCode::KeyD) {
        match state.get() {
            DebugStates::Enabled => next_state.set(DebugStates::Disabled),
            DebugStates::Disabled => next_state.set(DebugStates::Enabled),
        };
    }
}