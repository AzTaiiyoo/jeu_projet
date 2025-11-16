use crate::player::PlayerClass;
use bevy::prelude::Resource;

/// Resource pour stocker la classe de joueur sélectionnée
#[derive(Resource)]
pub struct SelectedClass(pub PlayerClass);
