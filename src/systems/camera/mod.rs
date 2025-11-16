use bevy::{prelude::*, render::camera::ScalingMode};
use crate::config::TILE_SIZE;
use crate::systems::map::GameData;

/// Configure la caméra 2D avec scaling automatique basé sur la taille de la map
/// La caméra utilise AutoMin pour s'adapter à la fenêtre tout en gardant les proportions
pub fn setup_camera(mut commands: Commands, game_data: Res<GameData>) {
    let game_map = game_data.get_current_map();
    let window_width = game_map.width as f32 * TILE_SIZE;
    let window_height = game_map.height as f32 * TILE_SIZE;

    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: window_width,
        min_height: window_height,
    };
    commands.spawn(camera);
}
