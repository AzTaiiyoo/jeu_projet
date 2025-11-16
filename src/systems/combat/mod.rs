mod calculations;
mod cleanup;
mod logic;
mod setup;

pub use cleanup::cleanup_combat;
pub use logic::handle_combat;
pub use setup::setup_combat;

// Réexporter les fonctions de calcul pour usage futur (tests, etc.)
#[allow(unused_imports)]
pub use calculations::{calculate_damage, check_dodge, DamageResult};
