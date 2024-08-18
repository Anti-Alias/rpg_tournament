use derive_more::*;
use crate::equipment::{Equippable, Outfit};


#[derive(From, Clone, PartialEq, Debug)]
pub enum Item {
    Equippable(Equippable),
}


impl From<Outfit> for Item {
    fn from(outfit: Outfit) -> Self {
        Self::Equippable(Equippable::from(outfit))
    }
}