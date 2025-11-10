use bevy::{prelude::*, render::camera::ScalingMode, window::PresentMode};
use std::collections::HashSet;

mod assets;
mod combat;
mod enemy;
mod entity;
mod item;
mod map;
mod player;

use crate::assets::ImageAssets;
use crate::entity::Position;

// Game states
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    ClassSelection,
    Map,
    Combat,
    MapTransition,
}

// Resource to hold the selected player class
#[derive(Resource)]
struct SelectedClass(player::PlayerClass);

// Resource pour tracker les objets collectés (map_index, position)
#[derive(Resource, Default)]
struct CollectedItems {
    items: HashSet<(usize, Position)>,
}

// Resource pour tracker les ennemis vaincus (map_index, position)
#[derive(Resource, Default)]
struct DefeatedEnemies {
    enemies: HashSet<(usize, Position)>,
}

// Resource pour stocker les messages de jeu
#[derive(Resource, Default)]
struct GameLog {
    messages: Vec<String>,
}

impl GameLog {
    fn add_message(&mut self, message: String) {
        println!("Ajout message au log: {}", message);
        self.messages.push(message);
        // Garder seulement les 10 derniers messages
        if self.messages.len() > 10 {
            self.messages.remove(0);
        }
        println!("Total messages dans log: {}", self.messages.len());
    }
}

// Resource pour stocker l'ennemi en combat
#[derive(Resource)]
struct CurrentEnemy {
    entity: Entity,
    position: Position,
    enemy_type: enemy::EnemyType,
    hp: i32,
    stats: entity::Stats,
}

// Resource pour l'état du combat
#[derive(Resource, Default)]
struct CombatState {
    player_hp: i32,
    enemy_hp: i32,
    combat_log: Vec<String>,
    is_player_turn: bool,
}

fn main() {
    let game_data = map::GameData::new();
    let first_map = game_data.get_current_map();
    let window_width = first_map.width as f32 * map::TILE_SIZE;
    let window_height = first_map.height as f32 * map::TILE_SIZE;

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Jeu de l'Aventure".into(),
                resolution: (window_width + 300.0, window_height).into(), // Ajouter de l'espace pour le terminal
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
        .add_systems(Startup, setup_camera)
        .add_systems(
            OnEnter(GameState::ClassSelection),
            (assets::load_assets, setup_class_selection_ui),
        )
        .add_systems(
            Update,
            handle_class_selection.run_if(in_state(GameState::ClassSelection)),
        )
        .add_systems(
            OnExit(GameState::ClassSelection),
            (cleanup_class_selection_ui, spawn_player),
        )
        .add_systems(OnEnter(GameState::Map), (spawn_map, setup_info_terminal))
        .add_systems(OnExit(GameState::Map), (despawn_map, cleanup_info_terminal))
        .add_systems(OnEnter(GameState::MapTransition), map_transition)
        .add_systems(
            Update,
            (
                move_player,
                update_player_transform,
                check_for_item_pickup,
                check_for_enemy_encounter,
                update_info_terminal,
            )
                .run_if(in_state(GameState::Map)),
        )
        .add_systems(OnEnter(GameState::Combat), setup_combat)
        .add_systems(Update, handle_combat.run_if(in_state(GameState::Combat)))
        .add_systems(OnExit(GameState::Combat), cleanup_combat)
        .run();
}

fn setup_camera(mut commands: Commands, game_data: Res<map::GameData>) {
    let game_map = game_data.get_current_map();
    let window_width = game_map.width as f32 * map::TILE_SIZE;
    let window_height = game_map.height as f32 * map::TILE_SIZE;

    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: window_width,
        min_height: window_height,
    };
    commands.spawn(camera);
}

// UI Marker component
#[derive(Component)]
struct ClassSelectionUI;

// Component to hold PlayerClass for buttons
#[derive(Component)]
struct ClassButton(player::PlayerClass);

// Marker component pour le terminal d'information
#[derive(Component)]
struct InfoTerminal;

// Marker component pour le texte des stats
#[derive(Component)]
struct StatsText;

// Marker component pour le texte du log
#[derive(Component)]
struct LogText;

// Marker components pour l'UI de combat
#[derive(Component)]
struct CombatUI;

#[derive(Component)]
struct CombatLogText;

fn setup_class_selection_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Root node
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::DARK_GRAY.into(),
                ..default()
            },
            ClassSelectionUI,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn(TextBundle::from_section(
                "Choisissez votre classe",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 50.0,
                    color: Color::WHITE,
                },
            ));

            // Class selection buttons/images
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        width: Val::Percent(80.0),
                        height: Val::Percent(50.0),
                        margin: UiRect::top(Val::Px(50.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // Warrior
                    spawn_class_button(
                        parent,
                        &asset_server,
                        player::PlayerClass::Warrior,
                        "Guerrier",
                        "1",
                    );
                    // Mage
                    spawn_class_button(
                        parent,
                        &asset_server,
                        player::PlayerClass::Mage,
                        "Magicien",
                        "2",
                    );
                    // Assassin
                    spawn_class_button(
                        parent,
                        &asset_server,
                        player::PlayerClass::Assassin,
                        "Assassin",
                        "3",
                    );
                    // Executioner
                    spawn_class_button(
                        parent,
                        &asset_server,
                        player::PlayerClass::Executioner,
                        "Bourreau",
                        "4",
                    );
                });
        });
}

fn spawn_class_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    class_type: player::PlayerClass,
    name: &str,
    key_hint: &str,
) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(150.0),
                    height: Val::Px(200.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            },
            ClassButton(class_type),
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(100.0),
                    height: Val::Px(100.0),
                    ..default()
                },
                image: UiImage::new(asset_server.load(&format!("images/Classe/{}.jpg", name))),
                ..default()
            });
            parent.spawn(TextBundle::from_section(
                name,
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ));
            parent.spawn(TextBundle::from_section(
                format!("({})", key_hint),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 15.0,
                    color: Color::GRAY,
                },
            ));
        });
}

fn handle_class_selection(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut interaction_query: Query<
        (&Interaction, &ClassButton),
        (Changed<Interaction>, With<Button>),
    >,
) {
    let mut selected_class: Option<player::PlayerClass> = None;

    // Keyboard input
    if keyboard_input.just_pressed(KeyCode::Key1) {
        selected_class = Some(player::PlayerClass::Warrior);
    } else if keyboard_input.just_pressed(KeyCode::Key2) {
        selected_class = Some(player::PlayerClass::Mage);
    } else if keyboard_input.just_pressed(KeyCode::Key3) {
        selected_class = Some(player::PlayerClass::Assassin);
    } else if keyboard_input.just_pressed(KeyCode::Key4) {
        selected_class = Some(player::PlayerClass::Executioner);
    }

    // Button interaction
    for (interaction, class_button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            selected_class = Some(class_button.0);
            break;
        }
    }

    if let Some(class) = selected_class {
        println!("Selected class: {:?}", class);
        commands.insert_resource(SelectedClass(class));
        next_state.set(GameState::Map);
    }
}

fn cleanup_class_selection_ui(
    mut commands: Commands,
    ui_query: Query<Entity, With<ClassSelectionUI>>,
) {
    for entity in ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// Marker component for map entities
#[derive(Component)]
struct MapTile;

fn spawn_player(
    mut commands: Commands,
    selected_class: Res<SelectedClass>,
    game_data: Res<map::GameData>,
    image_assets: Res<ImageAssets>,
) {
    let game_map = game_data.get_current_map();
    let player_class = selected_class.0;
    let player_data = player::Player::new(player_class, game_map.player_start);
    let player_texture = player_data.class.get_image_handle(&image_assets);
    let player_translation = Vec2::new(
        game_map.player_start.x as f32 * map::TILE_SIZE
            - (game_map.width as f32 * map::TILE_SIZE / 2.0),
        game_map.player_start.y as f32 * map::TILE_SIZE
            - (game_map.height as f32 * map::TILE_SIZE / 2.0),
    );

    commands.spawn((
        SpriteBundle {
            texture: player_texture,
            transform: Transform::from_translation(player_translation.extend(1.0)), // Z-order 1.0 for player
            sprite: Sprite {
                custom_size: Some(Vec2::new(map::TILE_SIZE, map::TILE_SIZE)),
                ..default()
            },
            ..default()
        },
        player_data,
        game_map.player_start,
    ));

    commands.remove_resource::<SelectedClass>();
}

fn spawn_map(
    mut commands: Commands,
    game_data: Res<map::GameData>,
    image_assets: Res<ImageAssets>,
    collected_items: Res<CollectedItems>,
    defeated_enemies: Res<DefeatedEnemies>,
) {
    let game_map = game_data.get_current_map();
    // Spawn map tiles
    for y in 0..game_map.height {
        for x in 0..game_map.width {
            let tile_type = game_map.grid[y][x];
            let texture = match tile_type {
                map::Tile::Path => image_assets.path_tile.clone(),
                map::Tile::Wall => image_assets.wall_tile.clone(),
                map::Tile::Connection => image_assets.path_tile.clone(), // Placeholder for connection tile
            };

            let position = Vec2::new(
                x as f32 * map::TILE_SIZE - (game_map.width as f32 * map::TILE_SIZE / 2.0),
                y as f32 * map::TILE_SIZE - (game_map.height as f32 * map::TILE_SIZE / 2.0),
            );

            commands.spawn((
                SpriteBundle {
                    texture,
                    transform: Transform::from_translation(position.extend(0.0)),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(map::TILE_SIZE, map::TILE_SIZE)),
                        ..default()
                    },
                    ..default()
                },
                MapTile,
                entity::Position { x, y }, // Logical position
            ));
        }
    }

    // Spawn items seulement s'ils n'ont pas été collectés
    for (pos, item_type) in &game_map.items {
        // Vérifier si l'objet a déjà été collecté
        if collected_items
            .items
            .contains(&(game_data.current_map_index, *pos))
        {
            continue; // Ne pas spawner cet objet
        }

        let item_texture = item_type.get_image_handle(&image_assets);
        let item_translation = Vec2::new(
            pos.x as f32 * map::TILE_SIZE - (game_map.width as f32 * map::TILE_SIZE / 2.0),
            pos.y as f32 * map::TILE_SIZE - (game_map.height as f32 * map::TILE_SIZE / 2.0),
        );

        commands.spawn((
            SpriteBundle {
                texture: item_texture,
                transform: Transform::from_translation(item_translation.extend(0.5)), // Z-order 0.5 for items
                sprite: Sprite {
                    custom_size: Some(Vec2::new(map::TILE_SIZE, map::TILE_SIZE)),
                    ..default()
                },
                ..default()
            },
            item::Item {
                item_type: *item_type,
            },
            *pos,
        ));
    }

    // Spawn ennemis seulement s'ils n'ont pas été vaincus
    for (pos, enemy_type) in &game_map.enemies {
        // Vérifier si l'ennemi a déjà été vaincu
        if defeated_enemies
            .enemies
            .contains(&(game_data.current_map_index, *pos))
        {
            continue; // Ne pas spawner cet ennemi
        }

        let enemy_texture = enemy_type.get_image_handle(&image_assets);
        let enemy_translation = Vec2::new(
            pos.x as f32 * map::TILE_SIZE - (game_map.width as f32 * map::TILE_SIZE / 2.0),
            pos.y as f32 * map::TILE_SIZE - (game_map.height as f32 * map::TILE_SIZE / 2.0),
        );

        commands.spawn((
            SpriteBundle {
                texture: enemy_texture,
                transform: Transform::from_translation(enemy_translation.extend(0.7)), // Z-order 0.7 for enemies
                sprite: Sprite {
                    custom_size: Some(Vec2::new(map::TILE_SIZE, map::TILE_SIZE)),
                    ..default()
                },
                ..default()
            },
            enemy::Enemy::new(*enemy_type),
            *pos,
        ));
    }
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut entity::Position, With<player::Player>>,
    mut game_data: ResMut<map::GameData>,
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

fn update_player_transform(
    game_data: Res<map::GameData>,
    mut player_query: Query<(&entity::Position, &mut Transform), With<player::Player>>,
) {
    if let Ok((player_pos, mut player_transform)) = player_query.get_single_mut() {
        let game_map = game_data.get_current_map();
        let translation = Vec2::new(
            player_pos.x as f32 * map::TILE_SIZE - (game_map.width as f32 * map::TILE_SIZE / 2.0),
            player_pos.y as f32 * map::TILE_SIZE - (game_map.height as f32 * map::TILE_SIZE / 2.0),
        );
        player_transform.translation = translation.extend(1.0); // Ensure Z-order remains 1.0
    }
}

fn check_for_item_pickup(
    mut commands: Commands,
    mut player_query: Query<(&entity::Position, &mut player::Player)>,
    item_query: Query<(Entity, &entity::Position, &item::Item)>,
    mut collected_items: ResMut<CollectedItems>,
    game_data: Res<map::GameData>,
    mut game_log: ResMut<GameLog>,
) {
    if let Ok((player_pos, mut player_data)) = player_query.get_single_mut() {
        for (item_entity, item_pos, item_data) in item_query.iter() {
            if player_pos == item_pos {
                let item_stats = item::get_stats_for_item(item_data.item_type);

                // Ajouter un message au log
                let item_name = match item_data.item_type {
                    item::ItemType::Armure => "Armure",
                    item::ItemType::Katana => "Katana",
                    item::ItemType::Gants => "Gants",
                    item::ItemType::Pendentif => "Pendentif",
                };
                game_log.add_message(format!("Objet ramassé : {}", item_name));

                // Construire le message de stats
                let mut stat_messages = Vec::new();
                if item_stats.hp > 0 {
                    stat_messages.push(format!("HP +{}", item_stats.hp));
                }
                if item_stats.attack > 0 {
                    stat_messages.push(format!("ATK +{}", item_stats.attack));
                }
                if item_stats.speed > 0 {
                    stat_messages.push(format!("SPD +{}", item_stats.speed));
                }
                if item_stats.critical_chance > 0 {
                    stat_messages.push(format!("CRIT +{}%", item_stats.critical_chance));
                }
                if !stat_messages.is_empty() {
                    game_log.add_message(stat_messages.join(", "));
                }

                player_data.apply_item_stats(&item_stats);

                // Marquer l'objet comme collecté
                collected_items
                    .items
                    .insert((game_data.current_map_index, *item_pos));

                commands.entity(item_entity).despawn();
            }
        }
    }
}

fn check_for_enemy_encounter(
    mut commands: Commands,
    player_query: Query<(&entity::Position, &player::Player)>,
    enemy_query: Query<(Entity, &entity::Position, &enemy::Enemy)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut game_log: ResMut<GameLog>,
) {
    if let Ok((player_pos, player_data)) = player_query.get_single() {
        for (enemy_entity, enemy_pos, enemy_data) in enemy_query.iter() {
            if player_pos == enemy_pos {
                // Collision détectée ! Déclencher le combat
                game_log.add_message(format!(
                    "Combat contre {} !",
                    enemy_data.enemy_type.get_name()
                ));

                // Sauvegarder l'ennemi en combat
                commands.insert_resource(CurrentEnemy {
                    entity: enemy_entity,
                    position: *enemy_pos,
                    enemy_type: enemy_data.enemy_type,
                    hp: enemy_data.stats.hp,
                    stats: enemy_data.stats,
                });

                // Initialiser l'état du combat
                commands.insert_resource(CombatState {
                    player_hp: player_data.stats.hp,
                    enemy_hp: enemy_data.stats.hp,
                    combat_log: Vec::new(),
                    is_player_turn: true,
                });

                // Passer en mode combat
                next_state.set(GameState::Combat);
                break;
            }
        }
    }
}

fn despawn_map(
    mut commands: Commands,
    map_tile_query: Query<Entity, With<MapTile>>,
    item_query: Query<Entity, With<item::Item>>,
    enemy_query: Query<Entity, With<enemy::Enemy>>,
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

fn map_transition(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Map);
}

fn setup_info_terminal(
    mut commands: Commands,
    player_query: Query<&player::Player>,
    mut game_log: ResMut<GameLog>,
) {
    let player = player_query.single();

    // Ajouter un message de bienvenue
    game_log.add_message("=== Début de l'aventure ===".to_string());

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Px(0.0),
                    top: Val::Px(0.0),
                    width: Val::Px(280.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::rgba(0.15, 0.15, 0.15, 0.95).into(),
                z_index: ZIndex::Global(1000),
                ..default()
            },
            InfoTerminal,
        ))
        .with_children(|parent| {
            // Titre
            parent.spawn(
                TextBundle::from_section(
                    "Informations",
                    TextStyle {
                        font_size: 24.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                }),
            );

            // Section Stats du joueur
            parent.spawn(
                TextBundle::from_section(
                    "=== Statistiques ===",
                    TextStyle {
                        font_size: 18.0,
                        color: Color::rgb(0.8, 0.8, 0.2),
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::bottom(Val::Px(5.0)),
                    ..default()
                }),
            );

            let class_name = match player.class {
                player::PlayerClass::Warrior => "Guerrier",
                player::PlayerClass::Mage => "Magicien",
                player::PlayerClass::Assassin => "Assassin",
                player::PlayerClass::Executioner => "Bourreau",
            };

            let stats_text = format!(
                "Classe: {}\nHP: {}\nAttaque: {}\nVitesse: {}\nCritique: {}%",
                class_name,
                player.stats.hp,
                player.stats.attack,
                player.stats.speed,
                player.stats.critical_chance
            );

            parent.spawn((
                TextBundle::from_section(
                    stats_text,
                    TextStyle {
                        font_size: 16.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::bottom(Val::Px(15.0)),
                    ..default()
                }),
                StatsText,
            ));

            // Section Log
            parent.spawn(
                TextBundle::from_section(
                    "=== Événements ===",
                    TextStyle {
                        font_size: 18.0,
                        color: Color::rgb(0.8, 0.8, 0.2),
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::bottom(Val::Px(5.0)),
                    ..default()
                }),
            );

            parent.spawn((
                TextBundle::from_section(
                    "Bienvenue dans le jeu !",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::rgb(0.7, 0.7, 0.7),
                        ..default()
                    },
                ),
                LogText,
            ));
        });
}

fn cleanup_info_terminal(
    mut commands: Commands,
    terminal_query: Query<Entity, With<InfoTerminal>>,
) {
    for entity in terminal_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn update_info_terminal(
    player_query: Query<&player::Player>,
    mut stats_text_query: Query<&mut Text, (With<StatsText>, Without<LogText>)>,
    mut log_text_query: Query<&mut Text, (With<LogText>, Without<StatsText>)>,
    game_log: Res<GameLog>,
) {
    // Mettre à jour les stats du joueur
    if let Ok(player) = player_query.get_single() {
        if let Ok(mut text) = stats_text_query.get_single_mut() {
            let class_name = match player.class {
                player::PlayerClass::Warrior => "Guerrier",
                player::PlayerClass::Mage => "Magicien",
                player::PlayerClass::Assassin => "Assassin",
                player::PlayerClass::Executioner => "Bourreau",
            };

            let new_stats = format!(
                "Classe: {}\nHP: {}\nAttaque: {}\nVitesse: {}\nCritique: {}%",
                class_name,
                player.stats.hp,
                player.stats.attack,
                player.stats.speed,
                player.stats.critical_chance
            );

            // Ne mettre à jour que si le texte a changé pour éviter les re-renders inutiles
            if text.sections[0].value != new_stats {
                text.sections[0].value = new_stats;
            }
        }
    }

    // Mettre à jour le log
    if let Ok(mut text) = log_text_query.get_single_mut() {
        let new_log = if game_log.messages.is_empty() {
            "Bienvenue dans le jeu !".to_string()
        } else {
            game_log.messages.join("\n")
        };

        // Ne mettre à jour que si le texte a changé
        if text.sections[0].value != new_log {
            text.sections[0].value = new_log;
        }
    }
}

// ============ SYSTÈME DE COMBAT ============

fn setup_combat(
    mut commands: Commands,
    player_query: Query<&player::Player>,
    current_enemy: Res<CurrentEnemy>,
    mut combat_state: ResMut<CombatState>,
) {
    let player = player_query.single();

    // Créer l'UI de combat
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.9).into(),
                z_index: ZIndex::Global(2000),
                ..default()
            },
            CombatUI,
        ))
        .with_children(|parent| {
            // Titre
            parent.spawn(
                TextBundle::from_section(
                    "COMBAT !",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(1.0, 0.2, 0.2),
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                }),
            );

            // Stats du joueur
            let class_name = match player.class {
                player::PlayerClass::Warrior => "Guerrier",
                player::PlayerClass::Mage => "Magicien",
                player::PlayerClass::Assassin => "Assassin",
                player::PlayerClass::Executioner => "Bourreau",
            };

            parent.spawn(
                TextBundle::from_section(
                    format!(
                        "VOUS ({})\nHP: {} | ATK: {} | SPD: {} | CRIT: {}%",
                        class_name,
                        combat_state.player_hp,
                        player.stats.attack,
                        player.stats.speed,
                        player.stats.critical_chance
                    ),
                    TextStyle {
                        font_size: 24.0,
                        color: Color::rgb(0.2, 1.0, 0.2),
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                }),
            );

            // Stats de l'ennemi
            parent.spawn(
                TextBundle::from_section(
                    format!(
                        "{}\nHP: {} | ATK: {} | SPD: {} | CRIT: {}%",
                        current_enemy.enemy_type.get_name(),
                        combat_state.enemy_hp,
                        current_enemy.stats.attack,
                        current_enemy.stats.speed,
                        current_enemy.stats.critical_chance
                    ),
                    TextStyle {
                        font_size: 24.0,
                        color: Color::rgb(1.0, 0.2, 0.2),
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::bottom(Val::Px(30.0)),
                    ..default()
                }),
            );

            // Log de combat
            parent.spawn((
                TextBundle::from_section(
                    "Appuyez sur ESPACE pour attaquer !",
                    TextStyle {
                        font_size: 18.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                CombatLogText,
            ));
        });
}

fn handle_combat(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut player::Player>,
    current_enemy: Res<CurrentEnemy>,
    mut combat_state: ResMut<CombatState>,
    mut next_state: ResMut<NextState<GameState>>,
    mut game_log: ResMut<GameLog>,
    mut combat_log_text_query: Query<&mut Text, With<CombatLogText>>,
    mut defeated_enemies: ResMut<DefeatedEnemies>,
    game_data: Res<map::GameData>,
) {
    let mut player = player_query.single_mut();

    if keyboard_input.just_pressed(KeyCode::Space) && combat_state.is_player_turn {
        // Tour du joueur
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let mut log_messages = Vec::new();
        log_messages.push("--- Votre tour ---".to_string());

        // Vérifier esquive de l'ennemi
        let dodge_roll = rng.gen_range(1..=100);
        if dodge_roll <= current_enemy.stats.speed {
            log_messages.push("L'ennemi esquive !".to_string());
        } else {
            // Calculer les dégâts
            let mut damage = player.stats.attack;
            let crit_roll = rng.gen_range(1..=100);
            if crit_roll <= player.stats.critical_chance {
                damage *= 2;
                log_messages.push(format!("COUP CRITIQUE ! Degats: {}", damage));
            } else {
                log_messages.push(format!("Vous attaquez ! Degats: {}", damage));
            }

            combat_state.enemy_hp -= damage;
        }

        log_messages.push(format!("HP ennemi: {}", combat_state.enemy_hp.max(0)));

        // Vérifier si l'ennemi est vaincu
        if combat_state.enemy_hp <= 0 {
            log_messages.push("".to_string());
            log_messages.push("VICTOIRE !".to_string());
            log_messages.push("Choisissez une stat a ameliorer:".to_string());
            log_messages.push("H=HP | A=ATK | S=SPD | C=CRIT".to_string());

            combat_state.is_player_turn = false; // Attendre le choix du joueur
        } else {
            // Tour de l'ennemi
            log_messages.push("".to_string());
            log_messages.push("--- Tour ennemi ---".to_string());

            let dodge_roll = rng.gen_range(1..=100);
            if dodge_roll <= player.stats.speed {
                log_messages.push("Vous esquivez !".to_string());
            } else {
                let mut damage = current_enemy.stats.attack;
                let crit_roll = rng.gen_range(1..=100);
                if crit_roll <= current_enemy.stats.critical_chance {
                    damage *= 2;
                    log_messages.push(format!("COUP CRITIQUE ENNEMI ! Degats: {}", damage));
                } else {
                    log_messages.push(format!("Ennemi attaque ! Degats: {}", damage));
                }

                combat_state.player_hp -= damage;
            }

            log_messages.push(format!("Vos HP: {}", combat_state.player_hp.max(0)));

            // Vérifier si le joueur est vaincu
            if combat_state.player_hp <= 0 {
                log_messages.push("".to_string());
                log_messages.push("DEFAITE...".to_string());
                log_messages.push("Appuyez sur R pour recommencer".to_string());
                combat_state.is_player_turn = false;
            } else {
                log_messages.push("".to_string());
                log_messages.push("ESPACE pour continuer".to_string());
            }
        }

        // Mettre à jour le texte du log de combat
        if let Ok(mut text) = combat_log_text_query.get_single_mut() {
            text.sections[0].value = log_messages.join("\n");
        }
    }

    // Gestion de la victoire - choix de stat
    if combat_state.enemy_hp <= 0 && !combat_state.is_player_turn {
        let mut stat_chosen = false;

        if keyboard_input.just_pressed(KeyCode::H) {
            player.stats.hp += 10;
            game_log.add_message("HP +10 !".to_string());
            stat_chosen = true;
        } else if keyboard_input.just_pressed(KeyCode::A) {
            player.stats.attack += 2;
            game_log.add_message("ATK +2 !".to_string());
            stat_chosen = true;
        } else if keyboard_input.just_pressed(KeyCode::S) {
            player.stats.speed += 1;
            game_log.add_message("SPD +1 !".to_string());
            stat_chosen = true;
        } else if keyboard_input.just_pressed(KeyCode::C) {
            player.stats.critical_chance += 2;
            game_log.add_message("CRIT +2% !".to_string());
            stat_chosen = true;
        }

        if stat_chosen {
            // Marquer l'ennemi comme vaincu pour ne pas le respawn
            defeated_enemies
                .enemies
                .insert((game_data.current_map_index, current_enemy.position));

            game_log.add_message(format!("{} vaincu !", current_enemy.enemy_type.get_name()));

            // Retourner à la carte
            next_state.set(GameState::Map);
        }
    }

    // Gestion de la défaite
    if combat_state.player_hp <= 0 && keyboard_input.just_pressed(KeyCode::R) {
        // TODO: Implémenter le redémarrage du jeu
        next_state.set(GameState::ClassSelection);
    }
}

fn cleanup_combat(mut commands: Commands, combat_ui_query: Query<Entity, With<CombatUI>>) {
    for entity in combat_ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Nettoyer les ressources de combat
    commands.remove_resource::<CurrentEnemy>();
    commands.remove_resource::<CombatState>();
}
