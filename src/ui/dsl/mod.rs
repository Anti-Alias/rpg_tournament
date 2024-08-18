//! UI dsl that aids in building UI entity hierarchies with fewer lines of code.

mod widgets;
pub use widgets::*;

use bevy::prelude::*;
use smallvec::{SmallVec, smallvec};


/// Domain specific language that aids in building entity hierarchies.
/// Similar to [`ChildBuilder`](bevy::prelude::ChildBuilder), but does not use callback functions and is more UI oriented.
pub struct Dsl<'a, 'w, 's> {
    pub commands: &'a mut Commands<'w, 's>,
    last: Option<Entity>,
    stack: SmallVec<[Entity; 16]>,
}

impl<'a, 'w, 's> Dsl<'a, 'w, 's> {

    pub fn new(commands: &'a mut Commands<'w, 's>, parent: Entity) -> Self {
        Self {
            commands,
            last: None,
            stack: smallvec![parent],
        }
    }

    pub fn root(commands: &'a mut Commands<'w, 's>) -> Self {
        Self {
            commands,
            last: None,
            stack: SmallVec::new(),
        }
    }

    /// Spawns a bundle as a child of [`parent`](Self::parent).
    /// If [`None`], spawns as a root.
    pub fn spawn(&mut self, bundle: impl Bundle) {
        let id = self.commands.spawn(bundle).id();
        self.last = Some(id);
        if let Some(parent) = self.stack.last().copied() {
            self.commands.entity(parent).add_child(id);
        }
    }

    /// Spawns a bundle as a child of [`parent`](Self::parent).
    /// If [`None`], spawns as a root.
    /// Subsequent invocations of [`spawn`](Self::spawn) and [`begin`](Self::begin)
    /// will be children of this entity.
    pub fn begin(&mut self, bundle: impl Bundle) {
        let id = self.commands.spawn(bundle).id();
        self.last = Some(id);
        if let Some(parent) = self.stack.last().copied() {
            self.commands.entity(parent).add_child(id);
        }
        self.stack.push(id);
    }

    /// Subsequent invocations of [`spawn`](Self::spawn) and [`begin`](Self::begin)
    /// will be children of one parent up the insertion hierarchie.
    /// Counterpart to [`begin`](Self::begin).
    pub fn end(&mut self) {
        let old_parent = self.stack.pop();
        assert!(old_parent.is_some(), "end() called too many times");
    }

    /// The last entity inserted.
    pub fn last_opt(&self) -> Option<Entity> {
        self.last
    }

    /// The last entity inserted.
    pub fn last(&self) -> Entity {
        self.last.unwrap()
    }
}
