use crate::assets::ImageAssets;
use crate::entity::{Position, Stats};
use bevy::prelude::{Component, Handle, Image};

/// Les différentes classes jouables avec des profils de stats équilibrés
/// Chaque classe a des forces et faiblesses uniques pour varier le gameplay
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayerClass {
    Warrior,     // Tanky avec HP élevés
    Mage,        // Forte attaque mais fragile
    Assassin,    // Très rapide avec esquive élevée
    Executioner, // Spécialisé dans les coups critiques
}

impl PlayerClass {
    /// Retourne le handle de l'image correspondant à la classe
    /// Utilisé pour afficher le sprite du joueur
    pub fn get_image_handle(&self, image_assets: &ImageAssets) -> Handle<Image> {
        match self {
            PlayerClass::Warrior => image_assets.warrior_class.clone(),
            PlayerClass::Mage => image_assets.mage_class.clone(),
            PlayerClass::Assassin => image_assets.assassin_class.clone(),
            PlayerClass::Executioner => image_assets.executioner_class.clone(),
        }
    }
}

/// Component Bevy représentant le joueur
/// Contient la classe choisie, les stats actuelles et la position logique
#[derive(Component, Debug, Clone)]
pub struct Player {
    pub class: PlayerClass,
    pub stats: Stats,
    pub position: Position,
}

impl Player {
    /// Crée un nouveau joueur basé sur la classe choisie
    ///
    /// Stats de base par classe :
    /// - Guerrier: HP 120, ATK 10, SPD 5, CRIT 10%
    /// - Magicien: HP 90, ATK 15, SPD 3, CRIT 15%
    /// - Assassin: HP 100, ATK 8, SPD 12, CRIT 15%
    /// - Bourreau: HP 120, ATK 7, SPD 2, CRIT 25%
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

    /// Applique les bonus d'un objet collecté aux statistiques du joueur
    /// Les bonus sont additifs et permanents
    /// Affiche les nouvelles stats dans la console pour débogage
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
