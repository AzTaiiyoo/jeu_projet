use bevy::{prelude::*, window::PresentMode};

mod assets;
mod components;
mod config;
mod enemy;
mod entity;
mod item;
mod player;
mod resources;
mod states;
mod systems;

use config::TILE_SIZE;
use resources::*;
use states::GameState;
use systems::{camera, class_selection, combat, map, player as player_systems, ui};

fn main() {
    let game_data = map::GameData::new();
    let first_map = game_data.get_current_map();
    let window_width = first_map.width as f32 * TILE_SIZE;
    let window_height = first_map.height as f32 * TILE_SIZE;

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Elden World".into(),
                resolution: (window_width + 300.0, window_height).into(),
                present_mode: PresentMode::AutoVsync,
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(game_data)
        .insert_resource(CollectedItems::default())
        .insert_resource(DefeatedEnemies::default())
        .insert_resource(GameLog::default())
        .add_state::<GameState>()
        .add_systems(Startup, camera::setup_camera)
        .add_systems(
            OnEnter(GameState::ClassSelection),
            (assets::load_assets, class_selection::setup_ui),
        )
        .add_systems(
            Update,
            class_selection::handle_input.run_if(in_state(GameState::ClassSelection)),
        )
        .add_systems(
            OnExit(GameState::ClassSelection),
            (class_selection::cleanup_ui, class_selection::spawn_player),
        )
        .add_systems(OnEnter(GameState::Map), (map::spawn_map, ui::setup_info_terminal))
        .add_systems(OnExit(GameState::Map), (map::despawn_map, ui::cleanup_info_terminal))
        .add_systems(OnEnter(GameState::MapTransition), map::map_transition)
        .add_systems(
            Update,
            (
                player_systems::move_player,
                player_systems::update_transform,
                player_systems::check_item_pickup,
                player_systems::check_enemy_encounter,
                ui::update_info_terminal,
            )
                .run_if(in_state(GameState::Map)),
        )
        .add_systems(OnEnter(GameState::Combat), combat::setup_combat)
        .add_systems(Update, combat::handle_combat.run_if(in_state(GameState::Combat)))
        .add_systems(OnExit(GameState::Combat), combat::cleanup_combat)
        .run();
}
