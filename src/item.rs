use crate::assets::ImageAssets;
use crate::components::Stats;
use bevy::prelude::{Component, Handle, Image};

/// Types d'objets collectables avec leurs bonus spécifiques
/// Chaque objet améliore une statistique particulière
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ItemType {
    Armure,    // +50 HP (survie)
    Katana,    // +5 SPD (esquive)
    Gants,     // +10 ATK (dégâts)
    Pendentif, // +10% CRIT (coups critiques)
}

impl ItemType {
    /// Retourne le handle de l'image correspondant au type d'objet
    pub fn get_image_handle(&self, image_assets: &ImageAssets) -> Handle<Image> {
        match self {
            ItemType::Armure => image_assets.armor_item.clone(),
            ItemType::Katana => image_assets.katana_item.clone(),
            ItemType::Gants => image_assets.gloves_item.clone(),
            ItemType::Pendentif => image_assets.pendant_item.clone(),
        }
    }
}

/// Component Bevy représentant un objet collectable sur la map
#[derive(Component, Debug)]
pub struct Item {
    pub item_type: ItemType,
}

/// Retourne les bonus de stats pour chaque type d'objet
/// Les bonus sont appliqués de manière permanente au joueur lors de la collecte
///
/// Distribution sur les maps :
/// - Map 1 : Katana (+5 SPD), Armure (+50 HP)
/// - Map 2 : Gants (+10 ATK), Pendentif (+10% CRIT)
pub fn get_stats_for_item(item_type: ItemType) -> Stats {
    match item_type {
        ItemType::Armure => Stats {
            hp: 50,
            attack: 0,
            speed: 0,
            critical_chance: 0,
        },
        ItemType::Katana => Stats {
            hp: 0,
            attack: 0,
            speed: 10,
            critical_chance: 0,
        },
        ItemType::Gants => Stats {
            hp: 0,
            attack: 20,
            speed: 0,
            critical_chance: 0,
        },
        ItemType::Pendentif => Stats {
            hp: 0,
            attack: 0,
            speed: 0,
            critical_chance: 15,
        },
    }
}
