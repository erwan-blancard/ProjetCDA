
# 🃏🔥🍃⛰️🌊🎲 Randomi GO - Technologies utilisées

Ce document répertorie les technologies utilisées pour développer Randomi GO, un jeu de cartes stratégique en ligne jouable sur mobile et web.

## 🖊️ Organisation 

### 🎨 Figma
Outil de schématisation visuel
Raison: verision gratuite limitée, coopération en live, conversion vers html/css...

### 📃 Google Docs
Editeur de texte inclus dans la suite Google
Raison: gratuit, fluide, efficace pour résumer, écrire

### ⏹️ LucidChart
Outil de schématisation de base de données.
Raison: propose une version gratuite, système de coopération, conversion des tables 

### 👾 Discord 
Messagerie texte, appel audio et vidéo.
Raison: version gratuite, permet de passer des appels de bonne qualité, partage d'écran...

## 🏭 Production 

### 📝 Visual Studio Code
IDE Complet avec supportant de nombreux langage de programmation
Raison: Gratuit, populaire, extensible, permettant d'organiser ses projets.

### 🐙 Github: Versioning du projet
Comprend une branche main, dev, et des branches selon l'avancement du projet (back, front, transactions sql, éléments graphiques...).
Raison: Populaire, adapté à la gestion de projet en petit groupe.

### 🐋 Docker: Déploiement et Intégration Continue
Assure un projet propre et sans crash au lancement selon les différentes dépendances.
Raison: Populaire et permet un déploiement sur n'importe quel système sans prendre en compte les environnement locaux qui peuvent perturber le lancement.

## 🖥️ Front

### 🔰 HTML/CSS
Langage de balisage pour réaliser le front en version web, faire interagir les différents boutons et zones de texte, en coordination avec JavaScript et ses frameworks.
Raison: Langage universel de balisage.

### ✨ JavaScript
Langage de script pour dynamiser l’app web et travailler avec les frameworks associés.
Raison: Langage universel de script orienté web.

### 🔺 Three.js
Framework JavaScript pour la modélisation 3D et la réalisation du jeu sur navigateur.
Raison: Populaire dans le développement de jeu web, permet la gestion de la 3D et l'ajout de divers plugin. Framework intuitif.

## 🏢 Back

### 🦀 Rust
Langage bas niveau utilisé pour le back et les transactions SQL, rapide, efficace et permettant une gestion simplifiée de la mémoire comparé au C/C++.
Raison: Performant, fiable et adapté au développement web.

### ⚙️ Actix
Framework Rust pour gérer les différentes méthodes associées au projet.
Raison: Populaire parmis les framework utilisant Rust

## 🗃️ Base de données

### 🦫 DBeaver
Logiciel de gestion de base de données.
Raison: Compatible avec de nombreux système de gestion de base de donnée, gratuit et populaire.

###🐬 MySQL
Système de gestion de base de données utilisé dans le projet.
Raison: Populaire et adapté aux petits projets.

### ⛽ Diesel
ORM Rust adapté pour l’interaction avec la base de données.
Raison: populaire parmis les ORM Rust

### 🛠 CI/CD
GitHub Actions ou GitLab CI pour automatiser les tests et le déploiement.

## ☁️ Serveur

### 🛸 Fly.io
Serveur gratuit pour un nombre réduit d’utilisateurs, parfait pour tester le projet à moindre coût.
Raison: Offre gratuite ou peu onéreuse et scalable.

### 🔒 Firebase Auth
Gestion sécurisée des utilisateurs.
Raison: populaire

### 🏨 Hébergement (WordPress, Plesk ou Odoo à déterminer)
Pour le déploiement du site vitrine du projet. Dans un premier temps Wordpress via un nom de domaine actuellement fonctionnel.
Raison: On dispose d'un nom de domaine hébergé via Wordpress. Plesk est fourni par la plateforme. Odoo est de plus en plus popualaire et offre une gamme de service varié. Cela reste encore à étudier.

## 🎁 Distribution 

### 📱 Flutter
Permet un déploiement vers les formats d’application mobile.
Raison: Populaire, flexible et utile pour déploiement projet de petite ou moyenne envergure.

### 🤖 Android Studio
Outil pour déployer sur l'App Store.
Raison: Populaire et propose une gamme d'outil complet pour déployer une application vers le Google Play Store.

