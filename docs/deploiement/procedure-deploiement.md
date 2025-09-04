# Procédure de Déploiement - Randomi GO

## 1. Prérequis et Préparation

### 1.1 Vérification des Environnements

Avant tout déploiement, vérifier que tous les environnements sont opérationnels :

```bash
# Vérification de l'environnement de développement
docker compose up -d
docker compose ps

# Vérification des tests
npm run test          # Frontend
cargo test            # Backend
```

### 1.2 Gestion des Variables d'Environnement

Les variables d'environnement sont gérées de manière sécurisée :

- **Développement** : Fichier `.env` local
- **Tests** : Variables GitHub Actions
- **Production** : Variables DigitalOcean App Platform

Variables critiques :
- `DATABASE_URL` : Connexion PostgreSQL
- `BACKEND_SECRET_KEY` : Clé de chiffrement
- `POSTGRES_PASSWORD` : Mot de passe base de données
- `REDIS_URL` : Connexion Redis

## 2. Pipeline CI/CD

### 2.1 Workflow GitHub Actions

Le pipeline s'exécute automatiquement à chaque push sur la branche `main` :

1. **Tests Automatisés**
   - Tests unitaires backend (Rust)
   - Tests unitaires frontend (JavaScript)
   - Tests d'intégration
   - Validation de la qualité du code

2. **Build des Images**
   - Construction image backend Docker
   - Construction image frontend Docker
   - Validation des images

3. **Déploiement**
   - Déploiement automatique sur DigitalOcean
   - Mise à jour de la base de données
   - Vérification de la santé de l'application

### 2.2 Configuration des Jobs

```yaml
# .github/workflows/deploy.yml
name: Deploy to Production

on:
  push:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Run Backend Tests
        run: cargo test
      - name: Run Frontend Tests
        run: npm test

  build:
    needs: test
    runs-on: ubuntu-latest
    steps:
      - name: Build Backend Image
        run: docker build -t randomi-backend ./backend
      - name: Build Frontend Image
        run: docker build -t randomi-frontend ./frontend

  deploy:
    needs: build
    runs-on: ubuntu-latest
    steps:
      - name: Deploy to DigitalOcean
        uses: digitalocean/action-doctl@v2
        with:
          token: ${{ secrets.DIGITALOCEAN_ACCESS_TOKEN }}
```

## 3. Environnements de Tests

### 3.1 Tests d'Intégration (SIT)

**Objectif** : Valider l'intégration entre les composants

**Environnement** :
- Conteneurs Docker isolés
- Base de données de test
- Données de test préconfigurées

**Procédure** :
```bash
# Lancement de l'environnement SIT
docker compose -f docker-compose.test.yml up -d

# Exécution des tests d'intégration
npm run test:integration
cargo test --test integration

# Nettoyage
docker compose -f docker-compose.test.yml down
```

### 3.2 Tests d'Acceptation Client (UAT)

**Objectif** : Validation fonctionnelle par les utilisateurs finaux

**Environnement** :
- Serveur de staging DigitalOcean
- Données de production anonymisées
- Interface utilisateur complète

**Procédure** :
1. Déploiement automatique sur staging
2. Notification aux testeurs
3. Collecte des retours
4. Validation avant production

### 3.3 Tests de Production

**Objectif** : Vérification post-déploiement

**Vérifications** :
- Santé de l'application
- Performance des endpoints
- Intégrité de la base de données
- Certificats SSL

## 4. Procédure de Déploiement

### 4.1 Déploiement Automatique

Le déploiement est entièrement automatisé via GitHub Actions :

1. **Validation du Code**
   - Tests unitaires
   - Tests d'intégration
   - Analyse statique du code

2. **Construction des Images**
   - Build des images Docker optimisées
   - Scan de sécurité des images
   - Push vers le registry

3. **Déploiement sur Production**
   - Mise à jour des conteneurs
   - Migration de base de données
   - Vérification de la santé

### 4.2 Déploiement Manuel (Cas d'urgence)

En cas de problème nécessitant un déploiement manuel :

```bash
# Connexion au serveur de production
ssh root@randomi-go.com

# Arrêt des services
docker compose down

# Mise à jour du code
git pull origin main

# Reconstruction des images
docker compose build --no-cache

# Redémarrage des services
docker compose up -d

# Vérification
docker compose ps
curl -f http://localhost:8080/health
```

### 4.3 Rollback

En cas de problème post-déploiement :

```bash
# Retour à la version précédente
git checkout HEAD~1

# Reconstruction et redéploiement
docker compose build
docker compose up -d

# Vérification
docker compose ps
```

## 5. Gestion des Évolutions

### 5.1 Migrations de Base de Données

Les migrations sont gérées automatiquement via Diesel :

```bash
# Génération d'une nouvelle migration
diesel migration generate nom_migration

# Application des migrations
diesel migration run

# Rollback d'une migration
diesel migration revert
```

### 5.2 Mises à Jour de Dépendances

**Backend (Rust)** :
```bash
# Mise à jour des dépendances
cargo update

# Vérification de la compatibilité
cargo check
cargo test
```

**Frontend (JavaScript)** :
```bash
# Mise à jour des dépendances
npm update

# Vérification de la compatibilité
npm run build
npm test
```

## 6. Monitoring et Maintenance

### 6.1 Surveillance Continue

- **Prometheus** : Collecte des métriques
- **Grafana** : Tableaux de bord
- **AlertManager** : Alertes automatiques

### 6.2 Maintenance Préventive

- Sauvegardes quotidiennes
- Mise à jour de sécurité
- Nettoyage des logs
- Optimisation des performances

### 6.3 Procédures d'Urgence

1. **Incident Critique** : Rollback immédiat
2. **Problème de Performance** : Scale up temporaire
3. **Attaque Sécurité** : Isolation et analyse

## 7. Documentation et Formation

### 7.1 Documentation Technique

- Architecture système
- Procédures de déploiement
- Troubleshooting
- API documentation

### 7.2 Formation de l'Équipe

- Formation aux outils CI/CD
- Procédures d'urgence
- Bonnes pratiques de déploiement

## 8. Veille Technologique

### 8.1 Sources de Veille

- **Sécurité** : CVE, advisories
- **Performance** : Benchmarks, optimisations
- **Nouvelles Technologies** : Docker, Kubernetes, etc.

### 8.2 Mise à Jour de la Documentation

La documentation est mise à jour régulièrement pour refléter :
- Nouvelles procédures
- Évolutions technologiques
- Retours d'expérience
- Améliorations continues 