use crate::assets::ImageAssets;
use crate::entity::Stats;
use bevy::prelude::{Component, Handle, Image};

/// Types d'ennemis disponibles
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnemyType {
    SmallGoblin,  // Petit Gobelin - Le plus faible
    MediumGoblin, // Gobelin Moyen - Moyen
    LargeGoblin,  // Gros Gobelin - Fort
    Wolf,         // Loup - Boss/Le plus fort
}

impl EnemyType {
    pub fn get_image_handle(&self, image_assets: &ImageAssets) -> Handle<Image> {
        match self {
            EnemyType::SmallGoblin => image_assets.small_goblin.clone(),
            EnemyType::MediumGoblin => image_assets.medium_goblin.clone(),
            EnemyType::LargeGoblin => image_assets.large_goblin.clone(),
            EnemyType::Wolf => image_assets.wolf.clone(),
        }
    }

    pub fn get_name(&self) -> &'static str {
        match self {
            EnemyType::SmallGoblin => "Petit Gobelin",
            EnemyType::MediumGoblin => "Gobelin Moyen",
            EnemyType::LargeGoblin => "Gros Gobelin",
            EnemyType::Wolf => "Loup",
        }
    }
}

#[derive(Component, Debug)]
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub stats: Stats,
}

impl Enemy {
    pub fn new(enemy_type: EnemyType) -> Self {
        let stats = get_stats_for_enemy(enemy_type);
        Enemy { enemy_type, stats }
    }
}

/// Retourne les stats équilibrées pour chaque type d'ennemi
pub fn get_stats_for_enemy(enemy_type: EnemyType) -> Stats {
    match enemy_type {
        // Petit Gobelin : faible, rapide, peu de dégâts
        EnemyType::SmallGoblin => Stats {
            hp: 30,
            attack: 5,
            speed: 8,
            critical_chance: 5,
        },
        // Gobelin Moyen : équilibré
        EnemyType::MediumGoblin => Stats {
            hp: 50,
            attack: 8,
            speed: 5,
            critical_chance: 10,
        },
        // Gros Gobelin : tanky, lent, fort
        EnemyType::LargeGoblin => Stats {
            hp: 80,
            attack: 12,
            speed: 3,
            critical_chance: 8,
        },
        // Loup : Boss - très fort, rapide, dangereux
        EnemyType::Wolf => Stats {
            hp: 100,
            attack: 15,
            speed: 12,
            critical_chance: 20,
        },
    }
}
