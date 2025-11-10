Plan de modification du code étape par étape
Contexte
Le projet Rust actuel est un jeu roguelike en terminal. L'objectif est de le transformer en jeu graphique avec Bevy, en ajoutant 4 classes de personnages, 4 objets, et une représentation visuelle de la carte avec des images.

Étape 1 : Mise en place de Bevy et structure de base
Objectif : Initialiser Bevy et créer la structure pour gérer l'interface graphique.
Modifications :

Ajouter la dépendance bevy dans Cargo.toml (version stable recommandée : 0.12 ou plus récente)
Créer un nouveau fichier main.rs qui utilisera Bevy au lieu de la boucle de jeu terminal
Définir les états du jeu avec Bevy States :

ClassSelection : écran de sélection de classe
Map : jeu principal avec déplacement sur la carte
Combat : écran de combat (optionnel pour plus tard)

Code à créer :

Structure App avec les plugins Bevy de base
Système de caméra 2D
Configuration du dossier d'assets pointant vers public/

Étape 2 : Mise à jour des classes de personnages
Objectif : Modifier player.rs pour inclure les 4 classes avec leurs statistiques équilibrées.
Modifications dans player.rs :

Remplacer l'enum Class par PlayerClass avec 4 variantes :

Guerrier
Magicien
Assassin
Bourreau

Mettre à jour la fonction get_stats_for_class() avec les nouvelles statistiques :

Guerrier : HP=120, ATK=10, SPD=5, CRIT=10%
Magicien : HP=90, ATK=15, SPD=3, CRIT=15%
Assassin : HP=100, ATK=8, SPD=12, CRIT=15%
Bourreau : HP=120, ATK=7, SPD=2, CRIT=25%

Ajouter un champ image_path ou utiliser un match pour associer chaque classe à son image :

Guerrier → public/classes/Guerrier.jpg
Magicien → public/classes/Magicien.jpg
Assassin → public/classes/Assassin.jpg
Bourreau → public/classes/Bourreau.jpg

Étape 3 : Mise à jour des objets
Objectif : Modifier item.rs pour inclure les 4 nouveaux objets avec leurs effets.
Modifications dans item.rs :

Remplacer ou étendre l'enum ItemType avec :

Armure : +50 HP
Katana : +5 Vitesse
Gants : +10 Attaque
Pendentif : +10% Critique

Mettre à jour get_stats_for_item() pour retourner les bonnes statistiques
Associer chaque objet à son image :

Armure → public/objets/Armure.jpg
Katana → public/objets/Katana.jpg
Gants → public/objets/Gants.jpg
Pendentif → public/objets/Pendentif.jpg

Étape 4 : Création de l'écran de sélection de classe
Objectif : Créer une interface graphique Bevy pour choisir sa classe au début.
Systèmes Bevy à créer :

setup_class_selection_ui (OnEnter ClassSelection) :

Afficher un titre "Choisissez votre classe"
Afficher 4 boutons/images cliquables représentant chaque classe
Ou utiliser les touches 1, 2, 3, 4 pour sélectionner

handle_class_selection (Update pendant ClassSelection) :

Détecter l'input du joueur (clic ou touche)
Créer l'entité Player avec la classe choisie
Charger l'image correspondante
Transitionner vers l'état Map

cleanup_class_selection_ui (OnExit ClassSelection) :

Nettoyer les éléments UI de sélection

Étape 5 : Mise à jour de la carte avec images
Objectif : Remplacer l'affichage texte de la carte par des sprites Bevy.
Modifications dans map.rs :

Garder la structure de la carte (matrice de TileType)
Créer une Resource GameMap contenant la grille de tuiles

Nouveau système Bevy :

spawn_map (OnEnter Map) :

Charger les images Chemin.jpg et Muraille.jpg
Pour chaque case de la carte, spawner un sprite :

Position calculée en fonction de la taille des tuiles (ex: 64x64 pixels)
Texture : Chemin ou Muraille selon TileType

Définir une constante TILE_SIZE = 64.0 pour la taille en pixels

Utiliser un composant Position { x, y } pour tracking logique
Utiliser le Transform de Bevy pour la position graphique

Étape 6 : Affichage du joueur sur la carte
Objectif : Spawner le sprite du joueur sur la carte avec l'image de sa classe.
Système Bevy :

Dans spawn_map ou un système dédié spawn_player :

Récupérer l'entité Player créée lors de la sélection
Charger l'image correspondant à sa classe
Spawner un sprite à la position initiale du joueur
Ajouter le composant Position pour la logique de jeu
Le sprite doit être au-dessus des tuiles (z-order plus élevé)

Étape 7 : Placement des objets sur la carte
Objectif : Spawner les 4 objets à des positions fixes avec leurs images.
Système Bevy :

Dans spawn_map ou spawn_items :

Définir les positions fixes des objets (ex: Position { x: 3, y: 3 })
Pour chaque objet :

Spawner un sprite avec l'image correspondante
Ajouter les composants Item et Position
Z-order entre la carte et le joueur

Créer une liste des objets à placer :

rust let items = vec![
       (Position { x: 2, y: 3 }, ItemType::Katana),
       (Position { x: 5, y: 5 }, ItemType::Armure),
       // etc.
   ];

Étape 8 : Gestion du déplacement du joueur
Objectif : Permettre au joueur de se déplacer avec le clavier et mettre à jour son sprite.
Système Bevy :

move_player (Update pendant Map) :

Détecter les inputs clavier (ZQSD ou flèches)
Calculer la nouvelle position logique
Vérifier si la tuile est praticable (Path et pas Wall)
Mettre à jour le composant Position

update_player_transform (Update pendant Map) :

Synchroniser le Transform du sprite avec le Position logique
Conversion : transform.translation.x = position.x \* TILE_SIZE

Important : Supporter à la fois ZQSD (AZERTY) et WASD (QWERTY)

Étape 9 : Ramassage d'objets
Objectif : Détecter quand le joueur est sur un objet et l'appliquer.
Système Bevy :

check_for_item_pickup (Update pendant Map) :

Query toutes les entités Item avec leur Position
Query le Player avec sa Position
Si positions identiques :

Appliquer les stats de l'objet au joueur
Despawn l'entité objet
Afficher un message (console ou UI)

Étape 10 : Chargement et gestion des assets
Objectif : Centraliser le chargement des images pour éviter de les recharger.
Système Bevy :

Créer une Resource ImageAssets contenant tous les Handle<Image> :

rust #[derive(Resource)]
struct ImageAssets {
chemin: Handle<Image>,
muraille: Handle<Image>,
guerrier: Handle<Image>,
magicien: Handle<Image>,
// etc.
}

```

2. `load_assets` (Startup) :
   - Utiliser `AssetServer` pour charger toutes les images
   - Stocker dans la Resource

3. Utiliser ces handles dans tous les systèmes de spawn

---

## Étape 11 : Ajustements et polish

**Objectif :** Finaliser les détails et s'assurer que tout fonctionne.

**Tâches :**
1. Vérifier que les chemins d'images sont corrects
2. Ajuster la taille de la fenêtre en fonction de la carte
3. Centrer la caméra sur la carte
4. Ajouter des logs pour déboguer
5. Tester chaque classe et chaque objet
6. Vérifier que les collisions avec les murs fonctionnent
7. S'assurer que la transition entre états est fluide

---

## Étape 12 (Optionnelle) : Conservation du système de combat

**Objectif :** Intégrer le système de combat existant avec la nouvelle interface.

**Modifications :**
1. Garder le fichier `combat.rs` tel quel
2. Créer un état `Combat` dans Bevy
3. Quand le joueur entre en collision avec un ennemi :
   - Transitionner vers l'état `Combat`
   - Afficher une UI de combat ou garder le système terminal
4. Après le combat, retourner à l'état `Map`

---

## Résumé des fichiers à modifier

1. **Cargo.toml** : Ajouter dépendance Bevy
2. **main.rs** : Réécrire complètement avec Bevy
3. **player.rs** : Mettre à jour les 4 classes et leurs stats
4. **item.rs** : Mettre à jour les 4 objets et leurs stats
5. **map.rs** : Adapter pour Bevy (Resource + spawn)
6. **entity.rs** : Potentiellement ajouter des Components Bevy
7. **combat.rs** : Garder pour l'instant (intégration optionnelle)

---

## Structure finale du projet
```

src/
├── main.rs (Bevy App + systèmes graphiques)
├── player.rs (4 classes + stats)
├── item.rs (4 objets + stats)
├── map.rs (Carte + tuiles)
├── entity.rs (Components communs)
└── combat.rs (Logique de combat, optionnel)

public/
├── classes/
│ ├── Guerrier.jpg
│ ├── Magicien.jpg
│ ├── Assassin.jpg
│ └── Bourreau.jpg
├── objets/
│ ├── Armure.jpg
│ ├── Katana.jpg
│ ├── Gants.jpg
│ └── Pendentif.jpg
├── Chemin.jpg
└── Muraille.jpg

Recommandations pour l'IA qui implémente

Procéder étape par étape : Ne pas tout faire d'un coup
Tester après chaque étape : Compiler et exécuter pour vérifier
Commencer par les fondations : Bevy setup → Classes → Carte → Sprites
Utiliser les ECS patterns de Bevy : Components, Systems, Resources
Vérifier les chemins d'assets : S'assurer qu'ils correspondent aux fichiers réels
Garder le code existant : Réutiliser au maximum Stats, Position, etc.
