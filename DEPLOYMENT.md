# Guide de Déploiement DigitalOcean - Randomi GO

## Prérequis

1. **Compte DigitalOcean** avec un token d'API
2. **Repository GitHub** avec votre code
3. **doctl CLI** installé sur votre machine
4. **Node.js** installé localement pour la construction du frontend

## Installation de doctl

### Windows (PowerShell)
```powershell
# Télécharger et installer doctl
# https://docs.digitalocean.com/reference/doctl/how-to/install/
```

### macOS
```bash
brew install doctl
```

### Linux
```bash
# Suivre les instructions officielles
# https://docs.digitalocean.com/reference/doctl/how-to/install/
```

## Configuration

### 1. Authentification
```bash
doctl auth init
# Entrez votre token DigitalOcean
```

### 2. Préparer le repository
- Assurez-vous que votre code est sur GitHub
- Notez le nom du repository (ex: `username/ProjetCDA`)

### 3. Variables d'environnement
Créez un fichier `.env` local avec :
```env
BACKEND_SECRET_KEY=votre_clé_secrète_très_longue_et_complexe
```

## Déploiement

### Option 1: Déploiement complet (Recommandé)
```powershell
.\deploy-full.ps1
```
Ce script déploie automatiquement le backend et le frontend ensemble.

### Option 2: Déploiement séparé
```powershell
# Déployer le backend d'abord
.\deploy.ps1

# Puis déployer le frontend
.\deploy-frontend.ps1
```

### Option 3: Manuel
```bash
# 1. Mettre à jour le repository dans do-app.yaml
# 2. Construire le frontend
cd frontend
npm install
npm run build
cd ..

# 3. Déployer
doctl apps create --spec do-app.yaml
```

## Structure du déploiement

### Services déployés :
- **Frontend** : Interface utilisateur (Nginx + Vite)
- **Backend** : API Rust (Actix-web)
- **Base de données** : PostgreSQL 15

### Routes :
- `/` → Frontend
- `/api/*` → Backend
- `/ws/*` → WebSockets

### URLs générées :
- **Frontend** : `https://randomigo-app.ondigitalocean.app`
- **Backend API** : `https://randomigo-app.ondigitalocean.app/api`

## Construction du Frontend

### Localement (pour test)
```powershell
cd frontend
npm install
npm run build
```

### Structure de build
Le frontend est construit avec Vite et génère :
- `dist/` : Fichiers optimisés pour la production
- Assets statiques (JS, CSS, images)
- Configuration nginx pour servir l'application

## Surveillance

### Vérifier le statut
```bash
doctl apps list
doctl apps get <app-id>
```

### Logs
```bash
doctl apps logs <app-id>
doctl apps logs <app-id> --follow
```

### Mise à jour
```bash
doctl apps update <app-id> --spec do-app.yaml
```

## Résolution de problèmes

### Erreurs courantes :

1. **Build frontend échoue** :
   - Vérifiez que Node.js est installé
   - Exécutez `npm install` dans le dossier frontend
   - Vérifiez les erreurs de syntaxe JavaScript

2. **Frontend ne charge pas** :
   - Vérifiez la configuration nginx
   - Consultez les logs du frontend
   - Vérifiez que les assets sont bien servis

3. **Build échoue** :
   - Vérifiez les Dockerfiles
   - Consultez les logs de build

4. **Base de données non connectée** :
   - Vérifiez les variables d'environnement
   - Attendez que la DB soit prête

### Commandes utiles :
```bash
# Redémarrer un service
doctl apps update <app-id> --spec do-app.yaml

# Voir les variables d'environnement
doctl apps get <app-id> --format Spec.Env

# Tester la connectivité
curl https://votre-app.ondigitalocean.app/health

# Tester le frontend
curl https://votre-app.ondigitalocean.app/
```

## Configuration Frontend

### Variables d'environnement
Le frontend utilise la variable `VITE_BACKEND_URL` pour se connecter au backend.

### Configuration Nginx
Le fichier `frontend/nginx.conf` configure :
- Servir les fichiers statiques
- Redirection des API vers le backend
- Support des WebSockets
- Cache pour les assets

## Coûts estimés

- **Basic XXS** : ~$5/mois par service
- **Base de données** : ~$15/mois
- **Total estimé** : ~$25/mois

## Sécurité

- Toutes les variables sensibles sont stockées comme secrets
- HTTPS automatique avec Let's Encrypt
- Isolation des services
- Base de données managée par DigitalOcean

## Support

En cas de problème :
1. Consultez les logs : `doctl apps logs <app-id>`
2. Vérifiez le statut : `doctl apps get <app-id>`
3. Redéployez si nécessaire : `doctl apps update <app-id> --spec do-app.yaml`
4. Testez localement : `cd frontend && npm run dev` 