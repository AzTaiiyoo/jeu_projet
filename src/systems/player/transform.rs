use bevy::prelude::*;
use crate::components::Position;
use crate::config::TILE_SIZE;
use crate::player::Player;
use crate::systems::map::GameData;

/// Synchronise la position visuelle (Transform) du joueur avec sa position logique (Position)
/// Met à jour les coordonnées du sprite en tenant compte de la taille de la map
/// Maintient le Z-order à 1.0 pour que le joueur soit toujours visible au-dessus des autres éléments
pub fn update_transform(
    game_data: Res<GameData>,
    mut player_query: Query<(&Position, &mut Transform), With<Player>>,
) {
    if let Ok((player_pos, mut player_transform)) = player_query.get_single_mut() {
        let game_map = game_data.get_current_map();
        let translation = Vec2::new(
            player_pos.x as f32 * TILE_SIZE - (game_map.width as f32 * TILE_SIZE / 2.0),
            player_pos.y as f32 * TILE_SIZE - (game_map.height as f32 * TILE_SIZE / 2.0),
        );
        player_transform.translation = translation.extend(1.0);
    }
}
