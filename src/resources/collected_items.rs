use crate::components::Position;
use bevy::prelude::Resource;
use std::collections::HashSet;

/// Resource pour stocker les objets collectés par le joueur
/// Utilise un HashSet pour éviter les doublons
/// Stocke un tuple (index de la map, position de l'objet)
/// Permet de ne pas respawner les objets déjà collectés lors des transitions entre maps
#[derive(Resource, Default)]
pub struct CollectedItems {
    pub items: HashSet<(usize, Position)>,
}
