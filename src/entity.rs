/// Une structure de statistiques partagée par le joueur, les ennemis et les objets
#[derive(Debug, Clone, Copy, Default)]
pub struct Stats {
    pub hp: i32,
    pub attack: i32,
    pub speed: i32,
    pub critical_chance: i32, // Chance de coup critique en %
}

/// Une position sur la carte (coordonnées x, y)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

/// Les différents types d'objets que l'on peut trouver
#[derive(Debug, Clone, Copy)]
pub enum ItemType {
    HealthBoost,
    AttackBoost,
    SpeedBoost,
}

/// Un objet placé sur la carte
#[derive(Debug, Clone, Copy)]
pub struct Item {
    pub item_type: ItemType,
    pub position: Position,
    pub stats: Stats, // Les stats que l'objet confère
    pub name: &'static str,
}

/// Un ennemi sur la carte
#[derive(Debug, Clone, Copy)]
pub struct Enemy {
    pub id: u32,
    pub name: &'static str,
    pub stats: Stats,
    pub position: Position,
}