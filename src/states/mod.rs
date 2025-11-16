use bevy::prelude::States;

/// États du jeu utilisés pour gérer le flow de l'application
/// - ClassSelection: Écran de sélection de la classe du joueur
/// - Map: Mode exploration où le joueur se déplace sur la carte
/// - Combat: Mode combat tour par tour contre un ennemi
/// - MapTransition: État temporaire pour changer de carte
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    ClassSelection,
    Map,
    Combat,
    MapTransition,
}
