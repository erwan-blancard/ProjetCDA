# 🚀 Guide Serveur de Développement

## Problème identifié

L'erreur `file:///frontend/src/public/assets/sounds/attacks/hit_sfx.wav` indique que vous ouvrez les fichiers HTML directement dans le navigateur au lieu de les servir via un serveur HTTP.

## Solutions

### 1. 🐍 Python (Recommandé)

Si Python est installé sur votre système :

```bash
# Naviguer vers le dossier frontend
cd frontend

# Démarrer le serveur
python -m http.server 8000

# Ou pour Python 3
python3 -m http.server 8000
```

Puis ouvrir : `http://localhost:8000/test-simple-sounds.html`

### 2. 🟢 Node.js

Si Node.js est installé :

```bash
# Installer http-server globalement (une seule fois)
npm install -g http-server

# Naviguer vers le dossier frontend
cd frontend

# Démarrer le serveur
http-server -p 8000

# Ou avec npx (sans installation)
npx http-server -p 8000
```

Puis ouvrir : `http://localhost:8000/test-simple-sounds.html`

### 3. 🔧 VS Code (Extension)

1. Installer l'extension "Live Server" dans VS Code
2. Clic droit sur `test-simple-sounds.html`
3. Sélectionner "Open with Live Server"

### 4. 🌐 Autres serveurs

#### PHP
```bash
cd frontend
php -S localhost:8000
```

#### Ruby
```bash
cd frontend
ruby -run -e httpd . -p 8000
```

## Vérification

Une fois le serveur démarré, vous devriez voir :
- ✅ Les sons se chargent correctement
- ✅ Pas d'erreurs `file://` dans la console
- ✅ Les animations et sons fonctionnent ensemble

## Structure recommandée

```
frontend/
├── test-simple-sounds.html      # Test des sons
├── test-file-protocol.html      # Test protocole file
├── test-final-animations.html   # Test complet
└── src/
    └── public/
        └── assets/
            └── sounds/
                ├── attacks/
                ├── heals/
                ├── cards/
                ├── dice/
                └── ui/
```

## Dépannage

### Erreur "Port already in use"
```bash
# Utiliser un autre port
python -m http.server 8001
# ou
http-server -p 8001
```

### Erreur "Permission denied"
```bash
# Sur Linux/Mac, utiliser sudo si nécessaire
sudo python -m http.server 8000
```

### Erreur "Command not found"
- Vérifier que Python/Node.js est installé
- Vérifier que les commandes sont dans le PATH

## Avantages du serveur HTTP

1. **Sécurité** : Évite les restrictions CORS
2. **Performance** : Chargement plus rapide des ressources
3. **Compatibilité** : Fonctionne avec tous les navigateurs
4. **Développement** : Hot reload et debugging améliorés
