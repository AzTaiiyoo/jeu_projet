use crate::assets::ImageAssets;
use crate::components::ClassSelectionUI;
use crate::config::{TERMINAL_WIDTH, TILE_SIZE};
use crate::player::Player;
use crate::resources::SelectedClass;
use crate::systems::map::GameData;
use bevy::prelude::*;

pub fn cleanup_ui(mut commands: Commands, ui_query: Query<Entity, With<ClassSelectionUI>>) {
    for entity in ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Crée l'entité joueur avec la classe sélectionnée et le positionne sur la map
/// Appelé lors de la sortie de l'état ClassSelection
/// La ressource SelectedClass est supprimée après utilisation
pub fn spawn_player(
    mut commands: Commands,
    selected_class: Res<SelectedClass>,
    game_data: Res<GameData>,
    image_assets: Res<ImageAssets>,
) {
    let game_map = game_data.get_current_map();
    let player_class = selected_class.0;
    let player_data = Player::new(player_class, game_map.player_start);
    let player_texture = player_data.class.get_image_handle(&image_assets);

    // Offset pour décaler le joueur vers la gauche et éviter le chevauchement avec le terminal
    let x_offset = -TERMINAL_WIDTH / 2.0;

    let player_translation = Vec2::new(
        game_map.player_start.x as f32 * TILE_SIZE - (game_map.width as f32 * TILE_SIZE / 2.0)
            + x_offset,
        game_map.player_start.y as f32 * TILE_SIZE - (game_map.height as f32 * TILE_SIZE / 2.0),
    );

    commands.spawn((
        SpriteBundle {
            texture: player_texture,
            transform: Transform::from_translation(player_translation.extend(1.0)),
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..default()
            },
            ..default()
        },
        player_data,
        game_map.player_start,
    ));

    commands.remove_resource::<SelectedClass>();
}
