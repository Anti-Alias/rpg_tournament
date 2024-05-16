use crate::batch::{AssetBatch, SpawnBatch};
use crate::screen::{FadeIn, FadeOut};
use crate::ui::{set_dialog_message, spawn_dialog, TextAdvancer};
use super::TaskQueue;

use bevy::prelude::*;


/// Wraps a [`TaskQueue`] to extend its functionality.
#[derive(Deref, DerefMut)]
pub struct ExtTaskQueue<'a, 'b>(pub &'a mut TaskQueue<'b>);
impl<'a, 'b> ExtTaskQueue<'a, 'b> {

    /// Spawns 0 or more entities in a single batch.
    /// Spawn commands will wait until all handles have finished loading.
    pub fn spawn_batch<'c, F>(&mut self, callback: F)
    where
        F: FnOnce(Commands, &mut AssetBatch) + Send + Sync + 'static
    {
        self.push(SpawnBatch::new(callback));
    }

    /// Spawns a dialog entity.
    pub fn spawn_dialog(&mut self, message: impl Into<String>, dialog_id: Entity, text_id: Entity) {
        let message = message.into();
        self.spawn_batch(move |mut commands, assets| {
            spawn_dialog(&message, dialog_id, text_id, &mut commands, assets);
        });
    }

    /// Updates a dialog's text.
    pub fn set_dialog_message(&mut self, message: impl Into<String>, text_id: Entity) {
        let message = message.into();
        self.spawn_batch(move |mut commands, assets| {
            set_dialog_message(&message, text_id, &mut commands, assets);
        });
    }

    /// Waits for dialog text to stop advancing.
    pub fn wait_on_text(&mut self, text: Entity) {
        self.run(move |world, _delta| {
            let advancer = world.get::<TextAdvancer>(text).unwrap();
            advancer.finished().into()
        });
    }

    /// Sets a particular state.
    pub fn set_state<S: States>(&mut self, state: S) {
        self.start(move |world, _tq| {
            let mut next_state = world.resource_mut::<NextState<S>>();
            next_state.set(state);
        });
    }

    /// Quits if a particular state if equal to the state provided.
    pub fn quit_if_state<S: States>(&mut self, state: S, despawn_host: bool) {
        self.start(move |world, tq| {
            let current_state = world.resource::<State<S>>();
            if current_state.get() == &state {
                tq.quit(despawn_host);
            };
        });
    }
    
    /// Inserts a bundle to the host.
    pub fn insert_host<B: Bundle>(&mut self, bundle: B) {
        self.start(move |world, tq| {
            let host = tq.host();
            world.entity_mut(host).insert(bundle);
        });
    }

    /// Spawns fade entity and fades to the color specified.
    pub fn fade_in(&mut self, fade_id: Entity, color: Color, duration_secs: f32) {
        self.push(FadeIn::new(fade_id, color, duration_secs))
    }

    /// Fades out the fade entity. Despawns it when done.
    pub fn fade_out(&mut self, fade_id: Entity, duration_secs: f32) {
        self.push(FadeOut::new(fade_id, duration_secs))
    }
}