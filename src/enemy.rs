use crate::assets::ImageAssets;
use crate::components::Stats;
use bevy::prelude::{Component, Handle, Image};

/// Types d'ennemis avec difficulté progressive
/// Distribution sur les maps :
/// - Map 1 : 3 Petits Gobelins + 1 Gobelin Moyen (facile)
/// - Map 2 : 3 Gobelins Moyens + 1 Gros Gobelin + 1 Loup Boss (difficile)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnemyType {
    SmallGoblin,  // Petit Gobelin - Le plus faible, rapide
    MediumGoblin, // Gobelin Moyen - Équilibré
    LargeGoblin,  // Gros Gobelin - Tanky et fort
    Wolf,         // Loup - Boss final, très dangereux
}

impl EnemyType {
    /// Retourne le handle de l'image correspondant au type d'ennemi
    pub fn get_image_handle(&self, image_assets: &ImageAssets) -> Handle<Image> {
        match self {
            EnemyType::SmallGoblin => image_assets.small_goblin.clone(),
            EnemyType::MediumGoblin => image_assets.medium_goblin.clone(),
            EnemyType::LargeGoblin => image_assets.large_goblin.clone(),
            EnemyType::Wolf => image_assets.wolf.clone(),
        }
    }

    /// Retourne le nom français de l'ennemi pour l'affichage
    pub fn get_name(&self) -> &'static str {
        match self {
            EnemyType::SmallGoblin => "Petit Gobelin",
            EnemyType::MediumGoblin => "Gobelin Moyen",
            EnemyType::LargeGoblin => "Gros Gobelin",
            EnemyType::Wolf => "Loup",
        }
    }
}

/// Component Bevy représentant un ennemi sur la map
/// Contient le type d'ennemi et ses statistiques de combat
#[derive(Component, Debug)]
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub stats: Stats,
}

impl Enemy {
    /// Crée un nouvel ennemi avec les stats prédéfinies pour son type
    pub fn new(enemy_type: EnemyType) -> Self {
        let stats = get_stats_for_enemy(enemy_type);
        Enemy { enemy_type, stats }
    }
}

/// Retourne les stats équilibrées et prédéfinies pour chaque type d'ennemi
///
/// Équilibrage des ennemis :
/// - Petit Gobelin: HP 30, ATK 5, SPD 8, CRIT 5% (rapide mais faible)
/// - Gobelin Moyen: HP 50, ATK 8, SPD 5, CRIT 10% (équilibré)
/// - Gros Gobelin: HP 80, ATK 12, SPD 3, CRIT 8% (tanky et fort)
/// - Loup (Boss): HP 100, ATK 15, SPD 12, CRIT 20% (très dangereux)
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
