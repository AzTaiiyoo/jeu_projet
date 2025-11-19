use crate::components::{InfoTerminal, LogText, StatsText};
use crate::config::TERMINAL_WIDTH;
use crate::player::{Player, PlayerClass};
use crate::resources::GameLog;
use bevy::prelude::*;

/// Crée l'UI du terminal d'information sur le côté droit de l'écran
/// Affiche les statistiques du joueur et les événements récents
pub fn setup_info_terminal(
    mut commands: Commands,
    player_query: Query<&Player>,
    mut game_log: ResMut<GameLog>,
) {
    let player = player_query.single();

    game_log.add_message("=== Début de l'aventure ===".to_string());

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Px(0.0),
                    top: Val::Px(0.0),
                    width: Val::Px(TERMINAL_WIDTH),
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

            // Section Stats
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

            let class_name = get_class_name(player.class);
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

/// Met à jour en temps réel le contenu du terminal d'information
/// Synchronise les stats du joueur et le log de jeu
pub fn update_info_terminal(
    player_query: Query<&Player>,
    mut stats_text_query: Query<&mut Text, (With<StatsText>, Without<LogText>)>,
    mut log_text_query: Query<&mut Text, (With<LogText>, Without<StatsText>)>,
    game_log: Res<GameLog>,
) {
    // Mettre à jour les stats du joueur
    if let Ok(player) = player_query.get_single() {
        if let Ok(mut text) = stats_text_query.get_single_mut() {
            let class_name = get_class_name(player.class);
            let new_stats = format!(
                "Classe: {}\nHP: {}\nAttaque: {}\nVitesse: {}\nCritique: {}%",
                class_name,
                player.stats.hp,
                player.stats.attack,
                player.stats.speed,
                player.stats.critical_chance
            );

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

        if text.sections[0].value != new_log {
            text.sections[0].value = new_log;
        }
    }
}

pub fn cleanup_info_terminal(
    mut commands: Commands,
    terminal_query: Query<Entity, With<InfoTerminal>>,
) {
    for entity in terminal_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn get_class_name(class: PlayerClass) -> &'static str {
    match class {
        PlayerClass::Warrior => "Guerrier",
        PlayerClass::Mage => "Magicien",
        PlayerClass::Assassin => "Assassin",
        PlayerClass::Executioner => "Bourreau",
    }
}
