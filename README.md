
# 🃏🔥🍃⛰️🌊🎲 Randomi GO - Technologies utilisées

Ce document répertorie les technologies utilisées pour développer Randomi GO, un jeu de cartes stratégique en ligne jouable sur mobile et web.

## 🖊️ Organisation 

### 🎨 Figma
Outil de schématisation visuel<br>Raison: version gratuite limitée, coopération en live, conversion vers html/css...

### 📃 Google Docs
Editeur de texte inclus dans la suite Google<br>Raison: gratuit, fluide, efficace pour résumer, écrire, organiser...

### ⏹️ LucidChart
Outil de schématisation de base de données.<br>Raison: propose une version gratuite, système de coopération, conversion des tables 

### 👾 Discord 
Messagerie texte, appel audio et vidéo.<br>Raison: version gratuite, permet de passer des appels de bonne qualité, partage d'écran...

## 🏭 Production 

### 📝 Visual Studio Code
IDE Complet avec supportant de nombreux langages de programmation<br>Raison: Gratuit, populaire, extensible, permettant d'organiser ses projets.

### 🐙 Github: Versioning du projet
Comprend une branche main, dev, et des branches selon l'avancement du projet (back, front, transactions sql, éléments graphiques...).<br>Raison: Populaire, adapté à la gestion de projet en petit groupe.

### 🐋 Docker: Déploiement et Intégration Continue
Assure un projet propre et sans crash au lancement selon les différentes dépendances.<br>Raison: Populaire et permet un déploiement sur n'importe quel système sans prendre en compte les environnement locaux qui peuvent perturber le lancement.

## 🖥️ Front

### 🔰 HTML/CSS
Langage de balisage pour réaliser le front en version web, faire interagir les différents boutons et zones de texte, en coordination avec JavaScript et ses frameworks.<br>Raison: Langage universel de balisage, il est incontournable dans le développement web frontend.

### ✨ JavaScript
Langage de script pour dynamiser l’app web et travailler avec les frameworks associés.<br>Raison: Langage universel de script orienté web, il est quasi incontournable dans le développement web frontend et constitue la base de nombreux frameworks utilisé pour les sites et applications web (React, Vue.js, Angular.js)

### 🔺 Three.js
Framework JavaScript pour la modélisation 3D et la réalisation du jeu sur navigateur.<br>Raison: Populaire dans le développement de jeu web, permet la gestion de la 3D et l'ajout de divers plugin. Framework intuitif. Nous avons envisagé en premier lieu Babylon.js qui est assez performant dans le milieu du jeu web mais Three.js offre une documentation bien détaillée et intuitive, de plus la gestion de la 3D nous a séduit et nous a convaincu de choisir ce framework pour la partie front de notre projet.

## 🏢 Back

### 🦀 Rust
Langage bas niveau utilisé pour le back et les transactions SQL, rapide, efficace et permettant une gestion simplifiée de la mémoire comparé au C/C++.<br>Raison: Performant, fiable et adapté au développement web. D'autres languages ont été pensé comme le Python, C# OU C++. Rust a été choisi pour le côté découverte d'un nouveau language semblable au C++ dont on a développé certain projet et qui offre des avantages comparé à ce dernier. Python et C# sont efficaces mais semblent moins performant dans le cadre de la création d'un jeu et la rapidité des nombreuses requêtes que le jeu va prévoir.

### ⚙️ Actix
Framework Rust pour gérer les différentes méthodes associées au projet.<br>Raison: Populaire parmis les framework utilisant Rust, bien qu'on aurait pu choisir son concurrent Warp. Actix dispose d'une bonne documentation et permettra une bonne gestion du projet côté back.

## 🗃️ Base de données

### 🦫 DBeaver
Logiciel de gestion de base de données.<br>Raison: Compatible avec de nombreux système de gestion de base de donnée, gratuit et populaire.

### 🐘 PostgreSQL
Système de gestion de base de données utilisé dans le projet.<br>Raison: Populaire et adapté aux petits projets. Nous avons initialement choisi MySQL probablement par habitude mais Postgre semble mieux adapté aux requetes que l'on va devoir faire avec Rust/Actix.

### ⛽ Diesel
ORM Rust adapté pour l’interaction avec la base de données.<br>Raison: populaire parmis les ORM Rust. Il semble aujourd'hui incontournable dans le cadre de notre projet et offre une bonne documentation.

### 🛠 CI/CD
GitHub Actions ou GitLab CI pour automatiser les tests et le déploiement.

## ☁️ Serveur

### 🛸 Fly.io
Serveur gratuit pour un nombre réduit d’utilisateurs, parfait pour tester le projet à moindre coût.<br>Raison: Offre gratuite ou peu onéreuse et scalable.

### 🔒 Firebase Auth
Gestion sécurisée des utilisateurs.<br>Raison: populaire

### 🏨 Hébergement (à déterminer)
Pour le déploiement du site vitrine du projet. Dans un premier temps Wordpress via un nom de domaine actuellement fonctionnel.<br>Raison: On dispose d'un nom de domaine hébergé via Wordpress. Plesk est fourni par la plateforme. Odoo est de plus en plus popualaire et offre une gamme de service varié. Cela reste encore à étudier.

## 🎁 Distribution 

### 📱 Flutter
Permet un déploiement vers les formats d’applications mobile.<br>Raison: Populaire, flexible et utile pour déploiement projet de petite ou moyenne taille. C'est un outil populaire dans la distribution d'application mobile et permet de proposer une application simple à la portée d'un maximum d'utilisateur.

### 🤖 Android Studio
Outil pour déployer sur l'App Store.<br>Raison: Populaire et propose une gamme d'outil complète pour déployer une application vers le Google Play Store.

# 🔗 Liens utiles

## 💡 Idéation

## 📓 Cahier des charges

https://docs.google.com/document/d/1nBWuafzOJ0Ry4JP69ps5df1_py-3cjOuNRErDQIjI9o/edit?usp=sharing

## 🪧 Powerpoint

https://docs.google.com/presentation/d/1ITtxvwF4SyTxi-_HtnsYJE0bk_1MeJzer3F60HAFQyk/edit?usp=sharing



