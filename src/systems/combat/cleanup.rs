use crate::components::CombatUI;
use crate::resources::{CombatState, CurrentEnemy};
use bevy::prelude::*;

pub fn cleanup_combat(mut commands: Commands, combat_ui_query: Query<Entity, With<CombatUI>>) {
    for entity in combat_ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands.remove_resource::<CurrentEnemy>();
    commands.remove_resource::<CombatState>();
}
