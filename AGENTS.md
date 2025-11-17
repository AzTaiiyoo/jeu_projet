# Agent Guidelines - jeu_projet

Bevy-based RPG game in Rust. Target: macOS. Language: French comments/strings preferred.

## Build & Test Commands
- `cargo build` - Build project
- `cargo run` - Run game
- `cargo test` - Run all tests
- `cargo test <test_name>` - Run single test
- `cargo clippy` - Lint checks
- `cargo fmt` - Format code

## Code Style
- **Imports**: Group std → external crates (bevy, rand) → local modules (use crate::)
- **Formatting**: Run `cargo fmt` before commit. Use 4-space indents.
- **Types**: Use explicit types for public APIs. Derive Debug, Clone, Copy where applicable.
- **Naming**: snake_case for functions/variables, PascalCase for types/enums, SCREAMING_SNAKE for constants.
- **Components**: Use Bevy Component derive for ECS entities. Mark UI/map entities with marker components.
- **Error Handling**: Use Result/Option where fallible. Avoid unwrap() in production code.
- **Comments**: French preferred. Document public functions with `///` doc comments.
- **Bevy Patterns**: Use systems with queries, resources for global state, states for game flow.
- **Z-ordering**: Tiles=0.0, Items=0.5, Enemies=0.7, Player=1.0 for proper sprite layering.

## Project Structure (Restructuré)

### Core Files
- **src/main.rs** (73 lignes) - App setup minimal, system registration, states
- **src/assets.rs** - Asset loading (images, fonts)
- **src/player.rs** - Player struct, PlayerClass enum
- **src/enemy.rs** - Enemy struct, EnemyType enum
- **src/item.rs** - Item struct, ItemType enum

### Components (`src/components/`)
- **position.rs** - Position component (x, y grid coordinates)
- **stats.rs** - Stats struct (hp, attack, speed, critical_chance)
- **markers.rs** - Marker components (MapTile, InfoTerminal, CombatUI, ClassSelectionUI)

### Resources (`src/resources/`)
- **game_log.rs** - GameLog (messages système)
- **combat_state.rs** - CombatState, CurrentEnemy (état du combat)
- **collected_items.rs** - CollectedItems (objets ramassés par map)
- **defeated_enemies.rs** - DefeatedEnemies (ennemis vaincus par map)
- **selected_class.rs** - SelectedClass (classe choisie)

### States (`src/states/`)
- **mod.rs** - GameState enum (ClassSelection, Map, MapTransition, Combat)

### Config (`src/config/`)
- **mod.rs** - Constantes globales (TILE_SIZE, etc.)

### Systems (`src/systems/`)

#### Camera (`systems/camera/`)
- **setup.rs** - Setup de la caméra 2D

#### Class Selection (`systems/class_selection/`)
- **setup.rs** - UI de sélection de classe
- **input.rs** - Gestion des inputs (1-5 pour choisir)
- **cleanup.rs** - Nettoyage de l'UI
- **spawn.rs** - Spawn du joueur avec la classe choisie

#### Map (`systems/map/`)
- **data.rs** - Map struct, GameData resource, layouts des 3 maps
- **spawn.rs** - Spawn des tuiles, objets, ennemis
- **despawn.rs** - Despawn de la map actuelle
- **transition.rs** - Gestion des transitions entre maps

#### Player (`systems/player/`)
- **movement.rs** - Déplacement du joueur (ZQSD/flèches)
- **transform.rs** - Mise à jour de la position visuelle
- **item_pickup.rs** - Détection et collecte d'objets
- **enemy_encounter.rs** - Détection et déclenchement de combats

#### Combat (`systems/combat/`)
- **setup.rs** - Setup de l'UI de combat
- **logic.rs** - Logique des tours (attaque, défense, fuite)
- **calculations.rs** - Calculs de dégâts, critiques
- **cleanup.rs** - Nettoyage après combat

#### UI (`systems/ui/`)
- **info_terminal.rs** - Terminal d'infos (stats, log, inventaire)

## Important Notes
- **Map Connections**: Toujours s'assurer que les connexions sont bidirectionnelles et que le joueur n'arrive pas directement sur un connecteur (éviter les boucles de téléportation)
- **Map Layout**: Chaque ligne doit faire exactement 14 caractères. Les objets/ennemis doivent être placés sur des cases praticables ('P', ' ', ou 'C'), jamais sur des murs ('W')
- **Stats Consistency**: Utiliser le même struct Stats pour Player, Enemy, et Item (dans components/stats.rs)
- **System Ordering**: Les systèmes du joueur s'exécutent en parallèle dans Update. Pas besoin de .chain() pour l'instant.
- **Resource Tracking**: CollectedItems et DefeatedEnemies utilisent des HashSet<(map_index, Position)> pour persister entre les maps
