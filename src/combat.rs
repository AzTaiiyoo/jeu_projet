use crate::entity::Enemy;
use crate::player::Player;
use rand::Rng;

/// Démarre un combat tour par tour.
/// Renvoie 'true' si le joueur gagne, 'false' s'il perd.
pub fn start_combat(player: &mut Player, enemy: &Enemy) -> bool {
    println!(
        "Combat contre {} (HP: {}, ATK: {}, SPD: {}, CRIT: {}%)",
        enemy.name,
        enemy.stats.hp,
        enemy.stats.attack,
        enemy.stats.speed,
        enemy.stats.critical_chance
    );

    let mut rng = rand::thread_rng();
    let mut enemy_hp = enemy.stats.hp;
    let mut player_hp = player.stats.hp; // Utilise une copie pour le combat

    loop {
        // --- Tour du joueur ---
        println!("Vous attaquez !");
        // 1. Chance d'esquive de l'ennemi
        if rng.gen_range(1..=100) <= enemy.stats.speed {
            println!("L'ennemi a esquivé votre attaque !");
        } else {
            // 2. Calcul des dégâts (avec chance de critique)
            let mut damage = player.stats.attack;
            if rng.gen_range(1..=100) <= player.stats.critical_chance {
                println!("Coup critique !");
                damage *= 2;
            }
            enemy_hp -= damage;
            println!(
                "L'ennemi subit {} dégâts. HP restants : {}",
                damage,
                enemy_hp.max(0)
            );
        }

        if enemy_hp <= 0 {
            player.stats.hp = player_hp; // Restaure les HP du joueur à la fin du combat
            return true; // Le joueur gagne
        }

        println!("---");

        // --- Tour de l'ennemi ---
        println!("L'ennemi attaque !");
        // 1. Chance d'esquive du joueur
        if rng.gen_range(1..=100) <= player.stats.speed {
            println!("Vous avez esquivé l'attaque !");
        } else {
            // 2. Calcul des dégâts (avec chance de critique)
            let mut damage = enemy.stats.attack;
            if rng.gen_range(1..=100) <= enemy.stats.critical_chance {
                println!("L'ennemi vous inflige un coup critique !");
                damage *= 2;
            }
            player_hp -= damage;
            println!(
                "Vous subissez {} dégâts. HP restants : {}",
                damage,
                player_hp.max(0)
            );
        }

        if player_hp <= 0 {
            player.stats.hp = 0; // Met à jour les HP réels du joueur
            return false; // Le joueur perd
        }

        println!("--- Prochain tour ---");
        // Petite pause pour voir ce qu'il se passe
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
