use derive_more::*;

use crate::equipment::{Equippable, Outfit};


#[derive(From, Clone, PartialEq, Debug)]
pub enum Item {
    Consumable(Consumable),
    Equippable(Equippable),
}

impl Item {
    pub fn info(&self) -> ItemInfo {
        match self {
            Item::Equippable(equippable)                => ItemInfo { image: equippable.info().image },
            Item::Consumable(Consumable::HealthPotion1) => ItemInfo { image: "consumables/health_potion_1.png" },
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

#[derive(Clone, PartialEq, Debug)]
pub enum Consumable {
    HealthPotion1,
}