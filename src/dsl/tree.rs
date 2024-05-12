use bevy::prelude::*;
use bevy::utils::smallvec::{smallvec, SmallVec};

/// Utility that aids in building an entity hierarchy.
pub struct TreeBuilder<'a, 'b, 'c> {
    pub commands: &'a mut Commands<'b, 'c>,
    ancestors: SmallVec<[Entity; 16]>,
    last_entity: Option<Entity>,
    next_entity: Option<Entity>,
}

impl<'a, 'b, 'c> TreeBuilder<'a, 'b, 'c> {

    pub fn new(parent: Entity, commands: &'a mut Commands<'b, 'c>) -> Self {
        Self {
            commands,
            ancestors: smallvec![parent],
            last_entity: None,
            next_entity: None,
        }
    }

    pub fn root(commands: &'a mut Commands<'b, 'c>) -> Self {
        Self {
            commands,
            ancestors: SmallVec::new(),
            last_entity: None,
            next_entity: None,
        }
    }

    /// Spawns an entity.
    pub fn spawn(&mut self, bundle: impl Bundle) {
        let entity_spawned = match (self.next_entity, self.parent()) {
            (None, None)                => self.commands.spawn(bundle).id(),
            (None, Some(parent))        => self.commands.spawn(bundle).set_parent(parent).id(),
            (Some(next), None)          => self.commands.entity(next).insert(bundle).remove_parent().id(),
            (Some(next), Some(parent))  => self.commands.entity(next).insert(bundle).set_parent(parent).id(),
        };
        self.next_entity = None;
        self.last_entity = Some(entity_spawned);
    }

    /// Inserts a bundle into the last entity spawned.
    pub fn insert(&mut self, bundle: impl Bundle) {
        let last_entity = self.last_entity.expect("Cannot 'insert' here");
        self.commands.entity(last_entity).insert(bundle);
    }

    /// The last entity spawned.
    /// Cleared on begin().
    pub fn last(&self) -> Entity {
        self.last_entity.expect("Last entity not found")
    }

    /// Subsequent invocations of "spawn" will spawn entities
    /// as children of the last entity spawned.
    pub fn begin(&mut self) {
        if self.next_entity.is_some() {
            panic!("Cannot 'begin' if next entity set");
        }
        let last_entity = self.last_entity.expect("Cannot 'begin' here");
        self.ancestors.push(last_entity);
        self.last_entity = None;
    }

    /// Moves focus back to parent.
    pub fn end(&mut self) {
        if self.next_entity.is_some() {
            panic!("Cannot 'end' if next entity set");
        }
        self.last_entity = Some(self.ancestors.pop().expect("Cannot 'end' here"));
    }

    /// Sets the next entity to spawn into.
    /// Clears after spawning.
    pub fn next(&mut self, next_entity: Entity) {
        if self.next_entity.is_some() {
            panic!("Cannot call 'next' if next entity is already set");
        }
        self.next_entity = Some(next_entity);
    }

    fn parent(&self) -> Option<Entity> {
        self.ancestors.last().copied()
    }
}

pub fn next(next_entity: Entity, t: &mut TreeBuilder) {
    t.next(next_entity);
}

pub fn insert(bundle: impl Bundle, t: &mut TreeBuilder) {
    t.insert(bundle);
}

pub fn last(t: &TreeBuilder) -> Entity {
    t.last()
}

/// Helper function.
pub fn begin(t: &mut TreeBuilder) {
    t.begin();
}

/// Helper function.
pub fn end(t: &mut TreeBuilder) {
    t.end();
}