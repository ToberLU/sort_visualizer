# TODO.md — Progression du projet Sort Visualizer

## Priorités d'apprentissage

### Niveau Beginner

- [ ] Refactoriser pour utiliser des threads (tri + affichage séparés)
  - Utiliser `Arc<RwLock<TableSort>>` pour le partage
  - Thread tri: acquire write lock (écriture)
  - Thread affichage: acquire read lock (lecture seule)
- [ ] Corriger SimpleSort pour qu'il soit 100% correct
- [ ] Ajouter des commentaires éducatifs au code existant
- [ ] Implémenter une fonctionnalité simple (ex: compteur de comparaisons)

### Niveau Intermédiaire

- [ ] Ajouter des contrôles clavier (pause, vitesse, algo suivant)
- [ ] Implémenter Heapsort avec visualisation progressive
- [ ] Créer un trait `SortAlgorithm` commun aux algorithmes
- [ ] Corriger QuickSort pour une visualisation progressive (step-by-step)

### Niveau Avancé

- [ ] Refactoriser avec des iterators et fonctions d'ordre supérieur
- [ ] Ajouter des animations avancées (décalages, fusions)
- [ ] Implémenter MergeSort

---

## Notes du projet

- 100 barres visualization avec Raylib
- current_index = JAUNE, sorted = VERT
- Problème connu: QuickSort et SelectionSort font tout en un frame
