use crate::components::{ClassButton, ClassSelectionUI};
use crate::player::PlayerClass;
use bevy::prelude::*;

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                    spawn_class_button(
                        parent,
                        &asset_server,
                        PlayerClass::Warrior,
                        "Guerrier",
                        "1",
                    );
                    spawn_class_button(parent, &asset_server, PlayerClass::Mage, "Magicien", "2");
                    spawn_class_button(
                        parent,
                        &asset_server,
                        PlayerClass::Assassin,
                        "Assassin",
                        "3",
                    );
                    spawn_class_button(
                        parent,
                        &asset_server,
                        PlayerClass::Executioner,
                        "Bourreau",
                        "4",
                    );
                });
        });
}

fn spawn_class_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    class_type: PlayerClass,
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
