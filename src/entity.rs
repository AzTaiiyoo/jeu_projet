use bevy::prelude::Component;

/// Structure de statistiques partagée par le joueur, les ennemis et les objets
/// Cette centralisation garantit la cohérence des calculs de combat
///
/// Statistiques :
/// - hp: Points de vie (Health Points)
/// - attack: Dégâts infligés par attaque
/// - speed: Chance d'esquive en % (1-100)
/// - critical_chance: Chance de coup critique en % (×2 dégâts)
#[derive(Debug, Clone, Copy, Default)]
pub struct Stats {
    pub hp: i32,
    pub attack: i32,
    pub speed: i32,
    pub critical_chance: i32, // Chance de coup critique en %
}

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

/// Structure d'ennemi (non utilisée dans le code actuel, voir enemy.rs à la place)
#[derive(Debug, Clone, Copy)]
pub struct Enemy {
    pub id: u32,
    pub name: &'static str,
    pub stats: Stats,
    pub position: Position,
}
