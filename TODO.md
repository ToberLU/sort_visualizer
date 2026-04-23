# TODO.md — Progression du projet Sort Visualizer

## Priorités d'apprentissage

### Niveau Beginner

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
