use bevy::prelude::*;
use crate::components::Position;
use crate::enemy::Enemy;
use crate::player::Player;
use crate::resources::{CombatState, CurrentEnemy, GameLog};
use crate::states::GameState;

/// Vérifie si le joueur est sur la même case qu'un ennemi
/// Si oui :
/// - Sauvegarde les données de l'ennemi dans CurrentEnemy
/// - Initialise l'état du combat (CombatState)
/// - Ajoute un message au log
/// - Change l'état du jeu vers Combat
pub fn check_enemy_encounter(
    mut commands: Commands,
    player_query: Query<(&Position, &Player)>,
    enemy_query: Query<(Entity, &Position, &Enemy)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut game_log: ResMut<GameLog>,
) {
    if let Ok((player_pos, player_data)) = player_query.get_single() {
        for (enemy_entity, enemy_pos, enemy_data) in enemy_query.iter() {
            if player_pos == enemy_pos {
                game_log.add_message(format!(
                    "Combat contre {} !",
                    enemy_data.enemy_type.get_name()
                ));

                commands.insert_resource(CurrentEnemy {
                    entity: enemy_entity,
                    position: *enemy_pos,
                    enemy_type: enemy_data.enemy_type,
                    hp: enemy_data.stats.hp,
                    stats: enemy_data.stats,
                });

                commands.insert_resource(CombatState {
                    player_hp: player_data.stats.hp,
                    enemy_hp: enemy_data.stats.hp,
                    combat_log: Vec::new(),
                    is_player_turn: true,
                });

                next_state.set(GameState::Combat);
                break;
            }
        }
    }
}
