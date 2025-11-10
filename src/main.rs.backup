use bevy::{prelude::*, window::PresentMode};

mod combat;
mod entity;
mod map;
mod player;
mod item;
mod assets;

use crate::assets::ImageAssets;

// Game states
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    ClassSelection,
    Map,
    Combat,
}

// Resource to hold the selected player class
#[derive(Resource)]
struct SelectedClass(player::PlayerClass);

fn main() {
    let game_map = map::GameMap::new();
    let window_width = game_map.width as f32 * map::TILE_SIZE;
    let window_height = game_map.height as f32 * map::TILE_SIZE;

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Jeu de l'Aventure".into(),
                resolution: (window_width, window_height).into(),
                present_mode: PresentMode::AutoVsync,
                resizable: false, // Optional: make window non-resizable
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(game_map)
        .add_state::<GameState>()
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(GameState::ClassSelection), (assets::load_assets, setup_class_selection_ui))
        .add_systems(Update, handle_class_selection.run_if(in_state(GameState::ClassSelection)))
        .add_systems(OnExit(GameState::ClassSelection), cleanup_class_selection_ui)
        .add_systems(OnEnter(GameState::Map), spawn_map) // Add spawn_map system
        .add_systems(Update, (move_player, update_player_transform, check_for_item_pickup).run_if(in_state(GameState::Map))) // Add player movement and item pickup systems
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// UI Marker component
#[derive(Component)]
struct ClassSelectionUI;

// Component to hold PlayerClass for buttons
#[derive(Component)]
struct ClassButton(player::PlayerClass);

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
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"), // Assuming a font exists
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
                    spawn_class_button(parent, &asset_server, player::PlayerClass::Warrior, "Guerrier", "1");
                    // Mage
                    spawn_class_button(parent, &asset_server, player::PlayerClass::Mage, "Magicien", "2");
                    // Assassin
                    spawn_class_button(parent, &asset_server, player::PlayerClass::Assassin, "Assassin", "3");
                    // Executioner
                    spawn_class_button(parent, &asset_server, player::PlayerClass::Executioner, "Bourreau", "4");
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
            ClassButton(class_type), // Add ClassButton as a component to the button
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

fn cleanup_class_selection_ui(mut commands: Commands, ui_query: Query<Entity, With<ClassSelectionUI>>) {
    for entity in ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// Marker component for map entities
#[derive(Component)]
struct MapTile;

fn spawn_map(
    mut commands: Commands,
    game_map: Res<map::GameMap>,
    image_assets: Res<ImageAssets>,
    selected_class: Res<SelectedClass>,
) {
    // Spawn map tiles
    for y in 0..game_map.height {
        for x in 0..game_map.width {
            let tile_type = game_map.grid[y][x];
            let texture = match tile_type {
                map::Tile::Path => image_assets.path_tile.clone(),
                map::Tile::Wall => image_assets.wall_tile.clone(),
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

    // Spawn player
    let player_class = selected_class.0;
    let player_data = player::Player::new(player_class, game_map.player_start);
    let player_texture = player_data.class.get_image_handle(&image_assets);
    let player_translation = Vec2::new(
        game_map.player_start.x as f32 * map::TILE_SIZE - (game_map.width as f32 * map::TILE_SIZE / 2.0),
        game_map.player_start.y as f32 * map::TILE_SIZE - (game_map.height as f32 * map::TILE_SIZE / 2.0),
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

    // Spawn items
    let items_to_place = vec![
        (entity::Position { x: 2, y: 3 }, item::ItemType::Katana),
        (entity::Position { x: 5, y: 5 }, item::ItemType::Armure),
        (entity::Position { x: 7, y: 1 }, item::ItemType::Gants),
        (entity::Position { x: 8, y: 4 }, item::ItemType::Pendentif),
    ];

    for (pos, item_type) in items_to_place {
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
            item::Item { item_type },
            pos,
        ));
    }

    commands.remove_resource::<SelectedClass>();
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut entity::Position, With<player::Player>>,
    game_map: Res<map::GameMap>,
) {
    let mut player_pos = player_query.single_mut();
    let mut new_pos = player_pos.clone();

    if keyboard_input.just_pressed(KeyCode::Z) || keyboard_input.just_pressed(KeyCode::Up) {
        new_pos.y = new_pos.y.saturating_sub(1);
    } else if keyboard_input.just_pressed(KeyCode::S) || keyboard_input.just_pressed(KeyCode::Down) {
        new_pos.y = (new_pos.y + 1).min(game_map.height - 1);
    } else if keyboard_input.just_pressed(KeyCode::Q) || keyboard_input.just_pressed(KeyCode::Left) {
        new_pos.x = new_pos.x.saturating_sub(1);
    } else if keyboard_input.just_pressed(KeyCode::D) || keyboard_input.just_pressed(KeyCode::Right) {
        new_pos.x = (new_pos.x + 1).min(game_map.width - 1);
    }

    if game_map.is_walkable(new_pos.x, new_pos.y) {
        *player_pos = new_pos;
    }
}

fn update_player_transform(
    game_map: Res<map::GameMap>,
    mut player_query: Query<(&entity::Position, &mut Transform), With<player::Player>>,
) {
    if let Ok((player_pos, mut player_transform)) = player_query.get_single_mut() {
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
) {
    if let Ok((player_pos, mut player_data)) = player_query.get_single_mut() {
        for (item_entity, item_pos, item_data) in item_query.iter() {
            if player_pos == item_pos {
                let item_stats = item::get_stats_for_item(item_data.item_type);
                player_data.apply_item_stats(&item_stats);
                println!("Picked up item: {:?}", item_data.item_type);
                commands.entity(item_entity).despawn();
            }
        }
    }
}