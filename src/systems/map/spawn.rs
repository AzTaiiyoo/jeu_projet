use bevy::prelude::*;
use crate::assets::ImageAssets;
use crate::components::{MapTile, Position};
use crate::config::{TILE_SIZE, TERMINAL_WIDTH};
use crate::enemy::Enemy;
use crate::item::Item;
use crate::resources::{CollectedItems, DefeatedEnemies};
use crate::systems::map::{GameData, Tile};

/// Génère tous les éléments visuels de la map actuelle :
/// - Les tuiles (murs et chemins)
/// - Les objets collectables (en vérifiant qu'ils n'ont pas déjà été collectés)
/// - Les ennemis (en vérifiant qu'ils n'ont pas déjà été vaincus)
///
/// Z-ordering : Tiles (0.0) → Items (0.5) → Ennemis (0.7) → Joueur (1.0)
pub fn spawn_map(
    mut commands: Commands,
    game_data: Res<GameData>,
    image_assets: Res<ImageAssets>,
    collected_items: Res<CollectedItems>,
    defeated_enemies: Res<DefeatedEnemies>,
) {
    let game_map = game_data.get_current_map();
    
    // Offset pour décaler la map vers la gauche et la faire toucher le terminal
    let x_offset = -TERMINAL_WIDTH / 2.0;
    
    // Spawn map tiles
    for y in 0..game_map.height {
        for x in 0..game_map.width {
            let tile_type = game_map.grid[y][x];
            let texture = match tile_type {
                Tile::Path => image_assets.path_tile.clone(),
                Tile::Wall => image_assets.wall_tile.clone(),
                Tile::Connection => image_assets.path_tile.clone(),
            };

            let position = Vec2::new(
                x as f32 * TILE_SIZE - (game_map.width as f32 * TILE_SIZE / 2.0) + x_offset,
                y as f32 * TILE_SIZE - (game_map.height as f32 * TILE_SIZE / 2.0),
            );

            commands.spawn((
                SpriteBundle {
                    texture,
                    transform: Transform::from_translation(position.extend(0.0)),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..default()
                    },
                    ..default()
                },
                MapTile,
                Position { x, y },
            ));
        }
    }

    // Spawn items seulement s'ils n'ont pas été collectés
    for (pos, item_type) in &game_map.items {
        if collected_items
            .items
            .contains(&(game_data.current_map_index, *pos))
        {
            continue;
        }

        let item_texture = item_type.get_image_handle(&image_assets);
        let item_translation = Vec2::new(
            pos.x as f32 * TILE_SIZE - (game_map.width as f32 * TILE_SIZE / 2.0) + x_offset,
            pos.y as f32 * TILE_SIZE - (game_map.height as f32 * TILE_SIZE / 2.0),
        );

        commands.spawn((
            SpriteBundle {
                texture: item_texture,
                transform: Transform::from_translation(item_translation.extend(0.5)),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                ..default()
            },
            Item {
                item_type: *item_type,
            },
            *pos,
        ));
    }

    // Spawn ennemis seulement s'ils n'ont pas été vaincus
    for (pos, enemy_type) in &game_map.enemies {
        if defeated_enemies
            .enemies
            .contains(&(game_data.current_map_index, *pos))
        {
            continue;
        }

        let enemy_texture = enemy_type.get_image_handle(&image_assets);
        let enemy_translation = Vec2::new(
            pos.x as f32 * TILE_SIZE - (game_map.width as f32 * TILE_SIZE / 2.0) + x_offset,
            pos.y as f32 * TILE_SIZE - (game_map.height as f32 * TILE_SIZE / 2.0),
        );

        commands.spawn((
            SpriteBundle {
                texture: enemy_texture,
                transform: Transform::from_translation(enemy_translation.extend(0.7)),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                ..default()
            },
            Enemy::new(*enemy_type),
            *pos,
        ));
    }
}
