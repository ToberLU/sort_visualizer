# Sort Visualizer

Visualiseur d'algorithmes de tri écrit en Rust, conçu comme projet d'apprentissage.

## Objectif

Apprendre Rust en s'amusant avec des visualisations graphiques d'algorithmes de tri.

## Stack technique

- **Rust** (edition 2024)
- **Raylib 5.5.1** — bibliothèque graphique légère pour le rendu 2D/3D
- **rand 0.10.1** — mélange aléatoire du tableau initial

## Algorithmes implémentés

| Algorithme | Fonction | État |
|------------|----------|------|
| **SimpleSort** (Bubble Sort) | `sort_step()` | ✅ Fonctionnel |
| **SelectionSort** | `selection_sort()` | ✅ Fonctionnel |
| **QuickSort** | `quicksort()` + `partition()` | ✅ Fonctionnel |
| **Heapsort** | `heapsort()` | ❌ Stub (non implémenté) |

## Architecture

```
src/
├── main.rs           # Point d'entrée + boucle de rendu Raylib
├── mod.rs            # Déclaration du module sort
└── sort/
    └── sorting.rs    # Implémentation des algorithmes de tri
```

### Structure des données — `TableSort`

```rust
table: [i32; SIZE]        // Le tableau à trier (100 éléments)
current_index: usize     // Élément actuellement examiné
pivot: usize              // Frontière du tri (varie selon l'algo)
min_index: usize          // Indice du minimum (selection sort)
sorted: bool              // Drapeau de terminaison
sort_type: SortingAlgo    // Algorithme actif
```

### Boucle de visualisation (main.rs:44-69)

- **100 barres** représentent les valeurs du tableau
- **BLANC** — état par défaut
- **JAUNE** — élément actuellement comparé (`current_index`)
- **VERT** — éléments triés

### Constantes de rendu

| Constante | Valeur | Description |
|-----------|--------|-------------|
| `SIZE` | 100 | Nombre d'éléments |
| `FPS` | 100 | Images par seconde |
| `WIDTH` | 1900 | Largeur de la fenêtre |
| `HEIGHT` | 1000 | Hauteur de la fenêtre |

## Problèmes connus

### QuickSort non-visualisable

`quicksort()` effectue le tri complet en un seul frame au lieu de progresser étape par étape. Pour une visualisation correcte, l'algorithme devrait muter son état progressivement.

### SelectionSort incomplet

Même problème que QuickSort — pas de progression frame par frame visible.

## Prochaines étapes d'apprentissage (suggérées)

1. Corriger QuickSort pour une visualisation progressive
2. Implémenter Heapsort
3. Ajouter des contrôles clavier (pause, vitesse, algo suivant)
4. Ajouter des statistiques (comparaisons, échanges)
5. Implémenter MergeSort avec visualisation de la fusion

## Compilation et exécution

```bash
cargo run
```

## Licence

Projet d'apprentissage personnel.