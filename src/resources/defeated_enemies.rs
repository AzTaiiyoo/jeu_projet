use crate::components::Position;
use bevy::prelude::Resource;
use std::collections::HashSet;

/// Resource pour stocker les ennemis vaincus par le joueur
/// Utilise un HashSet pour éviter les doublons
/// Stocke un tuple (index de la map, position de l'ennemi)
/// Permet de ne pas respawner les ennemis déjà vaincus lors des transitions entre maps
#[derive(Resource, Default)]
pub struct DefeatedEnemies {
    pub enemies: HashSet<(usize, Position)>,
}
