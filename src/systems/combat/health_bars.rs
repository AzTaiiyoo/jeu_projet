use bevy::prelude::*;

use crate::components::HealthBar;
use crate::player::Player;
use crate::resources::{CombatState, CurrentEnemy};

/// Système qui met à jour dynamiquement la largeur des barres de vie
/// en fonction des HP actuels du joueur et de l'ennemi
/// Appelé en boucle pendant le combat (Update schedule)
pub fn update_health_bars(
    mut health_bar_query: Query<(&HealthBar, &mut Style)>,
    combat_state: Res<CombatState>,
    player_query: Query<&Player>,
    current_enemy: Res<CurrentEnemy>,
) {
    // Récupérer les stats max
    let player = match player_query.get_single() {
        Ok(p) => p,
        Err(_) => return,
    };

    let player_max_hp = player.stats.hp as f32;
    let enemy_max_hp = current_enemy.stats.hp as f32;

    // Mettre à jour chaque barre
    for (bar_type, mut style) in health_bar_query.iter_mut() {
        match bar_type {
            HealthBar::Player => {
                let hp_percent = (combat_state.player_hp as f32 / player_max_hp * 100.0)
                    .max(0.0)
                    .min(100.0);
                style.width = Val::Percent(hp_percent);
            }
            HealthBar::Enemy => {
                let hp_percent = (combat_state.enemy_hp as f32 / enemy_max_hp * 100.0)
                    .max(0.0)
                    .min(100.0);
                style.width = Val::Percent(hp_percent);
            }
        }
    }
}
