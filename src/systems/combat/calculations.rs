use crate::components::Stats;
use rand::Rng;

/// Résultat d'un calcul de dégâts
#[derive(Debug)]
pub struct DamageResult {
    pub damage: i32,
    pub is_critical: bool,
}

/// Calcule les dégâts infligés par un attaquant
/// Prend en compte la chance de coup critique qui double les dégâts
pub fn calculate_damage(attacker_stats: &Stats, rng: &mut impl Rng) -> DamageResult {
    let mut damage = attacker_stats.attack;
    let crit_roll = rng.gen_range(1..=100);
    let is_critical = crit_roll <= attacker_stats.critical_chance;
    
    if is_critical {
        damage *= 2;
    }
    
    DamageResult { damage, is_critical }
}

/// Vérifie si une attaque est esquivée
/// La chance d'esquive est basée sur la vitesse du défenseur (1-100)
pub fn check_dodge(defender_speed: i32, rng: &mut impl Rng) -> bool {
    let dodge_roll = rng.gen_range(1..=100);
    dodge_roll <= defender_speed
}
