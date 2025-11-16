use bevy::prelude::Resource;

/// Resource pour stocker les messages de jeu affichés dans le terminal d'information
/// Conserve un historique des 10 derniers messages pour éviter de surcharger l'UI
#[derive(Resource, Default)]
pub struct GameLog {
    pub messages: Vec<String>,
}

impl GameLog {
    /// Ajoute un message au log de jeu
    /// Limite automatiquement à 10 messages en supprimant les plus anciens
    pub fn add_message(&mut self, message: String) {
        println!("Ajout message au log: {}", message);
        self.messages.push(message);
        // Garder seulement les 10 derniers messages
        if self.messages.len() > 10 {
            self.messages.remove(0);
        }
        println!("Total messages dans log: {}", self.messages.len());
    }
}
