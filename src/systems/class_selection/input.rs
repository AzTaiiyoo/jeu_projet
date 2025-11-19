use crate::components::ClassButton;
use crate::player::PlayerClass;
use crate::resources::SelectedClass;
use crate::states::GameState;
use bevy::prelude::*;

pub fn handle_input(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut interaction_query: Query<
        (&Interaction, &ClassButton),
        (Changed<Interaction>, With<Button>),
    >,
) {
    let mut selected_class: Option<PlayerClass> = None;

    // Keyboard input
    if keyboard_input.just_pressed(KeyCode::Key1) {
        selected_class = Some(PlayerClass::Warrior);
    } else if keyboard_input.just_pressed(KeyCode::Key2) {
        selected_class = Some(PlayerClass::Mage);
    } else if keyboard_input.just_pressed(KeyCode::Key3) {
        selected_class = Some(PlayerClass::Assassin);
    } else if keyboard_input.just_pressed(KeyCode::Key4) {
        selected_class = Some(PlayerClass::Executioner);
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
