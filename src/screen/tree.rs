use bevy::prelude::*;
use bevy_ui_dsl::Class;

/// Utility that aids in building a entity hierarchy.
/// Particularly useful for building UIs.
pub struct TreeBuilder<'a, 'b, 'c> {
    pub commands: &'a mut Commands<'b, 'c>,
    ancestors: Vec<Entity>,
    last_entity: Option<Entity>,
}

impl<'a, 'b, 'c> TreeBuilder<'a, 'b, 'c> {

    pub fn new(commands: &'a mut Commands<'b, 'c>) -> Self {
        Self {
            commands,
            ancestors: Vec::new(),
            last_entity: None,
        }
    }

    pub fn spawn(&mut self, bundle: impl Bundle) -> &mut Self {
        let entity_spawned = match self.parent() {
            Some(parent) => self.commands.spawn(bundle).set_parent(parent).id(),
            None => self.commands.spawn(bundle).id()
        };
        self.last_entity = Some(entity_spawned);
        self
    }

    pub fn insert(&mut self, bundle: impl Bundle) -> &mut Self {
        let last_entity = self.last_entity.expect("Cannot 'insert' here");
        self.commands.entity(last_entity).insert(bundle);
        self
    }

    /// Moves focus to last child spawned.
    pub fn begin(&mut self) -> &mut Self {
        let last_entity = self.last_entity.expect("Cannot 'begin' here");
        self.ancestors.push(last_entity);
        self.last_entity = None;
        self
    }

    /// Moves focus back to parent.
    pub fn end(&mut self) -> &mut Self {
        if self.ancestors.is_empty() {
            panic!("Cannot 'end' here");
        }
        let last_idx = self.ancestors.len() - 1;
        self.last_entity = Some(self.ancestors.remove(last_idx));
        self
    }

    fn parent(&self) -> Option<Entity> {
        self.ancestors.last().copied()
    }
}

pub fn begin(t: &mut TreeBuilder) {
    t.begin();
}

pub fn end(t: &mut TreeBuilder) {
    t.end();
}

pub fn node(class: impl Class<NodeBundle>, t: &mut TreeBuilder) {
    let mut bundle = NodeBundle::default();
    class.apply(&mut bundle);
    t.spawn(bundle);
}

pub fn text(
    value: impl Into<String>,
    class: impl Class<TextBundle>,
    t: &mut TreeBuilder
) {
    let mut bundle = TextBundle::default();
    class.apply(&mut bundle);
    t.spawn(bundle);
}


mod test {
    use bevy::prelude::*;
    use super::*;

    fn test() {
        let mut commands: Commands = get_commands();
        let t = &mut TreeBuilder::new(&mut commands);
        node(c_node, t);
        begin(t);
            node(c_node, t);
            node(c_node, t);
            node(c_node, t);
        end(t);
    }

    fn get_commands<'a, 'b>() -> Commands<'a, 'b> {
        todo!()
    }

    // ----------------- Classes -----------------
    fn c_node(b: &mut NodeBundle) {

    }
}


