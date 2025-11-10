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
- **Stats**: Use shared Stats struct (entity.rs) for player/enemy/item stats consistency.
- **Z-ordering**: Tiles=0.0, Items=0.5, Player=1.0 for proper sprite layering.

## Project Structure
- src/main.rs - App setup, UI, game states
- src/entity.rs - Shared Position, Stats, Enemy
- src/player.rs - Player, PlayerClass
- src/combat.rs - Turn-based combat logic
- src/item.rs - Item types and stat bonuses
- src/map.rs - Map generation and navigation
- src/assets.rs - Asset loading
