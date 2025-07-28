# Guide de Déploiement Frontend - Randomi GO

## ✅ Configuration terminée

Le frontend de Randomi GO est maintenant configuré pour le déploiement sur DigitalOcean Apps Platform.

## 📁 Fichiers modifiés/créés

### 1. `frontend/Dockerfile`
- ✅ Corrigé pour installer toutes les dépendances (pas seulement production)
- ✅ Configuration multi-stage optimisée

### 2. `frontend/nginx.conf`
- ✅ Configuration nginx pour servir l'application SPA
- ✅ Redirection des API vers le backend
- ✅ Support des WebSockets
- ✅ Cache optimisé pour les assets

### 3. `frontend/vite.config.js`
- ✅ Configuration Vite pour la production
- ✅ Ports configurés pour le développement et preview

### 4. `frontend/.do/app.yaml`
- ✅ Configuration spécifique pour DigitalOcean Apps
- ✅ Variables d'environnement pour le backend

### 5. Scripts de déploiement
- ✅ `deploy-frontend.ps1` - Déploiement frontend seul
- ✅ `deploy-full.ps1` - Déploiement backend + frontend

## 🚀 Comment déployer

### Option 1: Déploiement complet (Recommandé)
```powershell
.\deploy-full.ps1
```

### Option 2: Déploiement séparé
```powershell
# 1. Déployer le backend d'abord
.\deploy.ps1

# 2. Déployer le frontend
.\deploy-frontend.ps1
```

### Option 3: Manuel
```powershell
# 1. Construire le frontend
cd frontend
npm install
npm run build
cd ..

# 2. Déployer
doctl apps create --spec do-app.yaml
```

## 🔧 Test local

Pour tester le frontend localement :

```powershell
cd frontend
npm install
npm run dev
```

Le frontend sera disponible sur `http://localhost:3000`

## 📊 URLs après déploiement

- **Frontend** : `https://randomigo-app.ondigitalocean.app`
- **Backend API** : `https://randomigo-app.ondigitalocean.app/api`

## 🔍 Vérification

Après déploiement, vérifiez :

1. **Frontend accessible** : Visitez l'URL du frontend
2. **API fonctionnelle** : Testez `/api/health`
3. **Assets chargés** : Vérifiez que les images et CSS se chargent
4. **Logs** : `doctl apps logs <app-id>`

## 🐛 Résolution de problèmes

### Build échoue
```powershell
cd frontend
npm install
npm run build
```

### Frontend ne charge pas
- Vérifiez les logs : `doctl apps logs <app-id>`
- Testez l'URL directement
- Vérifiez la configuration nginx

### API non accessible
- Vérifiez que le backend est déployé
- Testez l'URL de l'API directement
- Vérifiez les variables d'environnement

## 📝 Notes importantes

1. **Node.js requis** : Assurez-vous d'avoir Node.js installé localement
2. **Repository GitHub** : Le code doit être sur GitHub
3. **doctl configuré** : Authentifiez-vous avec `doctl auth init`
4. **Variables d'environnement** : Configurez `VITE_BACKEND_URL` pour pointer vers votre backend

## 🎯 Prochaines étapes

1. Déployez avec `.\deploy-full.ps1`
2. Testez l'application complète
3. Configurez un domaine personnalisé si nécessaire
4. Mettez en place le monitoring

---

**Status** : ✅ Prêt pour le déploiement 