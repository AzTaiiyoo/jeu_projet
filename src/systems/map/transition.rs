use bevy::prelude::*;
use crate::states::GameState;

pub fn map_transition(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Map);
}
