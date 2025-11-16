use bevy::prelude::Component;

/// Position logique sur la grille de la carte (coordonnées x, y)
/// Utilisée par tous les éléments positionnables (joueur, ennemis, objets, tuiles)
///
/// Implémente Hash et Eq pour pouvoir être utilisée comme clé dans des HashSet/HashMap
/// (nécessaire pour tracker les objets collectés et ennemis vaincus)
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
