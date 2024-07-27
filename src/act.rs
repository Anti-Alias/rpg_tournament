//! Includes action helper functions for appending actions.

use std::time::Duration;
use extension_trait::extension_trait;
use crate::action::{StartEnv, EndEnv};
use crate::action::common::{End, Print, Quit, Start, Wait};

#[extension_trait]
pub impl<'a> StartEnvExt for StartEnv<'a> {
    
    fn wait_millis(&mut self, millis: u64) {
        self.push(Wait(Duration::from_millis(millis)));
    }
    
    fn print(&mut self, str: &'static str) {
        self.push(Print(str));
    }
    
    fn wait_secs(&mut self, seconds: f32) {
        self.push(Wait(Duration::from_secs_f32(seconds)));
    }

    fn quit(&mut self) {
        self.push(Quit);
    }
    
    fn start<C, R>(&mut self, callback: C)
    where
        C: FnOnce(&mut StartEnv) -> R + Send + Sync + 'static,
        R: Send + Sync + 'static,
    {
        self.push(Start::Do(callback));
    }
    
    fn end<C, R>(&mut self, callback: C)
    where
        C: FnOnce(&mut EndEnv) -> R + Send + Sync + 'static,
        R: Send + Sync + 'static,
    {
        self.push(End::Do(callback));
    }
}