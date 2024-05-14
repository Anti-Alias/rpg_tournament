mod widgets;
mod dialog;

use bevy::ecs::system::Command;
pub use widgets::*;
pub use dialog::*;
use bevy::prelude::*;
use std::sync::Arc;

pub fn ui_plugin(app: &mut App) {
    app.add_systems(Update, (handle_ui_interactions, advance_text));
}

fn handle_ui_interactions(
    mut commands: Commands,
    press_interactions: Query<(&Interaction, &OnPress), Changed<Interaction>>,
) {
    for (interaction, on_press) in &press_interactions {
        if interaction == &Interaction::Pressed {
            commands.add(on_press.0.clone())
        }
    }
}

/// Dynamic callback.
#[derive(Clone)]
pub struct DynCallback(Arc<dyn Callback>);

impl<C: Callback> From<C> for DynCallback {
    fn from(value: C) -> Self {
        Self(Arc::new(value))
    }
}

impl Command for DynCallback {
    fn apply(self, world: &mut World) {
        self.0.invoke(world);
    }
}

/// A simple callback function responding to something that happened to an entity.
pub trait Callback: Send + Sync + 'static {
    fn invoke(&self, world: &mut World);
}

impl<F, R> Callback for F
where
    F: Fn(&mut World) -> R + Send + Sync + 'static,
{
    fn invoke(&self, world: &mut World) {
        self(world);
    }
}

/// Component that gets fires when an entity gets pressed.
#[derive(Component)]
pub struct OnPress(DynCallback);
impl OnPress {
    pub fn call(callback: impl Into<DynCallback>) -> Self {
        Self(callback.into())
    }
}