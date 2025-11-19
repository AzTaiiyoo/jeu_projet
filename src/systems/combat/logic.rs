use crate::components::{ActionMessageText, CombatLogText, EnemyStatsText, PlayerStatsText};
use crate::player::Player;
use crate::resources::{CombatState, CurrentEnemy, DefeatedEnemies, GameLog};
use crate::states::GameState;
use crate::systems::combat::calculations::{calculate_damage, check_dodge};
use crate::systems::map::GameData;
use bevy::prelude::*;

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
    mut action_message_query: Query<&mut Text, (With<ActionMessageText>, Without<CombatLogText>)>,
    mut combat_log_query: Query<&mut Text, (With<CombatLogText>, Without<ActionMessageText>)>,
    mut player_stats_query: Query<
        &mut Text,
        (
            With<PlayerStatsText>,
            Without<EnemyStatsText>,
            Without<ActionMessageText>,
            Without<CombatLogText>,
        ),
    >,
    mut enemy_stats_query: Query<
        &mut Text,
        (
            With<EnemyStatsText>,
            Without<PlayerStatsText>,
            Without<ActionMessageText>,
            Without<CombatLogText>,
        ),
    >,
    mut defeated_enemies: ResMut<DefeatedEnemies>,
    game_data: Res<GameData>,
) {
    let mut player = player_query.single_mut();

    if keyboard_input.just_pressed(KeyCode::Space) && combat_state.is_player_turn {
        let mut rng = rand::thread_rng();
        let mut action_msg = String::new();
        let mut log_lines = Vec::new();

        action_msg.push_str("--- VOTRE TOUR ---\n\n");

        // Tour du joueur
        if check_dodge(current_enemy.stats.speed, &mut rng) {
            action_msg.push_str("L'ennemi esquive votre attaque !");
            log_lines.push("Esquive ennemi".to_string());
        } else {
            let result = calculate_damage(&player.stats, &mut rng);
            if result.is_critical {
                action_msg.push_str(&format!("⚡ COUP CRITIQUE !\nDegats: {}", result.damage));
                log_lines.push(format!("CRIT {} dmg", result.damage));
            } else {
                action_msg.push_str(&format!("Vous attaquez !\nDegats: {}", result.damage));
                log_lines.push(format!("Atk {} dmg", result.damage));
            }
            combat_state.enemy_hp -= result.damage;
        }

        // Vérifier victoire
        if combat_state.enemy_hp <= 0 {
            action_msg = "🎉 VICTOIRE ! 🎉\n\nChoisissez une amelioration:\nH = +10 HP | A = +2 ATK\nS = +1 SPD | C = +2% CRIT".to_string();
            log_lines.push("VICTOIRE !".to_string());
            combat_state.is_player_turn = false;
        } else {
            // Tour de l'ennemi
            action_msg.push_str(&format!(
                "\n\nHP ennemi: {}\n\n--- TOUR ENNEMI ---\n\n",
                combat_state.enemy_hp.max(0)
            ));

            if check_dodge(player.stats.speed, &mut rng) {
                action_msg.push_str("Vous esquivez l'attaque !");
                log_lines.push("Esquive joueur".to_string());
            } else {
                let result = calculate_damage(&current_enemy.stats, &mut rng);
                if result.is_critical {
                    action_msg
                        .push_str(&format!("⚡ CRITIQUE ENNEMI !\nDegats: {}", result.damage));
                    log_lines.push(format!("Ennemi CRIT {} dmg", result.damage));
                } else {
                    action_msg.push_str(&format!("L'ennemi attaque !\nDegats: {}", result.damage));
                    log_lines.push(format!("Ennemi {} dmg", result.damage));
                }
                combat_state.player_hp -= result.damage;
            }

            // Vérifier défaite
            if combat_state.player_hp <= 0 {
                action_msg = "💀 DEFAITE... 💀\n\nAppuyez sur R pour recommencer".to_string();
                log_lines.push("DEFAITE".to_string());
                combat_state.is_player_turn = false;
            } else {
                action_msg.push_str(&format!(
                    "\n\nVos HP: {}\n\n[ESPACE] pour continuer",
                    combat_state.player_hp.max(0)
                ));
            }
        }

        // Mettre à jour le message d'action au centre
        if let Ok(mut text) = action_message_query.get_single_mut() {
            text.sections[0].value = action_msg;
        }

        // Mettre à jour le log d'historique (garder les 5 dernières lignes)
        if let Ok(mut text) = combat_log_query.get_single_mut() {
            combat_state.combat_log.extend(log_lines);
            let recent_logs: Vec<String> = combat_state
                .combat_log
                .iter()
                .rev()
                .take(5)
                .rev()
                .cloned()
                .collect();
            text.sections[0].value = recent_logs.join("\n");
        }

        // Mettre à jour les textes de stats
        if let Ok(mut text) = player_stats_query.get_single_mut() {
            text.sections[0].value = format!(
                "Points de vie: {} / {}\nAttaque: {}\nVitesse: {}\nCritique: {}%",
                combat_state.player_hp.max(0),
                player.stats.hp,
                player.stats.attack,
                player.stats.speed,
                player.stats.critical_chance
            );
        }

        if let Ok(mut text) = enemy_stats_query.get_single_mut() {
            text.sections[0].value = format!(
                "Points de vie: {} / {}\nAttaque: {}\nVitesse: {}\nCritique: {}%",
                combat_state.enemy_hp.max(0),
                current_enemy.stats.hp,
                current_enemy.stats.attack,
                current_enemy.stats.speed,
                current_enemy.stats.critical_chance
            );
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
