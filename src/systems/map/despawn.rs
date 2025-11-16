use bevy::prelude::*;
use crate::components::MapTile;
use crate::enemy::Enemy;
use crate::item::Item;

/// Détruit toutes les entités de la map lors du changement d'état
/// Nettoie les tuiles, objets et ennemis pour préparer le chargement de la nouvelle map
pub fn despawn_map(
    mut commands: Commands,
    map_tile_query: Query<Entity, With<MapTile>>,
    item_query: Query<Entity, With<Item>>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    for entity in map_tile_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in item_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in enemy_query.iter() {
        commands.entity(entity).despawn();
    }
}
