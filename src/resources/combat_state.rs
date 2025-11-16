use crate::components::{Position, Stats};
use crate::enemy::EnemyType;
use bevy::prelude::{Entity, Resource};

/// Resource pour stocker l'ennemi actuellement en combat
/// Conserve l'entité Bevy, la position, le type et les stats de l'ennemi
/// Utilisée pour accéder aux données de l'ennemi pendant le combat
#[derive(Resource)]
pub struct CurrentEnemy {
    #[allow(dead_code)]
    pub entity: Entity,
    pub position: Position,
    pub enemy_type: EnemyType,
    #[allow(dead_code)]
    pub hp: i32,
    pub stats: Stats,
}

/// Resource pour gérer l'état du combat tour par tour
/// Contient les HP actuels des combattants, le log de combat et le tour actuel
#[derive(Resource, Default)]
pub struct CombatState {
    pub player_hp: i32,
    pub enemy_hp: i32,
    #[allow(dead_code)]
    pub combat_log: Vec<String>,
    pub is_player_turn: bool,
}
