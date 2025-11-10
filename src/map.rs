use bevy::prelude::Resource;
use crate::entity::Position;

/// Les différents types de tuiles qui composent la carte
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
    Wall, // Un mur infranchissable
    Path, // Un chemin praticable
}

pub const TILE_SIZE: f32 = 64.0;

/// La structure de la carte du jeu comme ressource Bevy
#[derive(Resource)]
pub struct GameMap {
    pub grid: Vec<Vec<Tile>>,
    pub player_start: Position,
    pub width: usize,
    pub height: usize,
}

impl GameMap {
    /// Crée une nouvelle carte de jeu (statique pour l'instant)
    pub fn new() -> Self {
        // 'W' = Mur, 'P' = Chemin
        let layout = vec![
            "WWWWWWWWWW",
            "WP P P P W", // Le 'P' à (1, 1) est le départ
            "W WWWW P W",
            "W P  P P W",
            "W WWWW W W",
            "W P  P   W", // Un chemin vers un objet/ennemi
            "WWWWWWWWWW",
        ];

        let grid: Vec<Vec<Tile>> = layout
            .iter()
            .map(|row| {
                row.chars()
                    .map(|c| match c {
                        'W' => Tile::Wall,
                        _ => Tile::Path, // 'P' et ' ' sont des chemins
                    })
                    .collect()
            })
            .collect();

        let player_start = Position { x: 1, y: 1 };
        let height = grid.len();
        let width = grid.get(0).map_or(0, |row| row.len());

        GameMap { grid, player_start, width, height }
    }

    /// Vérifie si une tuile donnée est praticable
    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        // Vérifie les limites de la carte
        if y >= self.grid.len() || x >= self.grid[y].len() {
            return false;
        }
        // Vérifie le type de tuile
        matches!(self.grid[y][x], Tile::Path)
    }
}