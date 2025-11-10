// Fait savoir à Rust que nous avons d'autres fichiers (modules)
mod combat;
mod entity;
mod game;
mod map;
mod player;

use game::Game;
use player::{Class, Player};
use std::io;

fn main() {
    println!("Bienvenue dans l'Aventure !");

    // 1. Sélection de la classe
    let player_class = choose_class();
    println!("Vous avez choisi la classe : {:?}", player_class);

    // 2. Initialisation du jeu
    let mut game = Game::new(player_class);

    // 3. Lancement de la boucle de jeu principale
    game.run();
}

/// Demande à l'utilisateur de choisir une classe
fn choose_class() -> Class {
    loop {
        println!("Choisissez votre classe :");
        println!("1. Guerrier (Plus de vie et d'attaque)");
        println!("2. Voleur (Plus de vitesse)");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Échec de la lecture de l'entrée");

        match choice.trim() {
            "1" => return Class::Warrior,
            "2" => return Class::Rogue,
            _ => println!("Choix invalide. Veuillez taper 1 ou 2."),
        }
    }
}