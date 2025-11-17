# Agent Guidelines - jeu_projet

Bevy 0.12 ECS RPG game in Rust (edition 2024). Target: macOS. Language: French for comments/strings.

## Commands
- `cargo build` / `cargo run` - Build/run game
- `cargo test` - Run all tests | `cargo test <test_name>` - Run specific test
- `cargo clippy` - Lint | `cargo fmt` - Format (run before commits)

## Code Style
- **Imports**: Group std → external (bevy, rand) → local (use crate::). Example: see src/player.rs:1-3
- **Formatting**: 4-space indents, run `cargo fmt` before commit
- **Types**: Explicit types for public APIs, derive Debug/Clone/Copy where sensible
- **Naming**: snake_case (functions/vars), PascalCase (types/enums), SCREAMING_SNAKE_CASE (constants)
- **Bevy ECS**: Component derive for entities, marker components for UI/map, resources for global state, systems with queries
- **Error Handling**: Result/Option for fallible ops, avoid unwrap() in production
- **Comments**: French preferred, `///` doc comments for public functions with usage examples (see src/player.rs:39-45)
- **Z-ordering**: Tiles=0.0, Items=0.5, Enemies=0.7, Player=1.0 (sprite layering)

## Architecture
- **Core**: main.rs (app setup), assets.rs, player.rs (PlayerClass enum), enemy.rs, item.rs
- **Components**: position.rs, stats.rs (shared Stats struct), markers.rs
- **Resources**: game_log.rs, combat_state.rs, collected_items.rs (HashSet<(map_index, Position)>), defeated_enemies.rs, selected_class.rs
- **States**: GameState enum (ClassSelection → Map ↔ MapTransition ↔ Combat)
- **Systems**: camera/, class_selection/, map/ (data/spawn/despawn/transition), player/ (movement/transform/item_pickup/enemy_encounter), combat/ (setup/logic/calculations/cleanup), ui/

## Critical Rules
- **Stats**: Always use components/stats.rs Stats struct for Player/Enemy/Item consistency
- **Maps**: 14-char lines, place items/enemies on walkable tiles ('P'/' '/'C'), never walls ('W'). Ensure bidirectional connectors to avoid teleport loops
- **System Order**: Player systems run parallel in Update, no .chain() needed yet
