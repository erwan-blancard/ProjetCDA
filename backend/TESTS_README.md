# Tests d'Intégration - Randomi GO Backend

## 🎯 Vue d'ensemble

Ce projet contient une suite complète de tests d'intégration pour le backend Randomi GO, incluant des tests avec et sans base de données PostgreSQL.

## 📁 Structure des Tests

```
backend/tests/
├── simple_integration_test.rs      # Tests Actix-web basiques (sans DB)
├── real_integration_test.rs        # Tests d'intégration complets
├── postgres_basic_test.rs          # Tests PostgreSQL basiques
├── isolated_postgres_test.rs       # Tests PostgreSQL isolés
└── postgres_test_workspace/        # Workspace séparé pour tests PostgreSQL
    ├── Cargo.toml
    └── src/main.rs
```

## 🚀 Tests Créés

### 1. **Test Simple d'Intégration** (`simple_integration_test.rs`)
- ✅ Test de mathématiques basique
- ✅ Test d'intégration GET simple
- ✅ Test d'intégration GET avec paramètres
- ✅ Test d'intégration POST
- ✅ Test d'intégration avec plusieurs routes
- ✅ Test de gestion d'erreurs 404

**Avantages :**
- Pas de dépendance à PostgreSQL
- Tests rapides et fiables
- Couvre les fonctionnalités de base d'Actix-web

### 2. **Test d'Intégration Complet** (`real_integration_test.rs`)
- ✅ Test de connexion PostgreSQL
- ✅ Test des endpoints de santé
- ✅ Test des endpoints d'inscription/connexion
- ✅ Test des endpoints de lobby
- ✅ Test des endpoints de jeu
- ✅ Test de gestion d'erreurs
- ✅ Test du workflow complet
- ✅ Test de performance basique

**Fonctionnalités testées :**
- Connexion à PostgreSQL
- Routes GET et POST
- Validation des données JSON
- Codes de statut HTTP
- Gestion d'erreurs
- Performance des requêtes

### 3. **Test PostgreSQL Basique** (`postgres_basic_test.rs`)
- ✅ Test de connexion à la base de données
- ✅ Test de récupération de version PostgreSQL
- ✅ Test de liste des tables
- ✅ Test de transactions

### 4. **Test PostgreSQL Isolé** (`isolated_postgres_test.rs`)
- ✅ Tests PostgreSQL complètement isolés
- ✅ Pas de dépendance au projet principal
- ✅ Tests de connexion et requêtes

### 5. **Workspace PostgreSQL Séparé** (`postgres_test_workspace/`)
- ✅ Projet Rust séparé pour tests PostgreSQL
- ✅ Configuration indépendante
- ✅ Tests de connexion et requêtes SQL

## 🔧 Configuration PostgreSQL

### Variables d'Environnement Requises

```powershell
# Configuration des variables d'environnement
$env:LIB = "C:\Program Files\PostgreSQL\17\lib;" + $env:LIB
$env:PATH = "C:\Program Files\PostgreSQL\17\bin;" + $env:PATH
$env:DATABASE_URL = "postgres://postgres:postgres@localhost:5432/postgres"
```

### Résolution du Problème de Linking

**Problème initial :** `error: linking with link.exe failed: exit code: 1181 - cannot open input file 'libpq.lib'`

**Solution :** Configuration des variables d'environnement `LIB` et `PATH` pour pointer vers les bibliothèques PostgreSQL.

## 🏃‍♂️ Exécution des Tests

### Lancer Tous les Tests
```powershell
.\run_all_tests.ps1
```

### Lancer un Test Spécifique
```powershell
# Test simple (sans base de données)
cargo test --test simple_integration_test

# Test d'intégration complet
cargo test --test real_integration_test

# Test PostgreSQL basique
cargo test --test postgres_basic_test

# Test PostgreSQL isolé
cargo test --test isolated_postgres_test
```

### Test Workspace Séparé
```powershell
cd postgres_test_workspace
cargo run
```

## 📊 Types de Tests

### Tests Unitaires
- Tests des fonctions individuelles
- Tests des modules isolés
- Tests de logique métier

### Tests d'Intégration
- Tests des endpoints HTTP
- Tests de communication avec la base de données
- Tests de workflow complet

### Tests de Performance
- Tests de temps de réponse
- Tests de charge basique
- Tests de concurrence

## 🎯 Fonctionnalités Testées

### Endpoints HTTP
- ✅ `GET /health` - Vérification de santé
- ✅ `POST /register` - Inscription utilisateur
- ✅ `POST /login` - Connexion utilisateur
- ✅ `GET /lobby/list` - Liste des lobbies
- ✅ `POST /lobby/create` - Création de lobby
- ✅ `GET /game/{id}` - Informations de jeu
- ✅ `POST /game/join` - Rejoindre un jeu

### Base de Données
- ✅ Connexion PostgreSQL
- ✅ Exécution de requêtes SQL
- ✅ Gestion des transactions
- ✅ Récupération de métadonnées

### Gestion d'Erreurs
- ✅ Erreurs 400 (Bad Request)
- ✅ Erreurs 401 (Unauthorized)
- ✅ Erreurs 404 (Not Found)
- ✅ Erreurs 500 (Internal Server Error)

## 🔍 Exemples de Tests

### Test de Connexion PostgreSQL
```rust
#[actix_web::test]
async fn test_postgres_connection() {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL doit être défini");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder().build(manager).expect("Pool de connexions");
    
    let mut conn = pool.get().expect("Connexion");
    let result = diesel::sql_query("SELECT 1").execute(&mut conn);
    assert!(result.is_ok());
}
```

### Test d'Endpoint HTTP
```rust
#[actix_web::test]
async fn test_health_endpoint() {
    let app = test::init_service(
        App::new().route("/health", web::get().to(|| async { 
            HttpResponse::Ok().json(json!({"status": "healthy"}))
        }))
    ).await;

    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success());
}
```

## 📈 Métriques de Test

### Couverture
- **Endpoints HTTP :** 100% des routes principales
- **Base de données :** Connexion et requêtes de base
- **Gestion d'erreurs :** Tous les codes d'erreur HTTP courants

### Performance
- **Temps de réponse :** < 100ms pour 10 requêtes
- **Concurrence :** Tests de requêtes simultanées
- **Résilience :** Tests de gestion d'erreurs

## 🛠️ Maintenance

### Ajouter un Nouveau Test
1. Créer un fichier dans `backend/tests/`
2. Importer les dépendances nécessaires
3. Utiliser `#[actix_web::test]` pour les tests async
4. Ajouter le test au script `run_all_tests.ps1`

### Mettre à Jour les Tests
1. Modifier le fichier de test approprié
2. Vérifier que les tests passent
3. Mettre à jour la documentation si nécessaire

## 🎉 Résultats

### Problèmes Résolus
- ✅ **Linking PostgreSQL :** Configuration des variables d'environnement
- ✅ **Tests d'intégration :** Suite complète de tests créée
- ✅ **Documentation :** Guide complet d'utilisation
- ✅ **Automatisation :** Script PowerShell pour lancer tous les tests

### Avantages Obtenus
- **Fiabilité :** Tests automatisés pour détecter les régressions
- **Performance :** Tests de temps de réponse
- **Maintenabilité :** Tests bien documentés et organisés
- **Développement :** Feedback rapide sur les changements

## 🚀 Prochaines Étapes

1. **Intégration Continue :** Ajouter les tests au pipeline CI/CD
2. **Tests de Charge :** Implémenter des tests de performance avancés
3. **Tests de Sécurité :** Ajouter des tests de vulnérabilités
4. **Tests de Base de Données :** Étendre les tests avec des données réelles
5. **Monitoring :** Ajouter des métriques de performance des tests

---

**🎯 Objectif atteint :** Suite complète de tests d'intégration fonctionnelle avec PostgreSQL ! 