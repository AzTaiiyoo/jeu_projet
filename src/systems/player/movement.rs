use bevy::prelude::*;
use crate::components::Position;
use crate::player::Player;
use crate::states::GameState;
use crate::systems::map::GameData;

/// Gère le déplacement du joueur avec les touches Z/Q/S/D ou flèches directionnelles
/// Vérifie que la nouvelle position est praticable avant de déplacer le joueur
/// Détecte les connexions entre maps et déclenche une transition si nécessaire
pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Position, With<Player>>,
    mut game_data: ResMut<GameData>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let mut player_pos = player_query.single_mut();
    let mut new_pos = player_pos.clone();

    let game_map = game_data.get_current_map();

    if keyboard_input.just_pressed(KeyCode::Z) || keyboard_input.just_pressed(KeyCode::Up) {
        new_pos.y = new_pos.y.saturating_sub(1);
    } else if keyboard_input.just_pressed(KeyCode::S) || keyboard_input.just_pressed(KeyCode::Down)
    {
        new_pos.y = (new_pos.y + 1).min(game_map.height - 1);
    } else if keyboard_input.just_pressed(KeyCode::Q) || keyboard_input.just_pressed(KeyCode::Left)
    {
        new_pos.x = new_pos.x.saturating_sub(1);
    } else if keyboard_input.just_pressed(KeyCode::D) || keyboard_input.just_pressed(KeyCode::Right)
    {
        new_pos.x = (new_pos.x + 1).min(game_map.width - 1);
    }

    if game_map.is_walkable(new_pos.x, new_pos.y) {
        *player_pos = new_pos;

        let connection = game_map.connections.get(&new_pos).cloned();
        if let Some((to_map_index, to_position)) = connection {
            game_data.current_map_index = to_map_index;
            *player_pos = to_position;
            next_state.set(GameState::MapTransition);
        }
    }
}
