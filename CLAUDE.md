# Jeu de l'Aventure - Documentation du Projet

## 📋 Vue d'ensemble

**Jeu de l'Aventure** est un roguelike 2D développé en Rust avec le moteur de jeu Bevy. Le joueur explore des cartes, combat des ennemis, collecte des objets et améliore ses statistiques pour progresser dans l'aventure.

---

## 🎯 Objectif du Jeu

Le joueur incarne un aventurier qui doit explorer des maps interconnectées, combattre des ennemis de difficulté croissante et devenir suffisamment fort pour vaincre le boss final : le **Loup**.

### Progression du Joueur

- **Exploration** : Naviguer entre différentes maps via des points de connexion
- **Combat** : Affronter des ennemis en combat tour par tour
- **Collecte** : Ramasser des objets pour améliorer ses statistiques
- **Évolution** : Choisir une statistique à améliorer après chaque victoire

### Condition de Victoire

Vaincre tous les ennemis, en particulier le Loup (boss) de la deuxième map.

---

## 🎮 Fonctionnalités Implémentées

### 1. Système de Classes (Sélection Initiale)

Le joueur choisit parmi 4 classes, chacune avec des stats de base différentes :

| Classe       | HP  | ATK | SPD | CRIT |
| ------------ | --- | --- | --- | ---- |
| **Guerrier** | 120 | 10  | 5   | 10%  |
| **Magicien** | 90  | 15  | 3   | 15%  |
| **Assassin** | 100 | 8   | 12  | 15%  |
| **Bourreau** | 120 | 7   | 2   | 25%  |

**Sélection** : Clic sur le bouton ou touches `1`, `2`, `3`, `4`

---

### 2. Système de Carte et Navigation

#### Maps

- **2 maps** interconnectées de taille 14x10 cases
- Tiles : Chemins praticables, murs infranchissables, connexions entre maps
- **Z-ordering** : Tiles (0.0) → Items (0.5) → Ennemis (0.7) → Joueur (1.0)

#### Déplacement

- **Touches** : `Z`/`Q`/`S`/`D` ou Flèches directionnelles
- Validation de collision avec les murs
- Transition automatique vers la map suivante via les connexions

#### Caméra

- Caméra 2D avec scaling automatique adapté à la taille de la map
- Fenêtre redimensionnable

---

### 3. Système d'Objets

4 types d'objets collectables avec des bonus permanents :

| Objet         | Bonus     |
| ------------- | --------- |
| **Armure**    | HP +50    |
| **Katana**    | SPD +5    |
| **Gants**     | ATK +10   |
| **Pendentif** | CRIT +10% |

**Fonctionnalités** :

- ✅ Collecte automatique en marchant dessus
- ✅ Les objets ne réapparaissent pas (système de tracking par map)
- ✅ Persistance entre changements de map
- ✅ Affichage des bonus dans le terminal d'information

---

### 4. Système d'Ennemis

4 types d'ennemis avec difficulté progressive :

| Ennemi            | HP  | ATK | SPD | CRIT | Niveau    |
| ----------------- | --- | --- | --- | ---- | --------- |
| **Petit Gobelin** | 30  | 5   | 8   | 5%   | Facile    |
| **Gobelin Moyen** | 50  | 8   | 5   | 10%  | Moyen     |
| **Gros Gobelin**  | 80  | 12  | 3   | 8%   | Difficile |
| **Loup (Boss)**   | 100 | 15  | 12  | 20%  | Boss      |

**Distribution** :

- **Map 1** : 3 Petits Gobelins + 1 Gobelin Moyen
- **Map 2** : 3 Gobelins Moyens + 1 Gros Gobelin + 1 Loup (boss)

**Fonctionnalités** :

- ✅ Spawn dynamique sur les maps
- ✅ Les ennemis vaincus ne réapparaissent pas (système de tracking)
- ✅ Assets graphiques différents pour chaque type

---

### 5. Système de Combat Tour par Tour

#### Déclenchement

Combat automatique quand le joueur marche sur la case d'un ennemi.

#### Interface de Combat

- Overlay fullscreen avec fond noir semi-transparent
- Affichage des stats du joueur et de l'ennemi
- Log de combat en temps réel
- Instructions claires

#### Mécanique de Combat

**Tour du Joueur** (appuyez sur `ESPACE`) :

1. **Calcul d'esquive** : L'ennemi esquive si `random(1-100) ≤ SPD_ennemi`
2. **Calcul de critique** : Coup critique si `random(1-100) ≤ CRIT_joueur`
3. **Dégâts** : ATK du joueur (×2 si critique)
4. Mise à jour des HP de l'ennemi

**Tour de l'Ennemi** (automatique) :

1. **Calcul d'esquive** : Le joueur esquive si `random(1-100) ≤ SPD_joueur`
2. **Calcul de critique** : Coup critique si `random(1-100) ≤ CRIT_ennemi`
3. **Dégâts** : ATK de l'ennemi (×2 si critique)
4. Mise à jour des HP du joueur

#### Fin de Combat

**Victoire** :

- Choix d'amélioration de stat :
  - `H` : HP +10
  - `A` : ATK +2
  - `S` : SPD +1
  - `C` : CRIT +2%
- L'ennemi disparaît définitivement de la map
- Retour automatique à l'exploration

**Défaite** :

- Game Over si HP du joueur ≤ 0
- `R` : Recommencer (retour à la sélection de classe)

---

### 6. Terminal d'Information

Panneau latéral (280px) affiché en permanence pendant l'exploration :

#### Section Statistiques

- Classe du joueur
- HP, Attaque, Vitesse, Critique (mise à jour en temps réel)

#### Section Événements (Log)

- Message de bienvenue
- Notification de ramassage d'objets
- Bonus de stats obtenus
- Résultats de combat
- 10 derniers messages conservés

**Style** :

- Fond gris foncé semi-transparent (rgba(0.15, 0.15, 0.15, 0.95))
- Police par défaut de Bevy (la police custom FiraSans était vide)
- Titres en jaune, texte principal en blanc

---

## 🛠️ Architecture Technique

### Structure du Projet

```
jeu_projet/
├── src/
│   ├── main.rs          # Point d'entrée, UI, états du jeu, systèmes
│   ├── assets.rs        # Gestion des assets graphiques
│   ├── combat.rs        # Logique de combat (non utilisé actuellement)
│   ├── entity.rs        # Stats, Position (structures partagées)
│   ├── enemy.rs         # Types d'ennemis et leurs stats
│   ├── item.rs          # Types d'objets et leurs bonus
│   ├── map.rs           # Génération et gestion des maps
│   └── player.rs        # Classes de joueur et stats
├── assets/
│   ├── fonts/           # Polices (FiraSans-Bold.ttf)
│   └── images/          # Sprites pour classes, ennemis, objets, tiles
├── Cargo.toml
├── AGENTS.md            # Guidelines pour agents de code
└── CLAUDE.md            # Ce fichier
```

### Technologies Utilisées

- **Langage** : Rust (édition 2024)
- **Moteur de jeu** : Bevy 0.12
- **Dépendances** :
  - `bevy` : Moteur ECS, rendu, UI
  - `rand` : Génération de nombres aléatoires pour le combat

### États du Jeu (States)

```
ClassSelection → Map ⇄ MapTransition
                  ↓
                Combat
```

- **ClassSelection** : Sélection de la classe du joueur
- **Map** : Exploration et mouvement du joueur
- **Combat** : Combat tour par tour avec un ennemi
- **MapTransition** : Transition entre maps (instantanée)

### Ressources Globales (Resources)

| Ressource         | Description                                    |
| ----------------- | ---------------------------------------------- |
| `GameData`        | Contient toutes les maps du jeu                |
| `CollectedItems`  | Set des objets collectés (map_index, position) |
| `DefeatedEnemies` | Set des ennemis vaincus (map_index, position)  |
| `GameLog`         | Messages de jeu (10 derniers)                  |
| `CurrentEnemy`    | Ennemi actuellement en combat                  |
| `CombatState`     | État du combat (HP, log, tour)                 |
| `ImageAssets`     | Handles vers tous les assets graphiques        |

### Systèmes Bevy Principaux

**État Map** :

- `spawn_map` : Génère les tiles, objets et ennemis
- `move_player` : Gère le déplacement du joueur
- `update_player_transform` : Synchronise position logique/visuelle
- `check_for_item_pickup` : Détecte et gère la collecte d'objets
- `check_for_enemy_encounter` : Détecte collision avec ennemis
- `update_info_terminal` : Met à jour l'UI d'information
- `despawn_map` : Nettoie la map au changement d'état

**État Combat** :

- `setup_combat` : Crée l'UI de combat
- `handle_combat` : Gère les tours de combat et choix de stats
- `cleanup_combat` : Nettoie l'UI et ressources de combat

---

## 🐛 Bugs Corrigés

### Bug 1 : Objets Réapparaissent Entre Maps

**Problème** :

- En passant de la Map 1 → Map 2 → Map 1, les objets collectés réapparaissaient
- Le joueur pouvait collecter les mêmes objets infiniment

**Cause** :

- `spawn_map` respawnait tous les objets de `game_map.items` sans vérification
- Pas de système de tracking des objets collectés

**Solution** :

1. Création de la ressource `CollectedItems` : HashSet<(usize, Position)>
2. Modification de `check_for_item_pickup` pour enregistrer chaque objet collecté
3. Modification de `spawn_map` pour vérifier si un objet a été collecté avant de le spawner
4. Persistance globale entre les maps

**Fichiers modifiés** : `src/main.rs`

---

### Bug 2 : Police de Caractères Manquante

**Problème** :

- Le terminal d'information s'affichait (rectangle rouge de test visible)
- Mais aucun texte n'apparaissait dedans
- Pas de warning explicite sur la police

**Cause** :

- Le fichier `assets/fonts/FiraSans-Bold.ttf` existait mais était **vide (0 octets)**
- Bevy chargeait la police mais ne pouvait pas rendre le texte

**Solution** :

1. Utilisation de la police par défaut de Bevy
2. Suppression de toutes les références à `asset_server.load("fonts/FiraSans-Bold.ttf")`
3. Utilisation de `TextStyle { font_size: X, color: Y, ..default() }`

**Note** : Pour utiliser une vraie police custom, télécharger FiraSans depuis Google Fonts et remplacer le fichier vide.

**Fichiers modifiés** : `src/main.rs`

---

### Bug 3 : Panic au Choix de Stat Après Combat

**Problème** :

```
thread 'Compute Task Pool (1)' panicked at src/main.rs:1074:22:
Attempting to create an EntityCommands for entity 151v0, which doesn't exist.
```

- Se produisait systématiquement lors du choix d'une stat (H/A/S/C) après victoire
- L'erreur persistait même après la première tentative de correction

**Cause Racine** :

Le problème venait du cycle de vie des états Bevy :

1. **Joueur marche sur ennemi** → Transition `Map → Combat`
2. **`OnExit(GameState::Map)`** est déclenché → `despawn_map()` supprime **TOUTES** les entités (tiles, items, **ennemis**)
3. **`OnEnter(GameState::Combat)`** → `setup_combat()` crée l'UI
4. **Combat et victoire** → Joueur choisit une stat
5. **Code tentait** `commands.entity(current_enemy.entity).despawn()` 
6. **❌ PANIC** : L'entité n'existe plus (supprimée à l'étape 2)

L'entité ennemie était donc **déjà détruite** par `despawn_map` lors de `OnExit(Map)`, avant même que le joueur ne choisisse une stat.

**Tentative de Solution Initiale** :

1. Création de la ressource `DefeatedEnemies` : HashSet<(usize, Position)>
2. Modification de `CurrentEnemy` pour inclure la position de l'ennemi
3. Marquage de l'ennemi comme vaincu dans `DefeatedEnemies`
4. `spawn_map` vérifie si un ennemi a été vaincu avant de le spawner

**Problème** : Le code tentait toujours de `despawn` l'entité qui n'existait plus → Même erreur

**Solution Finale** :

**Suppression complète de la ligne `despawn`** dans la gestion de victoire (`handle_combat`).

**Flux correct après correction** :
```
Map (ennemi présent)
  ↓
OnExit(Map) : despawn_map() supprime TOUT (y compris l'ennemi)
  ↓
Combat (entité ennemie n'existe plus)
  ↓
Victoire + choix stat
  ↓
Marquer position ennemi dans DefeatedEnemies
  ↓ (PAS de despawn ici, l'entité n'existe déjà plus)
OnExit(Combat) : cleanup UI
  ↓
OnEnter(Map) : spawn_map() skip les ennemis dans DefeatedEnemies
  ↓
Map (ennemi absent définitivement) ✅
```

**Changement de code** :
```rust
// AVANT (causait le panic)
if stat_chosen {
    defeated_enemies.enemies.insert((game_data.current_map_index, current_enemy.position));
    commands.entity(current_enemy.entity).despawn(); // ❌ Entité n'existe plus !
    next_state.set(GameState::Map);
}

// APRÈS (fonctionne)
if stat_chosen {
    defeated_enemies.enemies.insert((game_data.current_map_index, current_enemy.position));
    // Pas de despawn, l'entité a déjà été supprimée par OnExit(Map)
    next_state.set(GameState::Map);
}
```

**Résultat** :

- ✅ Plus de panic lors du choix de stat
- ✅ Les ennemis vaincus ne réapparaissent jamais
- ✅ Même système que pour les objets collectés
- ✅ Persistance entre maps

**Fichiers modifiés** : `src/main.rs`

---

## 🚀 Commandes de Build et Exécution

### Build

```bash
cargo build                # Build debug
cargo build --release      # Build optimisé
```

### Exécution

```bash
cargo run                  # Lancer le jeu
```

### Formatage et Linting

```bash
cargo fmt                  # Formater le code
cargo clippy               # Analyse statique
```

### Tests

```bash
cargo test                 # Tous les tests
cargo test <test_name>     # Test spécifique
```

---

## 🎯 Améliorations Futures Possibles

### Gameplay

- [ ] Plus de maps (3-5 maps au total)
- [ ] Plus de types d'ennemis
- [ ] Boss final plus difficile
- [ ] Système d'inventaire
- [ ] Objets consommables (potions)
- [ ] Différents types d'armes

### Équilibrage

- [ ] Ajuster les stats des ennemis
- [ ] Revoir les bonus des objets
- [ ] Scaling de difficulté plus progressif

### Technique

- [ ] Sauvegardes de partie
- [ ] Menu principal
- [ ] Paramètres (volume, résolution)
- [ ] Animations de combat
- [ ] Effets sonores et musique
- [ ] Meilleure gestion des polices custom

### UI/UX

- [ ] Support complet des caractères accentués
- [ ] Animations de transition entre maps
- [ ] Feedback visuel (dégâts, esquive)
- [ ] Minimap
- [ ] Barre de HP visuelle

---

## 📝 Notes de Développement

### Conventions de Code

- **Langue** : Commentaires et strings en français
- **Formatting** : 4 espaces d'indentation, `cargo fmt` avant commit
- **Naming** : snake_case (fonctions/variables), PascalCase (types/enums)
- **ECS Pattern** : Utilisation systématique de Components, Resources, Systems

### Z-ordering des Sprites

```
0.0  → Tiles (sol)
0.5  → Items (objets)
0.7  → Ennemis
1.0  → Joueur
1000 → UI Terminal (z_index)
2000 → UI Combat (z_index)
```

### Structure Stats Partagée

La struct `Stats` (entity.rs) est utilisée par :

- Player
- Enemy
- Item (bonus)

Cela garantit la cohérence et facilite les calculs de combat.

---

## 👥 Crédits

**Développeur** : Kiran Bonhomme  
**Framework** : Bevy 0.12  
**Langage** : Rust  
**Plateforme cible** : macOS

---

## 📄 Licence

Projet académique - Semestre 9

---

**Dernière mise à jour** : Novembre 2025
