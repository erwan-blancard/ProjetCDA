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