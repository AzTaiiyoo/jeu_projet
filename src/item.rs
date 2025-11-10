use crate::assets::ImageAssets;
use crate::entity::Stats;
use bevy::prelude::{Component, Handle, Image};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ItemType {
    Armure,
    Katana,
    Gants,
    Pendentif,
}

impl ItemType {
    pub fn get_image_handle(&self, image_assets: &ImageAssets) -> Handle<Image> {
        match self {
            ItemType::Armure => image_assets.armor_item.clone(),
            ItemType::Katana => image_assets.katana_item.clone(),
            ItemType::Gants => image_assets.gloves_item.clone(),
            ItemType::Pendentif => image_assets.pendant_item.clone(),
        }
    }
}

#[derive(Component, Debug)]
pub struct Item {
    pub item_type: ItemType,
}

pub fn get_stats_for_item(item_type: ItemType) -> Stats {
    match item_type {
        ItemType::Armure => Stats {
            hp: 50,
            attack: 0,
            speed: 0,
            critical_chance: 0,
        },
        ItemType::Katana => Stats {
            hp: 0,
            attack: 0,
            speed: 5,
            critical_chance: 0,
        },
        ItemType::Gants => Stats {
            hp: 0,
            attack: 10,
            speed: 0,
            critical_chance: 0,
        },
        ItemType::Pendentif => Stats {
            hp: 0,
            attack: 0,
            speed: 0,
            critical_chance: 10,
        },
    }
}
