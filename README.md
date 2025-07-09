# Projet CDA

Repository du projet réalisé en vue de l'obtention du titre "Concepteur Développeur d'Applications".

Ce projet est une version numérique du jeu de carte [Randomi](https://sgave.net/2024/09/28/randomi-vous-avez-carte-blanche-pour-gagner/) créé par [Driss Khelfi](https://github.com/driss-khelfi/), appelée **Randomi GO**.

## Configuration du Projet

Le projet se lance en utilisant Docker Compose: `docker compose up`.

Pour lancer le Frontend seul (sans utiliser Docker Compose):
- installer les dépendances avec `npm install`
- lancer l'application avec Vite: `npx vite`

Pour lancer le Backend seul: `docker compose up backend`.

### Variables d'Environnement

`docker-compose.yml` se sert des variables d'environnements définies dans le fichier `.env` à la racine du projet lors de la création des services.

Un exemple de configuration est fourni dans le fichier `.env.example`.

## Lancement des Tests

Lancer les tests du Frontend en local:
- installer les dépendances: `npm install`
- lancer les tests: `npm run test`

Lancer les tests du backend en local: `cargo test`
