# Système d'Animation et d'Effets Sonores

## Vue d'ensemble

Le système d'animation de Randomi GO utilise Three.js pour créer des effets visuels immersifs et un gestionnaire de sons pour synchroniser les effets audio avec les actions du jeu.

## Architecture

### Composants principaux

1. **AnimationManager** - Gestionnaire central des animations
2. **EffectGenerator** - Générateur d'effets visuels
3. **SoundManager** - Gestionnaire des effets sonores

### Structure des fichiers

```
frontend/src/js/game/animations/
├── AnimationManager.js    # Gestionnaire central
├── EffectGenerator.js    # Générateur d'effets
├── SoundManager.js       # Gestionnaire de sons
└── index.js              # Exports des modules
```

## Utilisation

### Initialisation

```javascript
import { AnimationManager } from './animations/AnimationManager.js';

// Dans le jeu principal
const animationManager = new AnimationManager(scene);
```

### Animation d'une carte jouée

```javascript
// Créer une action
const action = {
    type: 'ATTACK',
    amount: 5,
    element: 'fire'
};

// Jouer l'effet
animationManager.playCardEffect(card, target, action);
```

## Types d'effets

### Effets d'attaque
- **Feu** : Particules rouges explosives
- **Eau** : Particules bleues fluides
- **Terre** : Particules brunes d'impact
- **Air** : Particules blanches tourbillonnantes
- **Foudre** : Particules jaunes électriques
- **Glace** : Particules bleu clair cristallines
- **Ténèbres** : Particules violettes de vide

### Effets de soin
- Particules vertes montantes
- Animation de lueur verte sur le joueur

### Effets de cartes
- **Pioche** : Sphère bleue rotative (élément glace)
- **Défausse** : Cône orange descendant
- **Boost** : Spirale dorée (élément foudre)
- **Vol** : Spirale violette (élément ténèbres)

## Système d'éléments

### Mapping des cartes

Les cartes sont associées à des éléments selon leur ID :

- **Cartes 0-19** : Feu
- **Cartes 20-39** : Eau
- **Cartes 40-59** : Terre
- **Cartes 60-79** : Air
- **Cartes 80-99** : Foudre
- **Cartes 100-119** : Glace
- **Cartes 120-139** : Ténèbres

### Configuration des éléments

Chaque élément a une configuration spécifique :

```javascript
{
    color: { r: 1.0, g: 0.3, b: 0.0 },  // Couleur RGB
    particleSize: 0.12,                   // Taille des particules
    opacity: 0.9,                         // Opacité
    animation: 'explosion',               // Type d'animation
    sound: 'attack_light'                  // Son associé
}
```

## Effets sonores

### Sons d'attaque
- `attack_light` : Dégâts ≤ 3
- `attack_medium` : Dégâts 4-7
- `attack_heavy` : Dégâts ≥ 8
- `water_attack` : Attaque d'eau

### Sons de soin
- `heal_light` : Soin ≤ 5
- `heal_strong` : Soin > 5

### Sons de cartes
- `draw` : Pioche de carte
- `discard` : Défausse de carte
- `play_card` : Jeu de carte générique

### Sons de dés
- `dice_roll` : Lancement de dé
- `dice_land` : Atterrissage de dé

### Sons d'interface
- `victory` : Victoire
- `defeat` : Défaite
- `button_click` : Clic de bouton
- `notification` : Notification

## Fallback synthétique

Si les fichiers audio ne sont pas disponibles, le système utilise des sons synthétiques générés avec l'API Web Audio :

```javascript
// Exemple de son synthétique
playTone(frequency, duration, type)
```

## Intégration avec les événements

### DamagePlayerEvent
- Animation d'attaque avec élément
- Son d'attaque selon les dégâts
- Animation de secousse du joueur
- Mise à jour de l'UI

### HealPlayerEvent
- Animation de soin
- Son de soin selon le montant
- Animation de lueur verte
- Mise à jour de l'UI

### DrawCardEvent
- Animation de pioche
- Son de pioche
- Déplacement vers le joueur

### DiscardCardEvent
- Animation de défausse
- Son de défausse
- Animation de descente

### ThrowDiceEvent
- Son de lancement de dé
- Son d'atterrissage selon le résultat

### PutCardForward
- Animation de lueur selon l'élément
- Son de jeu de carte
- Déplacement vers le centre

## Optimisation des performances

### Nettoyage automatique
- Les effets sont automatiquement supprimés après animation
- Gestion des références pour éviter les fuites mémoire

### Limitation des particules
- Nombre maximum de particules par effet
- Réduction automatique selon l'intensité

### Gestion des erreurs
- Fallback vers les sons synthétiques
- Gestion gracieuse des erreurs de chargement

## Tests

### Fichiers de test
- `test-all-effects.html` : Test de tous les effets
- `test-animation-integration.html` : Test d'intégration

### Commandes de test
```javascript
// Test d'un effet spécifique
testEffect('ATTACK', 5);

// Test de tous les effets
testAllEffects();

// Nettoyage
clearEffects();
```

## Personnalisation

### Ajout d'un nouvel élément
1. Ajouter la configuration dans `getElementConfig()`
2. Créer l'animation dans `animateElementEffect()`
3. Ajouter le mapping dans `getElementMapping()`

### Ajout d'un nouveau son
1. Ajouter le fichier audio dans `preloadSounds()`
2. Créer la méthode de lecture
3. Intégrer dans les événements appropriés

## Dépannage

### Problèmes courants
- **Sons non joués** : Vérifier les chemins des fichiers
- **Effets non visibles** : Vérifier l'initialisation de la scène
- **Performances** : Réduire le nombre de particules

### Logs de débogage
Le système affiche des logs détaillés pour le débogage :
- `🎬 Animation: Playing card effect`
- `💥 DamagePlayerEvent: X dégâts`
- `💚 HealPlayerEvent: X HP`
- `🎴 DrawCardEvent: pioche une carte`
