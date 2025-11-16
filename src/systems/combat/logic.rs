use bevy::prelude::*;
use crate::components::CombatLogText;
use crate::player::Player;
use crate::resources::{CombatState, CurrentEnemy, DefeatedEnemies, GameLog};
use crate::states::GameState;
use crate::systems::combat::calculations::{calculate_damage, check_dodge};
use crate::systems::map::GameData;

/// Gère la logique du combat tour par tour
///
/// Mécanique de combat :
/// 1. Tour du joueur (ESPACE) :
///    - Calcul d'esquive de l'ennemi (basé sur sa vitesse)
///    - Calcul de critique du joueur (basé sur son taux de critique)
///    - Application des dégâts (×2 si critique)
///
/// 2. Tour de l'ennemi (automatique) :
///    - Même logique mais inversée
///
/// 3. Fin de combat :
///    - Victoire : Choix d'amélioration de stat (H/A/S/C)
///    - Défaite : Game Over avec option de recommencer (R)
pub fn handle_combat(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Player>,
    current_enemy: Res<CurrentEnemy>,
    mut combat_state: ResMut<CombatState>,
    mut next_state: ResMut<NextState<GameState>>,
    mut game_log: ResMut<GameLog>,
    mut combat_log_text_query: Query<&mut Text, With<CombatLogText>>,
    mut defeated_enemies: ResMut<DefeatedEnemies>,
    game_data: Res<GameData>,
) {
    let mut player = player_query.single_mut();

    if keyboard_input.just_pressed(KeyCode::Space) && combat_state.is_player_turn {
        let mut rng = rand::thread_rng();
        let mut log_messages = Vec::new();
        log_messages.push("--- Votre tour ---".to_string());

        // Tour du joueur
        if check_dodge(current_enemy.stats.speed, &mut rng) {
            log_messages.push("L'ennemi esquive !".to_string());
        } else {
            let result = calculate_damage(&player.stats, &mut rng);
            if result.is_critical {
                log_messages.push(format!("COUP CRITIQUE ! Degats: {}", result.damage));
            } else {
                log_messages.push(format!("Vous attaquez ! Degats: {}", result.damage));
            }
            combat_state.enemy_hp -= result.damage;
        }

        log_messages.push(format!("HP ennemi: {}", combat_state.enemy_hp.max(0)));

        // Vérifier victoire
        if combat_state.enemy_hp <= 0 {
            log_messages.push("".to_string());
            log_messages.push("VICTOIRE !".to_string());
            log_messages.push("Choisissez une stat a ameliorer:".to_string());
            log_messages.push("H=HP | A=ATK | S=SPD | C=CRIT".to_string());
            combat_state.is_player_turn = false;
        } else {
            // Tour de l'ennemi
            log_messages.push("".to_string());
            log_messages.push("--- Tour ennemi ---".to_string());

            if check_dodge(player.stats.speed, &mut rng) {
                log_messages.push("Vous esquivez !".to_string());
            } else {
                let result = calculate_damage(&current_enemy.stats, &mut rng);
                if result.is_critical {
                    log_messages.push(format!("COUP CRITIQUE ENNEMI ! Degats: {}", result.damage));
                } else {
                    log_messages.push(format!("Ennemi attaque ! Degats: {}", result.damage));
                }
                combat_state.player_hp -= result.damage;
            }

            log_messages.push(format!("Vos HP: {}", combat_state.player_hp.max(0)));

            // Vérifier défaite
            if combat_state.player_hp <= 0 {
                log_messages.push("".to_string());
                log_messages.push("DEFAITE...".to_string());
                log_messages.push("Appuyez sur R pour recommencer".to_string());
                combat_state.is_player_turn = false;
            } else {
                log_messages.push("".to_string());
                log_messages.push("ESPACE pour continuer".to_string());
            }
        }

        // Mettre à jour le texte du log de combat
        if let Ok(mut text) = combat_log_text_query.get_single_mut() {
            text.sections[0].value = log_messages.join("\n");
        }
    }

    // Gestion de la victoire - choix de stat
    if combat_state.enemy_hp <= 0 && !combat_state.is_player_turn {
        let mut stat_chosen = false;

        if keyboard_input.just_pressed(KeyCode::H) {
            player.stats.hp += 10;
            game_log.add_message("HP +10 !".to_string());
            stat_chosen = true;
        } else if keyboard_input.just_pressed(KeyCode::A) {
            player.stats.attack += 2;
            game_log.add_message("ATK +2 !".to_string());
            stat_chosen = true;
        } else if keyboard_input.just_pressed(KeyCode::S) {
            player.stats.speed += 1;
            game_log.add_message("SPD +1 !".to_string());
            stat_chosen = true;
        } else if keyboard_input.just_pressed(KeyCode::C) {
            player.stats.critical_chance += 2;
            game_log.add_message("CRIT +2% !".to_string());
            stat_chosen = true;
        }

        if stat_chosen {
            defeated_enemies
                .enemies
                .insert((game_data.current_map_index, current_enemy.position));

            game_log.add_message(format!("{} vaincu !", current_enemy.enemy_type.get_name()));
            next_state.set(GameState::Map);
        }
    }

    // Gestion de la défaite
    if combat_state.player_hp <= 0 && keyboard_input.just_pressed(KeyCode::R) {
        next_state.set(GameState::ClassSelection);
    }
}
