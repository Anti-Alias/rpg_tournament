use derive_more::*;
use crate::equipment::{Equippable, Outfit};


#[derive(From, Clone, PartialEq, Debug)]
pub enum Item {
    Equippable(Equippable),
}

impl Item {
    pub fn info(&self) -> ItemInfo {
        match self {
            Item::Equippable(equippable) => ItemInfo { image: equippable.info().image },
        }
    }
}

impl From<Outfit> for Item {
    fn from(outfit: Outfit) -> Self {
        Self::Equippable(Equippable::from(outfit))
    }
}

/// Common info among items.
#[derive(Clone, PartialEq, Debug)]
pub struct ItemInfo {
    pub image: &'static str,
}
