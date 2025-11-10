use crate::entity::{Position, Stats};

/// Les différentes classes jouables
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Class {
    Warrior,
    Rogue,
}

/// Représente le joueur
#[derive(Debug)]
pub struct Player {
    pub class: Class,
    pub stats: Stats,
    pub position: Position,
}

impl Player {
    /// Crée un nouveau joueur basé sur la classe choisie et une position de départ
    pub fn new(class: Class, start_position: Position) -> Self {
        let stats = match class {
            Class::Warrior => Stats {
                hp: 150,
                attack: 15,
                speed: 5,
                critical_chance: 5,
            },
            Class::Rogue => Stats {
                hp: 100,
                attack: 10,
                speed: 10,
                critical_chance: 10,
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