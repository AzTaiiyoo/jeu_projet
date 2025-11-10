use bevy::prelude::Component;
use crate::entity::Stats;

#[derive(Component, Debug)]
pub struct Enemy {
    pub name: &'static str,
}

pub fn get_stats_for_enemy(name: &'static str) -> Stats {
    match name {
        "Gobelin" => Stats {
            hp: 50,
            attack: 5,
            speed: 3,
            critical_chance: 5,
        },
        "Orc" => Stats {
            hp: 100,
            attack: 10,
            speed: 2,
            critical_chance: 10,
        },
        _ => Stats::default(),
    }
}
