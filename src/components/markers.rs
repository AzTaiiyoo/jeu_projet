use crate::player::PlayerClass;
use bevy::prelude::Component;

// ============ Marker Components ============
// Utilisés uniquement pour identifier/filtrer des entités dans les queries

/// Marker component pour les tuiles de la map
#[derive(Component)]
pub struct MapTile;

/// Marker component pour l'UI de sélection de classe
#[derive(Component)]
pub struct ClassSelectionUI;

/// Marker component pour bouton de sélection de classe
#[derive(Component)]
pub struct ClassButton(pub PlayerClass);

/// Marker component pour le terminal d'information
#[derive(Component)]
pub struct InfoTerminal;

/// Marker component pour le texte des stats du joueur
#[derive(Component)]
pub struct StatsText;

/// Marker component pour le texte du log de jeu
#[derive(Component)]
pub struct LogText;

/// Marker component pour l'UI de combat
#[derive(Component)]
pub struct CombatUI;

/// Marker component pour le texte du log de combat
#[derive(Component)]
pub struct CombatLogText;

/// Marker component pour une barre de vie (joueur ou ennemi)
#[derive(Component)]
pub enum HealthBar {
    Player,
    Enemy,
}

/// Marker component pour le texte des statistiques du joueur en combat
#[derive(Component)]
pub struct PlayerStatsText;

/// Marker component pour le texte des statistiques de l'ennemi en combat
#[derive(Component)]
pub struct EnemyStatsText;

/// Marker component pour le message d'action au centre (tour actuel)
#[derive(Component)]
pub struct ActionMessageText;
