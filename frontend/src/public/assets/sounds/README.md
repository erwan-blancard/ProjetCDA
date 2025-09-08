# 🎵 Sons pour Randomi GO

## Structure des dossiers

```
sounds/
├── attacks/          # Sons d'attaque
│   ├── attack_light.mp3    # Attaques légères (1-3 dégâts)
│   ├── attack_medium.mp3   # Attaques moyennes (4-7 dégâts)
│   └── attack_heavy.mp3    # Attaques lourdes (8+ dégâts)
├── heals/            # Sons de soin
│   ├── heal_light.mp3      # Soins légers (1-5 HP)
│   └── heal_strong.mp3     # Soins forts (6+ HP)
├── cards/            # Sons de cartes
│   ├── draw.mp3            # Pioche de carte
│   ├── discard.mp3         # Défausse de carte
│   └── play_card.mp3       # Joue une carte
├── dice/             # Sons de dés
│   ├── roll.mp3            # Lancement de dé
│   └── land.mp3            # Atterrissage de dé
└── ui/               # Sons d'interface
    ├── button_click.mp3    # Clic de bouton
    └── notification.mp3    # Notification
```

## Formats recommandés

- **MP3** : Format principal (compatible partout)
- **OGG Vorbis** : Format alternatif (meilleure qualité)
- **WAV** : Qualité parfaite mais plus lourd

## Spécifications techniques

- **Durée** : 0.1 à 0.5 secondes maximum
- **Qualité** : 44.1 kHz, 16-bit minimum
- **Taille** : < 50 KB par fichier
- **Volume** : Normalisé à -12 dB

## Sources recommandées

- **Freesound.org** : Sons libres de droits
- **Zapsplat** : Bibliothèque professionnelle
- **Adobe Audition** : Création de sons synthétiques
- **Audacity** : Édition audio gratuite

## Fallback

Si un fichier audio n'est pas trouvé, le système utilise automatiquement des sons synthétiques générés par le navigateur.

## Test

Pour tester les sons, utilisez la page de test : `test-animations.html`
