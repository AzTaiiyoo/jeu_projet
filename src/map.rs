use crate::entity::{Enemy, Item, Position};
use crate::player::Player;

/// Les différents types de tuiles qui composent la carte
/// Note : C'est la *carte de base*. Les objets et ennemis sont gérés séparément
/// pour plus de flexibilité (dans game.rs)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
    Wall, // Un mur infranchissable
    Path, // Un chemin praticable
}

/// La structure de la carte du jeu
pub struct Map {
    pub grid: Vec<Vec<Tile>>,
    pub player_start: Position,
}

impl Map {
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

        Map { grid, player_start }
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

    /// Affiche la carte dans le terminal
    pub fn display(&self, player: &Player, items: &[Item], enemies: &[Enemy]) {
        // Efface le terminal (fonctionne sur Linux/macOS, peut nécessiter 'cls' sur Windows)
        print!("\x1B[2J\x1B[H");

        for (y, row) in self.grid.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let pos = Position { x, y };
                // L'ordre d'affichage est important : Joueur > Ennemi > Objet > Tuile
                if player.position == pos {
                    print!("@ "); // Joueur
                } else if enemies.iter().any(|e| e.position == pos) {
                    print!("E "); // Ennemi
                } else if items.iter().any(|i| i.position == pos) {
                    print!("O "); // Objet
                } else {
                    match tile {
                        Tile::Wall => print!("# "), // Mur
                        Tile::Path => print!(". "), // Chemin
                    }
                }
            }
            println!(); // Nouvelle ligne à la fin de la rangée
        }
        println!("\n--- Légende ---");
        println!("@: Vous | E: Ennemi | O: Objet | #: Mur | .: Chemin");
        println!("Utilisez Z (haut), S (bas), Q (gauche), D (droite) pour vous déplacer.");
        println!("Stats actuelles : HP: {} | ATK: {} | SPD: {}", 
            player.stats.hp, player.stats.attack, player.stats.speed);
    }
}