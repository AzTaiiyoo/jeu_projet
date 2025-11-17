use crate::components::Position;
use crate::enemy::EnemyType;
use crate::item::ItemType;
use bevy::prelude::Resource;
use std::collections::HashMap;

/// Les différents types de tuiles qui composent la carte
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
    Wall,       // Mur infranchissable (bloque le déplacement)
    Path,       // Chemin praticable (permet le déplacement)
    Connection, // Point de connexion vers une autre map
}

/// Représente une carte du jeu
/// Contient la grille de tuiles, les objets, les ennemis et les connexions
#[derive(Resource, Clone, Default)]
pub struct Map {
    pub grid: Vec<Vec<Tile>>,
    pub player_start: Position,
    pub width: usize,
    pub height: usize,
    pub connections: HashMap<Position, (usize, Position)>,
    pub items: Vec<(Position, ItemType)>,
    pub enemies: Vec<(Position, EnemyType)>,
}

impl Map {
    /// Vérifie si une position donnée est praticable (pas un mur, dans les limites)
    /// Utilisé pour la validation du déplacement du joueur
    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        if y >= self.grid.len() || x >= self.grid[y].len() {
            return false;
        }
        matches!(self.grid[y][x], Tile::Path | Tile::Connection)
    }
}

/// Resource Bevy contenant toutes les maps du jeu
/// Gère la map actuelle et permet de naviguer entre les maps
#[derive(Resource)]
pub struct GameData {
    pub maps: Vec<Map>,
    pub current_map_index: usize,
}

impl GameData {
    /// Crée les données de jeu avec toutes les maps prédéfinies
    pub fn new() -> Self {
        let mut maps = Vec::new();

        // Map 1 - Agrandie avec des ennemis
        let layout1 = vec![
            "WWWWWWWWWWWWWW",
            "WP P P P P P W",
            "W WWWW P WWW W",
            "W P  P P   P W",
            "W P WWWWWW P W",
            "W P  P   P P C", // Connection at x=13, y=5
            "W WWWP WWW P W",
            "W P  P   P P W",
            "W P P P  P P W",
            "WWWWWWWWWWWWWW",
        ];
        let mut connections1 = HashMap::new();
        connections1.insert(Position { x: 13, y: 5 }, (1, Position { x: 1, y: 5 }));
        let items1 = vec![
            (Position { x: 2, y: 3 }, ItemType::Katana),
            (Position { x: 7, y: 7 }, ItemType::Armure),
        ];
        let enemies1 = vec![
            (Position { x: 4, y: 1 }, EnemyType::SmallGoblin),
            (Position { x: 8, y: 3 }, EnemyType::SmallGoblin),
            (Position { x: 5, y: 5 }, EnemyType::MediumGoblin),
            (Position { x: 10, y: 7 }, EnemyType::SmallGoblin),
        ];

        let grid1: Vec<Vec<Tile>> = layout1
            .iter()
            .map(|row| {
                row.chars()
                    .map(|c| match c {
                        'W' => Tile::Wall,
                        'C' => Tile::Connection,
                        _ => Tile::Path,
                    })
                    .collect()
            })
            .collect();

        let height1 = grid1.len();
        let width1 = grid1.get(0).map_or(0, |row| row.len());

        maps.push(Map {
            grid: grid1,
            player_start: Position { x: 1, y: 1 },
            width: width1,
            height: height1,
            connections: connections1,
            items: items1,
            enemies: enemies1,
        });

        // Map 2 - Agrandie avec plus d'ennemis
        let layout2 = vec![
            "WWWWWWWWWWWWWW",
            "W P P P P P PW",
            "W W PPPPPPPP W",
            "W P P  P   P W",
            "W WWWP WWWWP W",
            "C P  P     P W", // Connection at x=0, y=5
            "W WWWWWWWW P W",
            "W P    P P P W",
            "W P PP P P P W",
            "WWWWWWWWWWWWWW",
        ];
        let mut connections2 = HashMap::new();
        connections2.insert(Position { x: 0, y: 5 }, (0, Position { x: 12, y: 5 }));
        let items2 = vec![
            (Position { x: 10, y: 2 }, ItemType::Gants),
            (Position { x: 11, y: 7 }, ItemType::Pendentif),
        ];
        let enemies2 = vec![
            (Position { x: 5, y: 2 }, EnemyType::MediumGoblin),
            (Position { x: 8, y: 3 }, EnemyType::MediumGoblin),
            (Position { x: 10, y: 5 }, EnemyType::LargeGoblin),
            (Position { x: 6, y: 7 }, EnemyType::MediumGoblin),
            (Position { x: 11, y: 8 }, EnemyType::Wolf),
        ];

        let grid2: Vec<Vec<Tile>> = layout2
            .iter()
            .map(|row| {
                row.chars()
                    .map(|c| match c {
                        'W' => Tile::Wall,
                        'C' => Tile::Connection,
                        _ => Tile::Path,
                    })
                    .collect()
            })
            .collect();

        let height2 = grid2.len();
        let width2 = grid2.get(0).map_or(0, |row| row.len());

        maps.push(Map {
            grid: grid2,
            player_start: Position { x: 1, y: 1 },
            width: width2,
            height: height2,
            connections: connections2,
            items: items2,
            enemies: enemies2,
        });

        GameData {
            maps,
            current_map_index: 0,
        }
    }

    /// Retourne une référence à la map actuellement active
    pub fn get_current_map(&self) -> &Map {
        &self.maps[self.current_map_index]
    }
}
