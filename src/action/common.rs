use std::time::Duration;
use bevy::prelude::*;
use crate::act::StartEnvExt;
use crate::{Action, EndEnv, RunEnv, RunStatus, StartEnv};

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Wait(pub Duration);
impl Action for Wait {
    fn run(&mut self, env: &mut RunEnv) -> RunStatus {
        if self.0 > env.delta {
            self.0 -= env.delta;
            RunStatus::Running
        }
        else {
            let consumed = self.0;
            self.0 = Duration::ZERO;
            RunStatus::Finished { time_consumed: consumed }
        }
    }
}

/// An inline action that only implements start().
#[derive(Default)]
pub enum Start<C, R>
where
    C: FnOnce(&mut StartEnv) -> R + Send + Sync + 'static,
    R: Send + Sync + 'static,
{
    Do(C),
    #[default]
    Nothing,
}

impl<C, R> Start<C, R>
where
    C: FnOnce(&mut StartEnv) -> R + Send + Sync + 'static,
    R: Send + Sync + 'static,
{
    #[inline(always)]
    pub fn new(callback: C) -> Self {
        Self::Do(callback)
    }
}

impl<C, R> Action for Start<C, R>
where
    C: FnOnce(&mut StartEnv) -> R + Send + Sync + 'static,
    R: Send + Sync + 'static,
{
    fn start(&mut self, env: &mut StartEnv) {
        let Self::Do(callback) = std::mem::take(self) else { return };
        callback(env);
    }
}

/// An inline action that only implements end().
#[derive(Default)]
pub enum End<C, R>
where
    C: FnOnce(&mut EndEnv) -> R + Send + Sync + 'static,
    R: Send + Sync + 'static,
{
    Do(C),
    #[default]
    Nothing,
}

impl<C, R> End<C, R>
where
    C: FnOnce(&mut EndEnv) -> R + Send + Sync + 'static,
    R: Send + Sync + 'static,
{
    #[inline(always)]
    pub fn new(callback: C) -> Self {
        Self::Do(callback)
    }
}

impl<C, R> Action for End<C, R>
where
    C: FnOnce(&mut EndEnv) -> R + Send + Sync + 'static,
    R: Send + Sync + 'static,
{
    fn end(&mut self, env: &mut EndEnv) {
        let Self::Do(callback) = std::mem::take(self) else { return };
        callback(env);
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Print(pub &'static str);
impl Action for Print {
    fn start(&mut self, _env: &mut StartEnv) {
        println!("{}", self.0);
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Quit;
impl Action for Quit {
    fn run(&mut self, _env: &mut RunEnv) -> RunStatus {
        RunStatus::Quit
    }
}

#[derive(Resource, Default)]
struct CutsceneData {
    times_run: u32,
}

pub fn cutscene(env: &mut StartEnv) {
    env.start(|env| env.world.init_resource::<CutsceneData>());
    env.push(cutscene_loop);
    env.end(|env| env.world.remove_resource::<CutsceneData>());
}

fn cutscene_loop(env: &mut StartEnv) {
    env.print("First message");
    env.wait_secs(1.0);
    env.print("Second message");
    env.wait_secs(1.0);
    env.print("Third message");
    env.wait_secs(1.0);
    env.push(loop_if_not_done);
}

fn loop_if_not_done(env: &mut StartEnv) {
    let mut data = env.world.resource_mut::<CutsceneData>();
    data.times_run += 1;
    if data.times_run < 3 {
        env.push(cutscene_loop);
    }
    else {
        env.print("Done!");
    }
}