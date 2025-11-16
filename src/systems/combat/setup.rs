use bevy::prelude::*;
use crate::components::{CombatLogText, CombatUI};
use crate::player::Player;
use crate::resources::{CombatState, CurrentEnemy};

/// Configure l'interface de combat avec un overlay fullscreen
/// Affiche les stats des deux combattants et les instructions
/// Appelé lors de l'entrée en mode Combat
pub fn setup_combat(
    mut commands: Commands,
    player_query: Query<&Player>,
    current_enemy: Res<CurrentEnemy>,
    combat_state: Res<CombatState>,
) {
    let player = player_query.single();

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

            let class_name = match player.class {
                crate::player::PlayerClass::Warrior => "Guerrier",
                crate::player::PlayerClass::Mage => "Magicien",
                crate::player::PlayerClass::Assassin => "Assassin",
                crate::player::PlayerClass::Executioner => "Bourreau",
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
