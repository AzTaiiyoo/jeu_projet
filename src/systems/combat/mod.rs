mod calculations;
mod cleanup;
mod health_bars;
mod logic;
mod setup;

pub use cleanup::cleanup_combat;
pub use health_bars::update_health_bars;
pub use logic::handle_combat;
pub use setup::setup_combat;

// Réexporter les fonctions de calcul pour usage futur (tests, etc.)
#[allow(unused_imports)]
pub use calculations::{DamageResult, calculate_damage, check_dodge};
