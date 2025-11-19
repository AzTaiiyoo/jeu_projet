use bevy::prelude::*;

use crate::player::Player;
use crate::resources::*;
use crate::systems::map::GameData;

/// Système exécuté quand on retourne à ClassSelection (typiquement après la mort)
/// Nettoie complètement l'état de jeu : despawn le joueur + réinitialise les ressources
pub fn reset_game_state(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    mut game_data: ResMut<GameData>,
    mut collected_items: ResMut<CollectedItems>,
    mut defeated_enemies: ResMut<DefeatedEnemies>,
    mut game_log: ResMut<GameLog>,
) {
    // Despawn toutes les entités Player existantes pour éviter les duplications
    for entity in player_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Réinitialiser GameData à la map 0
    game_data.current_map_index = 0;

    // Vider les ressources de progression
    *collected_items = CollectedItems::default();
    *defeated_enemies = DefeatedEnemies::default();
    *game_log = GameLog::default();

    info!("État de jeu réinitialisé : Player despawn, resources reset");
}
