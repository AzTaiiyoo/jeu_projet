use crate::assets::ImageAssets;
use crate::entity::{Position, Stats};
use bevy::prelude::{Component, Handle, Image};

/// Les différentes classes jouables
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayerClass {
    Warrior,
    Mage,
    Assassin,
    Executioner,
}

impl PlayerClass {
    pub fn get_image_handle(&self, image_assets: &ImageAssets) -> Handle<Image> {
        match self {
            PlayerClass::Warrior => image_assets.warrior_class.clone(),
            PlayerClass::Mage => image_assets.mage_class.clone(),
            PlayerClass::Assassin => image_assets.assassin_class.clone(),
            PlayerClass::Executioner => image_assets.executioner_class.clone(),
        }
    }
}

/// Représente le joueur
#[derive(Component, Debug, Clone)]
pub struct Player {
    pub class: PlayerClass,
    pub stats: Stats,
    pub position: Position,
}

impl Player {
    /// Crée un nouveau joueur basé sur la classe choisie et une position de départ
    pub fn new(class: PlayerClass, start_position: Position) -> Self {
        let stats = match class {
            PlayerClass::Warrior => Stats {
                hp: 120,
                attack: 10,
                speed: 5,
                critical_chance: 10,
            },
            PlayerClass::Mage => Stats {
                hp: 90,
                attack: 15,
                speed: 3,
                critical_chance: 15,
            },
            PlayerClass::Assassin => Stats {
                hp: 100,
                attack: 8,
                speed: 12,
                critical_chance: 15,
            },
            PlayerClass::Executioner => Stats {
                hp: 120,
                attack: 7,
                speed: 2,
                critical_chance: 25,
            },
        };

        Player {
            class,
            stats,
            position: start_position,
        }
    }

    /// Applique les bonus d'un objet aux statistiques du joueur
    pub fn apply_item_stats(&mut self, item_stats: &Stats) {
        self.stats.hp += item_stats.hp;
        self.stats.attack += item_stats.attack;
        self.stats.speed += item_stats.speed;
        self.stats.critical_chance += item_stats.critical_chance;
        println!(
            "Stats mises à jour ! HP: {}, ATK: {}, SPD: {}, CRIT: {}%",
            self.stats.hp, self.stats.attack, self.stats.speed, self.stats.critical_chance
        );
    }
}
