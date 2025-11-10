use bevy::prelude::Component;

/// Une structure de statistiques partagée par le joueur, les ennemis et les objets
#[derive(Debug, Clone, Copy, Default)]
pub struct Stats {
    pub hp: i32,
    pub attack: i32,
    pub speed: i32,
    pub critical_chance: i32, // Chance de coup critique en %
}

/// Une position sur la carte (coordonnées x, y)
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

/// Un ennemi sur la carte
#[derive(Debug, Clone, Copy)]
pub struct Enemy {
    pub id: u32,
    pub name: &'static str,
    pub stats: Stats,
    pub position: Position,
}