use crate::combat::start_combat;
use crate::entity::{Enemy, Item, ItemType, Position, Stats};
use crate::map::{Map, Tile};
use crate::player::{Class, Player};
use std::io;

/// Les différents états possibles du jeu
pub enum GameState {
    Roaming, // Le joueur se déplace sur la carte
    Combat,  // Le joueur est en combat
    Menu,    // Le joueur est dans un menu (non implémenté)
    End,     // Le jeu est terminé
}

/// La structure principale du jeu, contenant tout l'état
pub struct Game {
    pub player: Player,
    pub map: Map,
    pub items: Vec<Item>,
    pub enemies: Vec<Enemy>,
    pub state: GameState,
}

impl Game {
    /// Initialise une nouvelle partie
    pub fn new(player_class: Class) -> Self {
        let map = Map::new();
        let player = Player::new(player_class, map.player_start);

        // --- Placement fixe des objets ---
        let items = vec![
            Item {
                item_type: ItemType::HealthBoost,
                position: Position { x: 3, y: 3 },
                stats: Stats { hp: 50, ..Default::default() },
                name: "Grande Potion de Vie",
            },
            Item {
                item_type: ItemType::AttackBoost,
                position: Position { x: 5, y: 5 },
                stats: Stats { attack: 10, ..Default::default() },
                name: "Épée tranchante",
            },
        ];

        // --- Placement fixe des ennemis ---
        let enemies = vec![
            Enemy {
                id: 1,
                name: "Gobelin faible",
                stats: Stats { hp: 30, attack: 5, speed: 3, critical_chance: 0 },
                position: Position { x: 5, y: 1 },
            },
            Enemy {
                id: 2,
                name: "Orc",
                stats: Stats { hp: 80, attack: 10, speed: 2, critical_chance: 5 },
                position: Position { x: 3, y: 5 },
            },
        ];

        Game {
            player,
            map,
            items,
            enemies,
            state: GameState::Roaming,
        }
    }

    /// La boucle de jeu principale
    pub fn run(&mut self) {
        // Utilise 'loop' au lieu de 'while' pour gérer les différents états
        loop {
            match self.state {
                GameState::Roaming => {
                    // 1. Afficher l'état actuel
                    self.map.display(&self.player, &self.items, &self.enemies);

                    // 2. Obtenir l'entrée du joueur
                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Échec de la lecture");

                    // 3. Mettre à jour l'état du jeu en fonction de l'entrée
                    self.update_roaming(input.trim());
                }
                GameState::Combat => {
                    println!("--- COMBAT ENGAGÉ ! ---");
                    // Trouve l'ennemi avec lequel on combat
                    let enemy_index = self
                        .enemies
                        .iter()
                        .position(|e| e.position == self.player.position)
                        .unwrap();
                    
                    let enemy = self.enemies[enemy_index];

                    let combat_result = start_combat(&mut self.player, &enemy);

                    if combat_result {
                        println!("Vous avez vaincu {}!", enemy.name);
                        // Enlève l'ennemi de la liste
                        self.enemies.remove(enemy_index);
                        // Propose au joueur d'améliorer une stat
                        self.prompt_stat_increase();
                    } else {
                        println!("Vous avez été vaincu...");
                        self.state = GameState::End; // Fin du jeu
                    }

                    // Après le combat, on retourne au déplacement
                    if !matches!(self.state, GameState::End) {
                        self.state = GameState::Roaming;
                    }
                }
                GameState::Menu => {
                    // Non implémenté
                    println!("Menu non implémenté.");
                    self.state = GameState::Roaming; // Retour au jeu
                }
                GameState::End => {
                    println!("Fin de la partie. Merci d'avoir joué !");
                    break; // Sort de la boucle 'loop'
                }
            }
        }
    }

    /// Gère la logique lorsque le joueur se déplace sur la carte
    fn update_roaming(&mut self, input: &str) {
        // Détermine la nouvelle position souhaitée
        let mut new_pos = self.player.position;
        match input.to_lowercase().as_str() {
            "z" => new_pos.y = new_pos.y.saturating_sub(1), // Haut
            "s" => new_pos.y += 1,                          // Bas
            "q" => new_pos.x = new_pos.x.saturating_sub(1), // Gauche
            "d" => new_pos.x += 1,                          // Droite
            _ => return, // Ne fait rien si l'entrée est invalide
        }

        // 1. Vérifier si la nouvelle position est un chemin praticable
        if !self.map.is_walkable(new_pos.x, new_pos.y) {
            println!("Vous ne pouvez pas aller là. C'est un mur.");
            return;
        }

        // 2. Vérifier s'il y a un ennemi à la nouvelle position
        if self.enemies.iter().any(|e| e.position == new_pos) {
            println!("Un ennemi bloque le passage !");
            self.player.position = new_pos; // Déplace le joueur sur l'ennemi
            self.state = GameState::Combat; // Déclenche le combat
            return;
        }

        // 3. Vérifier s'il y a un objet à la nouvelle position
        if let Some(index) = self.items.iter().position(|i| i.position == new_pos) {
            // Ramasser l'objet
            let item = self.items.remove(index); // Enlève l'objet de la carte
            println!("Vous avez trouvé : {} !", item.name);
            self.player.apply_item_stats(&item.stats);
        }

        // 4. Mettre à jour la position du joueur
        self.player.position = new_pos;
    }

    /// Demande au joueur quelle statistique il souhaite augmenter après une victoire
    fn prompt_stat_increase(&mut self) {
        loop {
            println!("\nChoisissez une statistique à améliorer :");
            println!("1. Vie (+10 HP)");
            println!("2. Attaque (+2 ATK)");
            println!("3. Vitesse/Esquive (+2 SPD)");
            println!("4. Chance de critique (+2 % CRIT)");

            let mut choice = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("Échec de la lecture de l'entrée");

            match choice.trim() {
                "1" => {
                    self.player.stats.hp += 10;
                    println!("Votre vie a augmenté ! HP actuels : {}", self.player.stats.hp);
                    break;
                }
                "2" => {
                    self.player.stats.attack += 2;
                    println!("Votre attaque a augmenté ! ATK actuelle : {}", self.player.stats.attack);
                    break;
                }
                "3" => {
                    self.player.stats.speed += 2;
                    println!("Votre vitesse a augmenté ! SPD actuelle : {}", self.player.stats.speed);
                    break;
                }
                "4" => {
                    self.player.stats.critical_chance += 2;
                    println!("Votre chance de critique a augmenté ! CRIT actuelle : {}%", self.player.stats.critical_chance);
                    break;
                }
                _ => println!("Choix invalide. Veuillez taper un numéro de 1 à 4."),
            }
        }
    }
}