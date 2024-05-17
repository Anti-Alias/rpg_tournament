use std::time::Duration;

use bevy::asset::AssetPath;
use bevy::ecs::system::CommandQueue;
use bevy::prelude::*;

use crate::task::*;


/// Extends the functionality of [`AssetServer`] by keeping track of the handles loaded since creation.
/// This is useful for waiting on a set of handles before continuing on to another task.
#[derive(Clone, Deref)]
pub struct AssetBatch {
    #[deref]
    pub assets: AssetServer,
    handles: Vec<UntypedHandle>,
}

impl AssetBatch {
    pub fn new(assets: AssetServer) -> Self {
        Self { assets, handles: vec![] }
    }

    /// Passthrough for [`load`](AssetServer::load).
    /// Keeps track of handle in separate vec.
    pub fn load<'b, A: Asset>(&mut self, path: impl Into<AssetPath<'b>>) -> Handle<A> {
        let handle = self.assets.load(path);
        self.handles.push(handle.clone().untyped());
        handle
    }

    /// Finishes the batch, returning the handles accumulated.
    pub fn finish(self) -> Vec<UntypedHandle> {
        self.handles
    }
}


/// [`Task`] used to spawn a batch of entities.
/// All spawn commands will remain enqueued until all handles have finished loading.
/// Can be used to spawn a single, or a group of entities.
pub struct SpawnBatch<F> {
    spawn_func: Option<F>,
    spawn_commands: CommandQueue,
    loading_handles: Vec<UntypedHandle>,
}

impl<F> SpawnBatch<F> {
    pub fn new(spawn_func: F) -> Self {
        Self {
            spawn_func: Some(spawn_func),
            spawn_commands: CommandQueue::default(),
            loading_handles: vec![]
        }
    }
}

impl<F> Task for SpawnBatch<F>
where
    F: FnOnce(&mut World, &mut CommandQueue, &mut AssetBatch) + Send + Sync + 'static,
{
    fn start(&mut self, world: &mut World, _tq: &mut TaskQueue) {       
        let assets = world.resource::<AssetServer>();
        let mut assets = AssetBatch::new(assets.clone());
        let spawn_func = self.spawn_func.take().unwrap();
        spawn_func(world, &mut self.spawn_commands, &mut assets);
        self.loading_handles = assets.finish();
    }

    fn run(&mut self, world: &mut World, _delta: Duration) -> TaskStatus {
        let assets = world.resource::<AssetServer>();
        self.loading_handles.retain(|handle| !assets.is_loaded_with_dependencies(handle));
        if self.loading_handles.is_empty() {
            self.spawn_commands.apply(world);
            TaskStatus::Finished
        }
        else {
            TaskStatus::NotFinished
        }
    }
}