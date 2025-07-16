# Projet CDA

Repository du projet réalisé en vue de l'obtention du titre "Concepteur Développeur d'Applications".

Les phases d'[idéation](#idéation) et de [conception](#conception) du projet sont exposées dans ce document.
Le cahier des charges du projet est quand à lui disponible sur Google Drive [ici](https://docs.google.com/document/d/1q1_h__Q5QH4UED8aIS-Iv8AhZG0YckE_t92xyG4v_JA/edit?usp=sharing).

Le dossier [docs](./docs/) contient des documents annexes, comme par exemple un schéma de la structure envisagée pour la base de données.

## Table des matières
- [Semaine 1](#semaine-1)
    - [Idéation](#idéation)
    - [Conception](#conception)
        - [Frontend](#frontend)
        - [Backend](#backend)
            - [Base de données](#base-de-données)
            - [API Web](#api-web)
            - [Serveur de jeu](#serveur-de-jeu)
- [Semaine 2](#semaine-2)
- [Semaine 3](#semaine-3)
- [Semaine 4](#semaine-4)


# Semaine 1

Objectifs:
- Idéation
- Cahier des charges
- Conception

## Idéation

Nous avons décidé de créer une version numérique du jeu de carte [Randomi](https://sgave.net/2024/09/28/randomi-vous-avez-carte-blanche-pour-gagner/) créé par [Driss Khelfi](https://github.com/driss-khelfi/), que l'on appellera **Randomi GO**.

**Randomi GO** se présentera comme une Progressive Web App afin de permettre à tout les appareils disposant d'un navigateur Web de jouer au jeu.

Plusieurs autres idées de projets ont été envisagé:
- Un "AdBlock" pour les stations de radio.
- Un GPS où l'utilisateur pourrait filmer ce qu'il y a devant lui et visualiser l'itinéraire grâce à la Réalité Augmentée, et potentiellement des informations sur les enseignes à proximité.
- Un Scanner d'images pour vérifier les droits d'auteurs qui lui sont associés.
- Une plateforme de création de mèmes dans des mini-jeux en groupe avec un système de vote.

Nous avons choisi de retenir **Randomi GO** pour le challenge technique que ce projet représente, et également pour nous donner l'occasion de découvrir le langage [Rust](https://www.rust-lang.org/) pour la gestion des parties en ligne et les intéractions avec une base de données à travers une API Web.


## Conception

Comme énoncé, **Randomi GO** se présentera comme un jeu de cartes en ligne sous la forme d'une Progressive Web App.

Les joueurs pourront s'affronter dans des parties en ligne grâce à un système de matchmaking.

Le projet sera divisé en deux parties: le [client](#frontend) et le [serveur](#backend).

### Frontend

Le Frontend sera réalisé en utilisant la bibliothèque [`Three.js`](https://threejs.org/).

`Three.js` sera utilisé pour afficher des éléments en 3D (cartes notamment) et gérer les animations, mais aussi les animations graphiques, les effets sonores et les particules, dans le but de proposer une expérience de jeu plus immersive.

Nous aurions également pu utiliser [`Babylon.js`](https://www.babylonjs.com/), qui est un framework plus axé sur le développement de jeux web en 3D, mais au vu de sa complexité et de celle du projet, nous avons plutôt opté pour `Three.js` pour avoir quelque chose de plus simple.

### Backend

Le Backend sera composé d'une base de données, d'une API Web et d'un (ou plusieurs) Serveur de jeu.
La base de données, l'API et le Serveur seront containerisées avec Docker.

#### Base de données

Utilisation de **Postgres** comme système de base de données.
Elle sera utilisée pour stocker les informations relatives aux joueurs (cartes, niveau, objets, etc.).

La structure de la base de données est détaillée [ici](./docs/Structure%20BDD%20Randomi%20GO.pdf).

#### API Web

L'API sera réalisé en **Rust** avec le framework web [`Actix`](https://actix.rs/), couplé avec [`Diesel`](https://diesel.rs) comme ORM pour manipuler les objects dans la base de données.

`Actix` et `Diesel` semblaient être des choix évidents: `Actix` étant l'un des plus populaires, et `Diesel` lui est complémentaire.

Elle sera utilisée pour:
- la création et gestion des comptes
- la gestion d'une liste d'amis
- l'inventaire des joueurs (cartes, objets cosmétiques, etc.) et l'achats d'objets
- le matchmaking

#### Serveur de jeu

**(TODO)**

Le Serveur sera également réalisé en Rust et utilisera WebSocket pour les communications client/serveur.


# Semaine 2

Objectifs:
- Décision des technos et features à garder pour la durée du projet
- Mise en place de la base de données et modélisation
- Création du Backend et containerisation

## Configuration et Exécution

#### Variables d'environnement

Les variables d'environnement utilisées dans [**docker-compose.yml**](./backend/docker-compose.yml) doivent être définies dans un fichier **.env**.

Le fichier devra contenir les variables suivantes:

```
DATABASE_HOST=randomi-db    # same name as db service
DATABASE_PORT=5432
POSTGRES_DB=randomi
POSTGRES_USER=backend
POSTGRES_PASSWORD=root
```

Ces variables seront utilisées dans le **docker-compose.yml** pour déterminer l'URL avec laquelle l'API pourra se connecter à la base de données.

#### Exécution des services

Pour lancer les services du Backend, il suffit d'utiliser `docker compose up` dans le répertoire [backend](./backend/):

```
cd backend
docker compose up
```

# Semaine 3



# Semaine 4
