use bevy::prelude::*;
use crate::components::Position;
use crate::item::{self, Item, ItemType};
use crate::player::Player;
use crate::resources::{CollectedItems, GameLog};
use crate::systems::map::GameData;

/// Vérifie si le joueur est sur la même case qu'un objet
/// Si oui :
/// - Applique les bonus de stats de l'objet au joueur
/// - Ajoute des messages au log de jeu
/// - Marque l'objet comme collecté dans CollectedItems
/// - Détruit l'entité de l'objet
pub fn check_item_pickup(
    mut commands: Commands,
    mut player_query: Query<(&Position, &mut Player)>,
    item_query: Query<(Entity, &Position, &Item)>,
    mut collected_items: ResMut<CollectedItems>,
    game_data: Res<GameData>,
    mut game_log: ResMut<GameLog>,
) {
    if let Ok((player_pos, mut player_data)) = player_query.get_single_mut() {
        for (item_entity, item_pos, item_data) in item_query.iter() {
            if player_pos == item_pos {
                let item_stats = item::get_stats_for_item(item_data.item_type);

                let item_name = match item_data.item_type {
                    ItemType::Armure => "Armure",
                    ItemType::Katana => "Katana",
                    ItemType::Gants => "Gants",
                    ItemType::Pendentif => "Pendentif",
                };
                game_log.add_message(format!("Objet ramassé : {}", item_name));

                let mut stat_messages = Vec::new();
                if item_stats.hp > 0 {
                    stat_messages.push(format!("HP +{}", item_stats.hp));
                }
                if item_stats.attack > 0 {
                    stat_messages.push(format!("ATK +{}", item_stats.attack));
                }
                if item_stats.speed > 0 {
                    stat_messages.push(format!("SPD +{}", item_stats.speed));
                }
                if item_stats.critical_chance > 0 {
                    stat_messages.push(format!("CRIT +{}%", item_stats.critical_chance));
                }
                if !stat_messages.is_empty() {
                    game_log.add_message(stat_messages.join(", "));
                }

                player_data.apply_item_stats(&item_stats);

                collected_items
                    .items
                    .insert((game_data.current_map_index, *item_pos));

                commands.entity(item_entity).despawn();
            }
        }
    }
}
