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


# Semaine 5



# Semaine 6

Lancer les tests du frontend en local: 
    -npm install
    -npm run test
        -accepter d'installer la dépendance "jsdom"
    -retaper: npm run test

Lancer les tests du backend en local, en étant sur la racine du projet: 
    -docker build --no-cache -f backend/Dockerfile.dev -t backend-dev ./backend
    -docker run -it --rm backend-dev
    (-cargo fix --bin backend --tests)
    -cargo test


# Tests d'intégration - Randomi GO Backend

Ce dossier contient les tests d'intégration pour le backend de Randomi GO.

## Structure des tests

### 1. `simple_integration_test.rs`
Tests de base pour vérifier que l'application Actix-web fonctionne correctement :
- ✅ Test de mathématiques basique
- ✅ Test d'intégration GET simple
- ✅ Test d'intégration GET avec paramètres
- ✅ Test d'intégration POST
- ✅ Test d'intégration avec plusieurs routes

### 2. `real_routes_test.rs`
Tests des vraies routes de l'application :
- ✅ Test de la route `/register`
- ✅ Test de la route `/login`
- ✅ Test des routes d'account (`/account/profile`)
- ✅ Test des routes de lobby (`/lobby/list/0`)
- ✅ Test des routes SSE (`/events`)
- ✅ Test que toutes les routes sont configurées
- ✅ Test des routes inconnues (404)

### 3. `authenticated_routes_test.rs`
Tests des routes authentifiées avec JWT :
- ✅ Test de création de JWT token
- ✅ Test de validation de JWT token
- ✅ Test de routes authentifiées avec token valide
- ✅ Test de routes authentifiées sans token
- ✅ Test de `/account/profile` avec authentification
- ✅ Test de `/lobby/create` avec authentification
- ✅ Test de `/account/requests` avec authentification
- ✅ Test des headers CORS

### 4. `error_handling_test.rs`
Tests de gestion d'erreurs et cas limites :
- ✅ Test avec JSON invalide
- ✅ Test avec champs manquants
- ✅ Test avec méthodes HTTP incorrectes
- ✅ Test avec paramètres de chemin invalides
- ✅ Test avec payload très large
- ✅ Test avec caractères spéciaux
- ✅ Test avec champs vides
- ✅ Test avec headers malformés
- ✅ Test de requêtes concurrentes
- ✅ Test du format des réponses d'erreur

## Comment lancer les tests

### Lancer tous les tests d'intégration
```bash
cd backend
cargo test --test "*_test"
```

### Lancer un test spécifique
```bash
# Tests simples
cargo test --test simple_integration_test

# Tests des vraies routes
cargo test --test real_routes_test

# Tests des routes authentifiées
cargo test --test authenticated_routes_test

# Tests de gestion d'erreurs
cargo test --test error_handling_test
```

### Utiliser le script PowerShell
```powershell
cd backend
.\run_integration_tests.ps1
```

## Ce que testent ces tests

### Tests de base
- ✅ Démarrage de l'application Actix-web
- ✅ Configuration des routes
- ✅ Réponses HTTP correctes
- ✅ Codes de statut appropriés

### Tests des routes
- ✅ Routes d'authentification (`/register`, `/login`)
- ✅ Routes d'account (`/account/profile`, `/account/requests`)
- ✅ Routes de jeu (`/lobby/create`, `/lobby/list`, `/lobby/find`)
- ✅ Routes SSE (`/events`)
- ✅ Gestion des erreurs 401 (non authentifié)
- ✅ Gestion des erreurs 404 (route inexistante)
- ✅ Gestion des erreurs 405 (méthode non autorisée)

### Tests d'authentification
- ✅ Création et validation de JWT tokens
- ✅ Middleware d'authentification
- ✅ Headers d'autorisation
- ✅ Gestion des tokens invalides

### Tests de robustesse
- ✅ JSON malformé
- ✅ Données manquantes
- ✅ Payloads trop grands
- ✅ Caractères spéciaux
- ✅ Requêtes concurrentes
- ✅ Headers malformés

## Avantages de ces tests

1. **Couverture complète** : Testent toutes les routes principales
2. **Robustesse** : Vérifient la gestion d'erreurs
3. **Authentification** : Testent le système JWT
4. **Indépendance** : Ne dépendent pas de la base de données
5. **Rapidité** : Tests rapides et fiables
6. **Maintenabilité** : Faciles à comprendre et modifier

## Notes importantes

- Ces tests ne nécessitent pas de base de données PostgreSQL
- Ils testent l'intégration entre les composants d'Actix-web
- Certains tests peuvent échouer si la base de données n'est pas configurée (c'est normal)
- Les warnings de compilation n'empêchent pas les tests de fonctionner

## Ajout de nouveaux tests

Pour ajouter de nouveaux tests d'intégration :

1. Créez un nouveau fichier `nom_du_test.rs` dans le dossier `tests/`
2. Utilisez la structure suivante :
```rust
use actix_web::{test, App, HttpResponse, Responder};
use crate::routes;

#[actix_web::test]
async fn test_nouvelle_fonctionnalite() {
    let app = test::init_service(
        App::new()
            .configure(routes::module::configure_routes)
    ).await;

    let req = test::TestRequest::get()
        .uri("/nouvelle-route")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
```

3. Ajoutez le test au script PowerShell si nécessaire