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
            Item::Consumable(Consumable::HealthPotion1)             => ItemInfo { image: "consumables/health_potion_1.png" },
            Item::Equippable(Equippable::Outfit(Outfit::Casual1))   => ItemInfo { image: "player/outfit/casual_1.png" },
            Item::Equippable(Equippable::Outfit(Outfit::Casual2))   => ItemInfo { image: "player/outfit/casual_2.png" },
            Item::Equippable(Equippable::Outfit(Outfit::Casual3))   => ItemInfo { image: "player/outfit/casual_3.png" },
            Item::Equippable(Equippable::Outfit(Outfit::Casual4))   => ItemInfo { image: "player/outfit/casual_4.png" },
            Item::Equippable(Equippable::Outfit(Outfit::Casual5))   => ItemInfo { image: "player/outfit/casual_5.png" },
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