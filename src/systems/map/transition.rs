use crate::states::GameState;
use bevy::prelude::*;

pub fn map_transition(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Map);
}
