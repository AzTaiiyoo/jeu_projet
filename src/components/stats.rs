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
    pub critical_chance: i32,
}
