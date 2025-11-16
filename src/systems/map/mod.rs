mod data;
mod despawn;
mod spawn;
mod transition;

pub use data::GameData;
pub use despawn::despawn_map;
pub use spawn::spawn_map;
pub use transition::map_transition;

// Réexporter pour usage futur
#[allow(unused_imports)]
pub use data::{Map, Tile};
