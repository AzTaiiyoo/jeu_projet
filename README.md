# Jeu de l'Aventure

Jeu RPG tour par tour développé en Rust avec le moteur de jeu Bevy.

---

## Lore

Dans un monde oublié où la magie et l'acier se côtoient, vous incarnez un héros désigné par le destin pour affronter les ténèbres qui menacent le royaume. Quatre voies s'offrent à vous :

Le **Guerrier**, gardien robuste des anciennes forteresses, dont la force et l'endurance font trembler les plus féroces adversaires. Formé dans les donjons profonds, il est le rempart contre les hordes ennemies.

Le **Magicien**, érudit des arcanes oubliées, capable de déchaîner des puissances destructrices au prix de sa propre fragilité. Son savoir provient des grimoires anciens cachés dans les bibliothèques perdues.

L'**Assassin**, ombre furtive et maître de l'esquive, dont les mouvements rapides comme l'éclair lui permettent de frapper avant même que ses ennemis ne réalisent sa présence.

Le **Bourreau**, exécuteur impitoyable dont les coups sont si précis qu'ils peuvent trancher la vie d'un seul coup critique. Redouté pour sa froideur et sa détermination.

Votre quête vous mènera à travers des chemins périlleux peuplés de gobelins et de créatures hostiles. Chaque victoire vous rendra plus fort, chaque objet trouvé vous rapprochera de votre destinée. Les ennemis vaincus ne reviendront plus hanter ces terres, mais de nouveaux dangers vous attendent à chaque tournant.

Oserez-vous braver l'inconnu et restaurer la paix dans ce monde chaotique ?

---

## Documentation

Cette section documente les concepts et techniques Rust/Bevy utilisés dans le projet, particulièrement utiles pour comprendre le code.

_Disclaimer_ : La dernière section de la doc explique comment l'IA a été utilisée pour développer le projet.

### 1. Système ECS (Entity-Component-System)

**Concept** : Bevy utilise l'architecture ECS où :

- **Entity** : Un identifiant unique (ex: le joueur, un ennemi)
- **Component** : Des données attachées à une entité (ex: `Position`, `Stats`, `Player`)
- **System** : Des fonctions qui opèrent sur les entités ayant certains components

**Exemple dans le code** :

```rust
#[derive(Component)]
struct Player { ... }

fn move_player(
    mut player_query: Query<&mut Position, With<Player>>
) { ... }
```

La query `Query<&mut Position, With<Player>>` récupère toutes les entités ayant un component `Position` ET un component `Player`.

### 2. States (États du jeu)

**Concept** : Les `States` permettent de gérer le flow de l'application en définissant différents modes de jeu.

```rust
#[derive(States)]
enum GameState {
    ClassSelection,  // Écran de sélection de classe
    Map,            // Exploration de la carte
    Combat,         // Combat tour par tour
    MapTransition,  // Transition entre cartes
}
```

**Systèmes conditionnels** :

- `.add_systems(OnEnter(GameState::Combat), setup_combat)` : Exécuté UNE FOIS à l'entrée de l'état Combat
- `.add_systems(Update, handle_combat.run_if(in_state(GameState::Combat)))` : Exécuté À CHAQUE FRAME tant qu'on est en état Combat
- `.add_systems(OnExit(GameState::Combat), cleanup_combat)` : Exécuté UNE FOIS à la sortie de l'état Combat

### 3. Resources

**Concept** : Les `Resource` sont des données globales accessibles partout dans le jeu (contrairement aux Components qui sont attachés à des entités spécifiques).

```rust
#[derive(Resource)]
struct GameLog {
    messages: Vec<String>,
}
```

**Utilisation** : On accède à une resource avec `Res<T>` (lecture seule) ou `ResMut<T>` (lecture/écriture).

```rust
fn update_log(mut game_log: ResMut<GameLog>) {
    game_log.add_message("Nouveau message".to_string());
}
```

### 4. HashSet pour la persistence

**Concept** : `HashSet` est une collection qui ne peut contenir qu'une seule fois chaque élément (pas de doublons).

```rust
#[derive(Resource, Default)]
struct CollectedItems {
    items: HashSet<(usize, Position)>,
}
```

**Utilisation** : Permet de garder en mémoire les objets collectés et ennemis vaincus, même après des transitions de carte.

```rust
// Marquer un objet comme collecté
collected_items.items.insert((map_index, position));

// Vérifier si déjà collecté
if collected_items.items.contains(&(map_index, position)) {
    // Ne pas respawner l'objet
}
```

### 5. Z-ordering (Profondeur des sprites)

**Concept** : Le composant `transform.translation.z` détermine l'ordre d'affichage des sprites (plus le Z est élevé, plus le sprite est au premier plan).

**Convention dans le projet** :

- `0.0` : Tuiles de la carte (arrière-plan)
- `0.5` : Objets collectables
- `0.7` : Ennemis
- `1.0` : Joueur (toujours visible au premier plan)

```rust
Transform::from_translation(position.extend(1.0)) // Player au Z=1.0
```

### 6. Query avec filtres multiples

**Concept** : On peut filtrer les queries pour éviter les conflits de mutabilité.

```rust
Query<&mut Text, (With<StatsText>, Without<LogText>)>
```

Signification : Récupère toutes les entités ayant un `Text` ET un `StatsText`, MAIS PAS de `LogText`. Le `Without` est crucial quand on a deux queries mutables sur le même type de component.

### 7. Marker Components

**Concept** : Des components vides utilisés uniquement pour identifier/filtrer des entités.

```rust
#[derive(Component)]
struct MapTile;

#[derive(Component)]
struct CombatUI;
```

**Utilisation** : Permet de despawn facilement toutes les entités d'une catégorie :

```rust
fn cleanup_combat(
    mut commands: Commands,
    query: Query<Entity, With<CombatUI>>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
```

### 8. Optimisation UI : Change Detection

**Concept** : Bevy peut détecter si une resource a changé pour éviter des re-renders inutiles.

```rust
// Ne mettre à jour que si le texte a changé
if text.sections[0].value != new_stats {
    text.sections[0].value = new_stats;
}
```

Cela évite de redessiner l'UI à chaque frame si rien n'a changé, ce qui améliore les performances.

### 9. Gestion des assets

**Concept** : Les assets (images, fonts) sont chargés de manière asynchrone par Bevy.

```rust
#[derive(Resource)]
struct ImageAssets {
    path_tile: Handle<Image>,
    warrior_class: Handle<Image>,
    // ...
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let image_assets = ImageAssets {
        path_tile: asset_server.load("images/Chemin.jpg"),
        // ...
    };
    commands.insert_resource(image_assets);
}
```

Le `Handle<Image>` est une référence vers l'image qui peut être clonée et partagée sans dupliquer l'image en mémoire.

### 10. Mécanique de combat RNG

**Concept** : Utilisation de `rand` pour générer des nombres aléatoires.

```rust
use rand::Rng;
let mut rng = rand::thread_rng();

// Esquive : nombre aléatoire entre 1 et 100
let dodge_roll = rng.gen_range(1..=100);
if dodge_roll <= stats.speed {
    // Esquive réussie !
}
```

**Critique** : Même principe avec `critical_chance` pour doubler les dégâts.

---

## Commandes

- **Sélection de classe** : `1`/`2`/`3`/`4` ou clic sur les icônes
- **Déplacement** : `Z`/`Q`/`S`/`D` ou flèches directionnelles
- **Combat** : `ESPACE` pour attaquer
- **Amélioration après victoire** : `H` (HP), `A` (ATK), `S` (SPD), `C` (CRIT)
- **Recommencer après défaite** : `R`

## Structure du projet

```
src/
├── main.rs       # Point d'entrée, UI, systèmes principaux
├── entity.rs     # Position, Stats, composants partagés
├── player.rs     # Classes et gestion du joueur
├── enemy.rs      # Types d'ennemis et leurs stats
├── combat.rs     # Logique de combat (actuellement inutilisé)
├── item.rs       # Types d'objets et bonus
├── map.rs        # Génération et navigation de carte
└── assets.rs     # Chargement des assets

assets/
├── fonts/        # Police pour l'UI
└── images/       # Sprites des classes, ennemis, objets, tuiles
```

## Compilation et exécution

```bash
# Compiler le projet
cargo build

# Lancer le jeu
cargo run

# Vérifier le code (linting)
cargo clippy

# Formater le code
cargo fmt
```

## Assistance de l'IA dans le projet

L'IA a été utilisé pour la compréhension de certains concepts de Rust/Bevy, ainsi que pour la gestion de la structure du code et certains éléments de la documentation.

**Parmi les concepts utilisés**, on trouve les points suivants :

- **assets.rs** :

  - Utilisation de **Handle<T>** pour gérer les assets (ici les images) du jeu. Handle<T> est générique et est un "point intelligent" qui permet de gérer les assets de manière asynchrone avec des références vers l'asset chargé plutôt que l'asset lui-même.
  - Utilisation de **AssetServer** en paramètre de la méthode pour charger les assets.

- **enemy.rs** :

  - Utilisation de **derive** pour implémenter des traits sur des types personnalisés. Notamment :
    ```rust
    #[derive(Debug, Clone, Copy, PartialEq, Component)]
    ```
    - Debug permet de faire un pretty print pour le debug
    - Clone permet de cloner l'objet
    - Copy permet de copier l'objet
    - PartialEq permet de comparer deux objets
    - Component permet de dire que c'est un composant Bevy

- **map.rs** :

  - Utilisation de **HashMap**, qui finalement fait partie de la bibliothèque standard de Rust et fonctionne de manière identique à celle trouvé sur les autres langages de programmation (comme Java, Python ...).
  - Compréhension de la différence entre :
    ```rust
    Vec<T> : // Créer un vecteur dynamique vide
    vec![] : // Créer un vecteur dynamique avec des éléments
    HashMap : // Créer un dictionnaire vide
    ```
  - Utilisation de **closure** pour itérer sur les éléments d'un vecteur avec :
    ```rust
    |element| { ... }
    ```
    qui fonctionne comme des "une fonction anonyme" qui permet d'économiser du code.

**La majorité des notions propre à beavy qui m'ont été expliquées par l'IA sont listés ci-dessous.**

- ## **main.rs** :
  ```rust
  App::new() : Créer une nouvelle application Bevy
  add_plugins : Ajouter des plugins Bevy
  add_state : Ajouter un état Bevy // sert à gérer les états du jeu
  add_systems : Ajouter des systèmes Bevy // sert à exécuter des fonctions à chaque frame
  NodeBundle : Ajouter un bundle Bevy // sert à ajouter des composants à une entité
  ```

L'auto-complétion de l'IA a été utilisée également pour accélérer le développement du projet.

L'essentiel des éléments du projet ayant été assité par IA (en plus des quelques concepts manquants) concerne l'utilisation de **Bevy**, dans un contexte **ECS (Entity Component System)**.
