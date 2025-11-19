use crate::components::*;
use crate::player::Player;
use crate::resources::{CombatState, CurrentEnemy};
use bevy::prelude::*;

/// Configure l'interface de combat avec un layout 3 zones :
/// - Gauche (30%) : Stats + barre de vie du joueur
/// - Centre (40%) : Messages de combat et actions
/// - Droite (30%) : Stats + barre de vie de l'ennemi
pub fn setup_combat(
    mut commands: Commands,
    player_query: Query<&Player>,
    current_enemy: Res<CurrentEnemy>,
    combat_state: Res<CombatState>,
) {
    let player = player_query.single();

    let class_name = match player.class {
        crate::player::PlayerClass::Warrior => "Guerrier",
        crate::player::PlayerClass::Mage => "Magicien",
        crate::player::PlayerClass::Assassin => "Assassin",
        crate::player::PlayerClass::Executioner => "Bourreau",
    };

    // Root container fullscreen
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.95).into(),
                z_index: ZIndex::Global(2000),
                ..default()
            },
            CombatUI,
        ))
        .with_children(|parent| {
            // ===== PANNEAU GAUCHE : JOUEUR =====
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(30.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Start,
                        padding: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                    background_color: Color::rgba(0.0, 0.2, 0.0, 0.3).into(),
                    ..default()
                })
                .with_children(|player_panel| {
                    // Titre joueur
                    player_panel.spawn(
                        TextBundle::from_section(
                            format!("VOUS ({})", class_name),
                            TextStyle {
                                font_size: 28.0,
                                color: Color::rgb(0.2, 1.0, 0.2),
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::bottom(Val::Px(15.0)),
                            ..default()
                        }),
                    );

                    // Barre de vie joueur - Container
                    player_panel
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Px(30.0),
                                margin: UiRect::bottom(Val::Px(10.0)),
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            background_color: Color::rgba(0.2, 0.2, 0.2, 0.8).into(),
                            border_color: Color::rgb(0.2, 1.0, 0.2).into(),
                            ..default()
                        })
                        .with_children(|hp_container| {
                            // Barre de vie remplie (calculée dynamiquement)
                            let hp_percent =
                                (combat_state.player_hp as f32 / player.stats.hp as f32 * 100.0)
                                    .max(0.0);
                            hp_container.spawn((
                                NodeBundle {
                                    style: Style {
                                        width: Val::Percent(hp_percent),
                                        height: Val::Percent(100.0),
                                        ..default()
                                    },
                                    background_color: Color::rgb(0.2, 1.0, 0.2).into(),
                                    ..default()
                                },
                                HealthBar::Player,
                            ));
                        });

                    // Stats joueur (multi-lignes)
                    player_panel.spawn((
                        TextBundle::from_section(
                            format!(
                                "Points de vie: {} / {}\nAttaque: {}\nVitesse: {}\nCritique: {}%",
                                combat_state.player_hp,
                                player.stats.hp,
                                player.stats.attack,
                                player.stats.speed,
                                player.stats.critical_chance
                            ),
                            TextStyle {
                                font_size: 20.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        ),
                        PlayerStatsText,
                    ));
                });

            // ===== PANNEAU CENTRE : COMBAT =====
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(40.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|center_panel| {
                    // Titre COMBAT
                    center_panel.spawn(
                        TextBundle::from_section(
                            "⚔ COMBAT ⚔",
                            TextStyle {
                                font_size: 48.0,
                                color: Color::rgb(1.0, 0.2, 0.2),
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::bottom(Val::Px(40.0)),
                            ..default()
                        }),
                    );

                    // Message d'action (tour actuel + dernière action)
                    center_panel.spawn((
                        TextBundle::from_section(
                            "Appuyez sur ESPACE pour attaquer !",
                            TextStyle {
                                font_size: 24.0,
                                color: Color::rgb(1.0, 1.0, 0.2),
                                ..default()
                            },
                        )
                        .with_text_alignment(TextAlignment::Center)
                        .with_style(Style {
                            margin: UiRect::bottom(Val::Px(30.0)),
                            ..default()
                        }),
                        ActionMessageText,
                    ));

                    // Log de combat (historique compact)
                    center_panel.spawn((
                        TextBundle::from_section(
                            "",
                            TextStyle {
                                font_size: 16.0,
                                color: Color::rgb(0.8, 0.8, 0.8),
                                ..default()
                            },
                        )
                        .with_text_alignment(TextAlignment::Center),
                        CombatLogText,
                    ));
                });

            // ===== PANNEAU DROIT : ENNEMI =====
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(30.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::End,
                        padding: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                    background_color: Color::rgba(0.2, 0.0, 0.0, 0.3).into(),
                    ..default()
                })
                .with_children(|enemy_panel| {
                    // Titre ennemi
                    enemy_panel.spawn(
                        TextBundle::from_section(
                            current_enemy.enemy_type.get_name(),
                            TextStyle {
                                font_size: 28.0,
                                color: Color::rgb(1.0, 0.2, 0.2),
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::bottom(Val::Px(15.0)),
                            ..default()
                        }),
                    );

                    // Barre de vie ennemi - Container
                    enemy_panel
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Px(30.0),
                                margin: UiRect::bottom(Val::Px(10.0)),
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            background_color: Color::rgba(0.2, 0.2, 0.2, 0.8).into(),
                            border_color: Color::rgb(1.0, 0.2, 0.2).into(),
                            ..default()
                        })
                        .with_children(|hp_container| {
                            // Barre de vie remplie (calculée dynamiquement)
                            let hp_percent = (combat_state.enemy_hp as f32
                                / current_enemy.stats.hp as f32
                                * 100.0)
                                .max(0.0);
                            hp_container.spawn((
                                NodeBundle {
                                    style: Style {
                                        width: Val::Percent(hp_percent),
                                        height: Val::Percent(100.0),
                                        ..default()
                                    },
                                    background_color: Color::rgb(1.0, 0.2, 0.2).into(),
                                    ..default()
                                },
                                HealthBar::Enemy,
                            ));
                        });

                    // Stats ennemi (multi-lignes, alignées à droite)
                    enemy_panel.spawn((
                        TextBundle::from_section(
                            format!(
                                "Points de vie: {} / {}\nAttaque: {}\nVitesse: {}\nCritique: {}%",
                                combat_state.enemy_hp,
                                current_enemy.stats.hp,
                                current_enemy.stats.attack,
                                current_enemy.stats.speed,
                                current_enemy.stats.critical_chance
                            ),
                            TextStyle {
                                font_size: 20.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        )
                        .with_text_alignment(TextAlignment::Right),
                        EnemyStatsText,
                    ));
                });
        });
}
