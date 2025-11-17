use bevy::prelude::*;

// Resource to hold image handles
#[derive(Resource)]
pub struct ImageAssets {
    // Map tiles
    pub path_tile: Handle<Image>,
    pub wall_tile: Handle<Image>,

    // Classes
    pub warrior_class: Handle<Image>,
    pub mage_class: Handle<Image>,
    pub assassin_class: Handle<Image>,
    pub executioner_class: Handle<Image>,

    // Items
    pub armor_item: Handle<Image>,
    pub katana_item: Handle<Image>,
    pub gloves_item: Handle<Image>,
    pub pendant_item: Handle<Image>,

    // Enemies
    pub small_goblin: Handle<Image>,
    pub medium_goblin: Handle<Image>,
    pub large_goblin: Handle<Image>,
    pub wolf: Handle<Image>,
    pub snake: Handle<Image>,
    pub drake: Handle<Image>,
    pub death_bird: Handle<Image>,
}

pub fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ImageAssets {
        path_tile: asset_server.load("images/Chemin.jpg"),
        wall_tile: asset_server.load("images/Muraille.jpg"),
        warrior_class: asset_server.load("images/Classe/Guerrier.jpg"),
        mage_class: asset_server.load("images/Classe/Magicien.jpg"),
        assassin_class: asset_server.load("images/Classe/Assassin.jpg"),
        executioner_class: asset_server.load("images/Classe/Bourreau.jpg"),
        armor_item: asset_server.load("images/Objets/Armure.jpg"),
        katana_item: asset_server.load("images/Objets/Katana.jpg"),
        gloves_item: asset_server.load("images/Objets/Gants.jpg"),
        pendant_item: asset_server.load("images/Objets/Pendentif.jpg"),
        small_goblin: asset_server.load("images/Enemies/Petit_Gobelin.jpg"),
        medium_goblin: asset_server.load("images/Enemies/Moyen_Gobelin.jpg"),
        large_goblin: asset_server.load("images/Enemies/Gros_Gobelin.jpg"),
        wolf: asset_server.load("images/Enemies/Loup.jpg"),
        snake: asset_server.load("images/Enemies/Serpent.jpg"),
        drake: asset_server.load("images/Enemies/Soldat_Draconide.jpg"),
        death_bird: asset_server.load("images/Enemies/Oiseau_Funeraire.jpg"),
    });
}
