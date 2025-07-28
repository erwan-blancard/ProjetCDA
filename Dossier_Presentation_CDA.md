# Dossier de Présentation - Titre Professionnel Concepteur Développeur d'Applications

**Projet : Randomi GO - Jeu de cartes en ligne**

---

## Sommaire

I. Présentation
- Objectifs et cibles du projet
- Description du projet
- Idéation
- Cahier des charges

II. Conception
- Organisation & Méthodologie
- Maquettage
- Design et charte graphique
- Accessibilité / Responsivité
- Architecture logicielle
- Conception de la base de données
- Routes et endpoints
- Contrôleurs / logique métier
- API et WebServices
- Backend (fonctionnement, choix, logique)
- Frontend (frameworks, composants...)

III. Production
- Authentification / gestion des utilisateurs
- Sécurité (validation, droits, protection)
- Gestion des erreurs / logs
- Tests (unitaires, fonctionnels, manuels...)
- Conteneurisation (Docker, images)
- Déploiement (CI/CD, hébergement)

IV. Analyse et synthèse
- Versionning
- Documentation (technique & utilisateur)
- Valeur ajoutée du projet
- Obstacles rencontrés
- Bénéfices personnels et techniques
- Perspectives d'amélioration
- Autres projets réalisés
- Conclusion + Remerciements
- Annexes

---

## I. Présentation

### Objectifs et cibles du projet

Le projet Randomi GO s'est fixé plusieurs objectifs ambitieux qui ont guidé l'ensemble du développement. L'objectif principal était de créer une version numérique du jeu de cartes Randomi sous forme de Progressive Web App, permettant ainsi aux joueurs de s'affronter en ligne avec un système de matchmaking performant. Cette approche offrait la possibilité de découvrir et maîtriser le langage Rust pour le développement backend, tout en implémentant une architecture moderne avec conteneurisation Docker. Le développement d'une interface utilisateur immersive avec Three.js constituait également un défi technique majeur pour offrir une expérience de jeu exceptionnelle.

Les cibles identifiées pour ce projet étaient multiples et complémentaires. Les joueurs de jeux de cartes en ligne représentaient le public principal, attirés par l'aspect social et compétitif du jeu. L'accessibilité sur tous les appareils disposant d'un navigateur web était un critère essentiel pour maximiser l'audience potentielle. Enfin, la communauté de développeurs intéressés par Rust et les Progressive Web Apps constituait une cible secondaire mais importante pour la diffusion des bonnes pratiques et l'amélioration continue du projet.

### Description du projet

Randomi GO se présente comme une adaptation numérique innovante du jeu de cartes Randomi créé par Driss Khelfi. Cette Progressive Web App révolutionne l'expérience de jeu traditionnelle en permettant aux joueurs de s'affronter en ligne avec d'autres participants du monde entier. L'interface 3D immersive développée avec Three.js offre une expérience visuelle exceptionnelle, tandis que le système d'authentification JWT garantit la sécurité des comptes utilisateurs. La base de données PostgreSQL assure la persistance des données de jeu, et l'API REST développée en Rust avec Actix-web fournit des performances exceptionnelles. L'ensemble du système est conteneurisé avec Docker pour faciliter le déploiement et la maintenance, et l'application est hébergée sur DigitalOcean pour une disponibilité optimale.

L'architecture technique du projet repose sur des technologies modernes et performantes. Le frontend utilise HTML5, CSS3 et JavaScript avec Three.js pour les éléments 3D, créant ainsi une interface utilisateur riche et interactive. Le backend développé en Rust avec Actix-web et Diesel ORM assure des performances exceptionnelles et une sécurité renforcée. La base de données PostgreSQL gère efficacement les données utilisateurs et de jeu, tandis que Docker et Docker Compose simplifient la conteneurisation et le déploiement. L'ensemble est hébergé sur DigitalOcean pour une infrastructure cloud robuste et scalable.

### Idéation

Le processus d'idéation a été particulièrement riche et a nécessité une analyse approfondie de plusieurs concepts innovants avant de retenir Randomi GO. L'équipe a exploré un système d'AdBlock pour stations de radio qui aurait permis un filtrage audio intelligent, ainsi qu'un GPS avec Réalité Augmentée offrant une navigation visuelle avec informations contextuelles. Un scanner d'images pour droits d'auteur avec détection automatique de propriété intellectuelle a également été envisagé, de même qu'une plateforme de création de mèmes avec des jeux collaboratifs et un système de vote. Finalement, Randomi GO a été choisi pour sa combinaison unique de challenge technique et d'aspect ludique.

Les critères de sélection ont été rigoureusement appliqués pour s'assurer de la viabilité et de l'intérêt du projet. Le challenge technique significatif représenté par l'apprentissage de Rust et le développement d'une interface 3D constituait un facteur déterminant. L'opportunité d'apprentissage de Rust, langage moderne et performant, était particulièrement attractive pour l'équipe. Le projet devait être complet avec frontend et backend pour offrir une expérience d'apprentissage complète. Le potentiel de déploiement en production était essentiel pour valider les compétences acquises. Enfin, l'aspect ludique et engageant du projet garantissait une motivation constante tout au long du développement.

### Cahier des charges

Le cahier des charges de Randomi GO définit un ensemble de fonctionnalités principales qui constituent le cœur de l'expérience utilisateur. Le système d'authentification et de gestion de comptes permet aux joueurs de créer leur profil et de personnaliser leur expérience de jeu. La création et gestion de lobbies de jeu offre une flexibilité totale pour organiser des parties privées ou rejoindre des parties publiques. Le système de matchmaking automatique optimise l'expérience en mettant en relation des joueurs de niveau similaire. L'interface de jeu en 3D avec Three.js plonge les joueurs dans une expérience immersive unique, tandis que le système de cartes et règles du jeu respecte fidèlement la mécanique du jeu original. Le chat en temps réel favorise les interactions sociales entre joueurs, et le système de statistiques de jeu permet un suivi détaillé des performances.

Les contraintes techniques définies dans le cahier des charges garantissent la qualité et la robustesse de l'application. La compatibilité multi-navigateurs assure une accessibilité maximale pour tous les utilisateurs, quel que soit leur navigateur préféré. Le responsive design adapte automatiquement l'interface à tous les types d'écrans, des smartphones aux écrans de bureau. Les performances optimisées garantissent une expérience de jeu fluide même sur des connexions limitées. La sécurité renforcée protège les données utilisateurs et prévient les tentatives de triche. Enfin, la scalabilité de l'architecture permet d'accueillir un nombre croissant de joueurs simultanés.

Les livrables définis dans le cahier des charges comprennent une application web entièrement fonctionnelle avec toutes les fonctionnalités spécifiées. La documentation technique complète facilite la maintenance et l'évolution future du projet. Les tests automatisés assurent la qualité et la fiabilité du code. Le déploiement en production valide l'ensemble des compétences techniques acquises et démontre la capacité à livrer un projet complet et fonctionnel.

---

## II. Conception

### Organisation & Méthodologie

La méthodologie Agile a été adoptée dès le début du projet pour assurer une gestion efficace et adaptative du développement. L'utilisation de Kanban pour la gestion des tâches a permis une visualisation claire de l'avancement du projet et une identification rapide des goulots d'étranglement. Les sprints de développement de deux semaines ont offert un rythme optimal pour la planification et la livraison de fonctionnalités. Les réunions de suivi quotidiennes ont maintenu la cohésion d'équipe et permis d'anticiper les difficultés. Les rétrospectives en fin de sprint ont favorisé l'amélioration continue des processus et des pratiques de développement.

L'écosystème d'outils choisi pour la gestion du projet a été soigneusement sélectionné pour optimiser la collaboration et la productivité. Trello a servi de tableau Kanban principal pour le suivi des tâches avec des colonnes dédiées aux différentes étapes du workflow. Git avec GitFlow a structuré le versioning du code avec des branches spécialisées pour les fonctionnalités, les corrections et les releases. GitHub a centralisé l'hébergement du code source et facilité la collaboration entre développeurs. Discord a été utilisé comme plateforme de communication d'équipe pour les échanges quotidiens et le partage d'informations techniques.

Le workflow GitFlow adopté a structuré efficacement le développement avec des branches spécialisées pour chaque type de modification. La branche main contient le code en production et reste stable en permanence. La branche develop centralise le code en cours de développement et sert de base pour l'intégration des nouvelles fonctionnalités. Les branches feature permettent le développement isolé de nouvelles fonctionnalités sans impacter le code principal. Les branches hotfix permettent de corriger rapidement les problèmes critiques en production. Les branches release facilitent la préparation des nouvelles versions avant leur déploiement.

### Maquettage

Le processus de maquettage a été essentiel pour définir l'expérience utilisateur et valider les choix d'interface avant le développement. Les maquettes réalisées couvrent l'ensemble des écrans principaux de l'application, de la page d'accueil avec menu principal à l'interface de jeu avec cartes 3D. L'interface d'authentification a été particulièrement soignée pour offrir une expérience de connexion fluide et sécurisée. Le lobby de jeu avec liste des parties permet aux joueurs de naviguer facilement entre les différentes options de jeu. L'interface de jeu avec cartes 3D constitue le cœur de l'expérience utilisateur et a nécessité plusieurs itérations pour optimiser l'ergonomie. Le menu des paramètres offre une personnalisation complète de l'expérience de jeu.

Les outils de maquettage choisis ont été sélectionnés pour leur complémentarité et leur efficacité. Figma a été utilisé pour créer les maquettes UI/UX détaillées avec une attention particulière portée aux interactions et aux micro-animations. Adobe XD a permis de développer des prototypes interactifs pour valider les parcours utilisateur avant le développement. Balsamiq a facilité la création rapide de wireframes pour les premières explorations d'idées et la validation des concepts avec l'équipe.

### Design et charte graphique

La charte graphique de Randomi GO a été conçue pour créer une identité visuelle forte et cohérente qui reflète l'esprit moderne et ludique du jeu. Les couleurs principales choisies - bleu (#1E3A8A), orange (#F97316) et blanc (#FFFFFF) - créent un contraste dynamique et professionnel. Le bleu apporte une dimension technologique et fiable, l'orange injecte de l'énergie et de la créativité, tandis que le blanc assure la lisibilité et la clarté de l'interface. La typographie Inter a été sélectionnée pour sa modernité et sa lisibilité optimale sur tous les supports.

Le style visuel adopté privilégie un design moderne et épuré qui met l'accent sur l'expérience utilisateur. Les cartes 3D avec effets de profondeur créent une sensation d'immersion et de tangibilité qui enrichit l'expérience de jeu. Les animations fluides et réactives répondent aux actions utilisateur de manière naturelle et engageante. Les particules et effets visuels ajoutent une dimension dynamique sans compromettre les performances. L'interface intuitive et immersive guide naturellement l'utilisateur à travers les différentes fonctionnalités du jeu.

### Accessibilité / Responsivité

L'accessibilité a été une préoccupation majeure tout au long du développement de Randomi GO pour garantir une expérience inclusive pour tous les utilisateurs. Le contraste suffisant entre les éléments d'interface assure une lisibilité optimale même pour les utilisateurs présentant des difficultés visuelles. La navigation au clavier permet aux utilisateurs de parcourir l'ensemble de l'application sans utiliser la souris, facilitant ainsi l'accès pour les personnes en situation de handicap moteur. Les textes alternatifs pour les images et éléments graphiques permettent aux lecteurs d'écran de restituer fidèlement le contenu aux utilisateurs malvoyants. La structure sémantique HTML5 respecte les standards d'accessibilité web et améliore la navigation pour tous les utilisateurs.

La responsivité de l'interface a été conçue selon une approche mobile-first pour garantir une expérience optimale sur tous les appareils. Le design s'adapte automatiquement aux différentes tailles d'écran avec des breakpoints définis à 320px, 768px, 1024px et 1440px. Les éléments 3D se redimensionnent intelligemment pour maintenir la lisibilité et l'ergonomie sur les petits écrans. L'interface tactile a été optimisée pour offrir une expérience fluide sur les tablettes et smartphones, avec des zones de toucher suffisamment grandes et des gestes intuitifs. Cette approche garantit que Randomi GO reste accessible et agréable à utiliser quel que soit l'appareil choisi par l'utilisateur.

### Architecture logicielle

L'architecture logicielle de Randomi GO a été conçue pour assurer la scalabilité, la maintenabilité et les performances optimales. L'architecture globale repose sur une séparation claire entre le frontend développé avec Three.js, le backend réalisé en Rust avec Actix-web, et la base de données PostgreSQL. Cette séparation des responsabilités permet une évolution indépendante de chaque composant et facilite la maintenance du code. Le frontend communique avec le backend via des appels API REST, tandis que le backend interagit avec la base de données pour la persistance des données.

Les patterns architecturaux choisis ont été sélectionnés pour leur robustesse et leur adaptabilité aux besoins du projet. Le pattern MVC (Modèle-Vue-Contrôleur) structure l'organisation du code en séparant clairement la logique métier, la présentation et le contrôle des données. Le Repository Pattern centralise l'accès aux données et simplifie les tests en permettant la substitution facile des sources de données. L'injection de dépendances favorise la modularité du code et facilite les tests unitaires. Le pattern Observer gère efficacement les événements temps réel nécessaires au fonctionnement du jeu multijoueur.

### Conception de la base de données

La conception de la base de données PostgreSQL pour Randomi GO a été réalisée avec une attention particulière portée à la normalisation, aux performances et à la scalabilité. La structure de la base de données comprend plusieurs tables principales qui organisent efficacement les données utilisateurs et de jeu. La table accounts centralise les informations d'authentification et de profil des utilisateurs avec des champs pour l'identifiant, le nom d'utilisateur, l'email, le hash du mot de passe et la date de création. La table friends gère les relations d'amitié entre utilisateurs avec un système de statut pour distinguer les demandes en attente des amitiés confirmées. La table lobbies organise les parties de jeu avec des informations sur l'hôte, le nombre maximum de joueurs et le statut de la partie. La table players associe les utilisateurs aux lobbies avec leur position dans le jeu. La table cards stocke les informations des cartes du jeu, tandis que user_cards gère l'inventaire personnel de chaque joueur.

Les relations entre les tables ont été conçues pour optimiser les performances et garantir l'intégrité des données. La relation many-to-many entre utilisateurs et amis est gérée par la table friends avec des contraintes d'intégrité référentielle. La relation one-to-many entre lobbies et joueurs permet une gestion efficace des parties multijoueurs. La relation many-to-many entre utilisateurs et cartes via la table user_cards offre une flexibilité maximale pour l'inventaire personnel. Les index ont été stratégiquement placés sur les clés étrangères et les champs de recherche fréquents pour optimiser les temps de réponse. Le partitioning des tables de logs permet une gestion efficace des données historiques sans impacter les performances des requêtes principales.

### Routes et endpoints

L'API REST de Randomi GO expose un ensemble complet d'endpoints organisés par fonctionnalité pour faciliter l'intégration et la maintenance. Les routes d'authentification incluent POST /register pour l'inscription de nouveaux utilisateurs, POST /login pour la connexion et POST /logout pour la déconnexion sécurisée. La gestion des comptes est assurée par GET /account/profile pour récupérer les informations utilisateur, PUT /account/profile pour la mise à jour du profil et GET /account/requests pour consulter les demandes d'amitié. Les routes de gestion des amis comprennent POST /account/friends/add pour envoyer des demandes d'amitié et DELETE /account/friends/remove pour supprimer des relations.

Les routes de lobby et de jeu constituent le cœur fonctionnel de l'application avec GET /lobby/list/{page} pour lister les parties disponibles, POST /lobby/create pour créer une nouvelle partie, GET /lobby/{id} pour récupérer les détails d'une partie, POST /lobby/{id}/join pour rejoindre une partie et DELETE /lobby/{id}/leave pour quitter une partie. Les événements temps réel sont gérés par GET /events qui utilise Server-Sent Events pour maintenir une connexion persistante avec le client.

Les codes de statut HTTP ont été choisis pour fournir des informations claires et précises sur le résultat des requêtes. Le code 200 indique un succès, 201 confirme la création d'une ressource, 400 signale une requête invalide, 401 indique un problème d'authentification, 403 révèle un manque d'autorisation, 404 signifie que la ressource n'existe pas, et 500 signale une erreur interne du serveur. Cette standardisation facilite le débogage et l'intégration côté client.

### Contrôleurs / logique métier

La structure des contrôleurs de Randomi GO a été conçue pour assurer une séparation claire des responsabilités et faciliter la maintenance du code. Le AuthController gère l'ensemble des opérations d'authentification avec une connexion à la base de données et une clé secrète JWT pour la génération et validation des tokens. Le AccountController centralise la gestion des comptes utilisateurs avec des opérations de lecture et modification des profils. Le LobbyController orchestre la création et gestion des parties de jeu avec une connexion au serveur de jeu pour la synchronisation temps réel.

La logique métier implémentée dans chaque contrôleur respecte les principes de validation des données d'entrée pour garantir l'intégrité des informations traitées. Le hachage sécurisé des mots de passe utilise l'algorithme bcrypt pour protéger les informations sensibles des utilisateurs. La génération et validation des JWT assurent une authentification robuste avec des tokens à durée de vie limitée. La gestion des sessions utilisateur permet de maintenir l'état de connexion de manière sécurisée. Le système de matchmaking analyse les préférences et le niveau des joueurs pour créer des parties équilibrées et engageantes.

### API et WebServices

Le format des réponses API de Randomi GO a été standardisé pour assurer une cohérence et une facilité d'intégration pour tous les clients. Chaque réponse inclut un champ success indiquant le statut de l'opération, un champ data contenant les informations de la réponse, un message descriptif pour l'utilisateur et un timestamp pour la traçabilité. Cette structure uniforme simplifie le traitement côté client et améliore l'expérience développeur.

La gestion des erreurs suit un format standardisé qui facilite le débogage et l'amélioration continue de l'application. Chaque erreur inclut un code d'erreur unique, un message explicatif et des détails techniques lorsque nécessaire. Les codes d'erreur sont organisés par catégorie pour faciliter l'identification et la résolution des problèmes. Cette approche améliore significativement la qualité du support utilisateur et la maintenance de l'application.

La documentation API utilise Swagger/OpenAPI pour fournir une documentation interactive et complète de tous les endpoints. Cette documentation inclut des exemples de requêtes et réponses pour chaque endpoint, des codes d'erreur détaillés avec leurs causes possibles, et un guide d'intégration pour les développeurs tiers. Cette approche favorise l'adoption de l'API et facilite l'intégration avec d'autres systèmes.

### Backend (fonctionnement, choix, logique)

Le choix de Rust pour le développement backend de Randomi GO a été motivé par plusieurs facteurs techniques décisifs. La performance exceptionnelle de Rust garantit des temps de réponse optimaux même sous charge importante, essentiel pour un jeu multijoueur en temps réel. La sécurité mémoire garantie par le système de propriété de Rust élimine les vulnérabilités courantes comme les buffer overflows et les use-after-free, crucial pour la sécurité des données utilisateurs. L'écosystème mature de Rust offre une large gamme de bibliothèques stables et bien documentées, facilitant le développement rapide et fiable. L'excellente documentation de Rust et sa communauté active ont été des atouts majeurs pour l'apprentissage et la résolution de problèmes.

L'architecture backend de Randomi GO suit une structure modulaire organisée autour de responsabilités claires. Le point d'entrée main.rs initialise l'application et configure les services essentiels. Le fichier lib.rs centralise la configuration globale et les paramètres de l'application. Le module auth.rs gère l'ensemble des opérations d'authentification avec validation des tokens JWT. La couche database organise l'accès aux données avec models.rs pour les structures de données, schema.rs pour la définition du schéma et actions.rs pour les opérations de base de données. Les contrôleurs dans le dossier routes gèrent les requêtes HTTP avec auth.rs, account.rs et game.rs. Le serveur de jeu dans le dossier server contient la logique métier avec game.rs pour les règles de jeu et events.rs pour la gestion des événements temps réel.

Les fonctionnalités backend implémentées couvrent l'ensemble des besoins de l'application. L'authentification JWT assure une sécurité robuste avec génération et validation de tokens sécurisés. La gestion des sessions permet de maintenir l'état utilisateur de manière efficace et sécurisée. La validation des données d'entrée garantit l'intégrité des informations traitées par l'application. Le logging structuré facilite le monitoring et le débogage en production. La gestion d'erreurs centralisée assure une expérience utilisateur cohérente même en cas de problème technique.

### Frontend (frameworks, composants...)

Le choix de Three.js pour le développement frontend de Randomi GO a été guidé par la nécessité de créer une expérience utilisateur immersive et moderne. Three.js offre un rendu 3D performant qui permet d'afficher les cartes de jeu avec une profondeur et un réalisme exceptionnels. Les animations fluides créent une sensation de fluidité et de réactivité qui enrichit significativement l'expérience de jeu. Les effets visuels avancés comme les particules et les shaders ajoutent une dimension esthétique sans compromettre les performances. La compatibilité navigateur de Three.js garantit une expérience cohérente sur tous les appareils et navigateurs modernes.

L'architecture frontend de Randomi GO suit une organisation modulaire qui facilite la maintenance et l'évolution du code. Le point d'entrée main.js initialise l'application et configure les services essentiels. Le dossier api organise la communication avec le backend avec auth.js pour l'authentification et game.js pour les opérations de jeu. Le dossier game contient la logique métier avec cards.js pour la gestion des cartes, player.js pour les informations joueur et events.js pour la gestion des événements. Le dossier ui centralise les composants d'interface avec viewmgr.js pour la gestion des vues, lobby.js pour l'interface de lobby et popup.js pour les dialogues. Le fichier utils.js fournit des fonctions utilitaires partagées par l'ensemble de l'application.

Les composants principaux de l'interface utilisateur ont été conçus pour offrir une expérience utilisateur optimale. Le ViewManager gère les transitions entre les différentes vues de l'application avec une logique de changement de vue fluide et intuitive. Le CardManager crée et anime les cartes 3D avec des effets de profondeur et des interactions tactiles naturelles. Ces composants travaillent en harmonie pour créer une interface cohérente et engageante qui guide naturellement l'utilisateur à travers les différentes fonctionnalités du jeu.

Les fonctionnalités frontend implémentées assurent une expérience utilisateur complète et satisfaisante. L'interface responsive s'adapte automatiquement à tous les types d'écrans, des smartphones aux écrans de bureau. Les animations fluides répondent aux actions utilisateur de manière naturelle et engageante. La gestion des événements assure une réactivité optimale aux interactions utilisateur. La communication WebSocket maintient une synchronisation temps réel avec le serveur de jeu. Le cache local optimise les performances en réduisant les requêtes réseau répétées.

---

## III. Production

### Authentification / gestion des utilisateurs

Le système d'authentification de Randomi GO a été conçu pour offrir une sécurité robuste tout en garantissant une expérience utilisateur fluide. La structure utilisateur centralise les informations essentielles avec un identifiant unique, un nom d'utilisateur personnalisable, une adresse email validée, un hash sécurisé du mot de passe et un timestamp de création pour la traçabilité. Le middleware d'authentification intercepte chaque requête pour valider les tokens JWT et extraire les informations utilisateur nécessaires au traitement de la requête. Cette approche assure une séparation claire entre l'authentification et la logique métier, facilitant la maintenance et les tests.

Les fonctionnalités d'authentification implémentées couvrent l'ensemble du cycle de vie utilisateur. L'inscription avec validation email garantit l'unicité des comptes et la qualité des données utilisateur. La connexion sécurisée utilise des tokens JWT avec expiration automatique pour limiter les risques de session compromise. La récupération de mot de passe permet aux utilisateurs de retrouver l'accès à leur compte de manière sécurisée. Les sessions persistantes maintiennent l'état de connexion entre les sessions de navigation. La déconnexion automatique protège les utilisateurs en cas d'inactivité prolongée.

La sécurité du système d'authentification repose sur plusieurs couches de protection. Le hachage bcrypt des mots de passe utilise un facteur de coût adaptatif qui s'ajuste automatiquement aux capacités de calcul disponibles. Les tokens JWT incluent une expiration configurable et sont signés avec une clé secrète robuste. La protection CSRF empêche les attaques de type cross-site request forgery. Le rate limiting par adresse IP limite les tentatives de connexion et prévient les attaques par force brute.

### Sécurité (validation, droits, protection)

La validation des données dans Randomi GO suit une approche systématique qui garantit l'intégrité et la sécurité des informations traitées. Chaque structure de données d'entrée inclut des validations spécifiques avec des contraintes de longueur, de format et de contenu. Les noms d'utilisateur sont limités entre 3 et 20 caractères pour éviter les abus tout en permettant la personnalisation. Les adresses email sont validées selon les standards RFC pour assurer la conformité et la fonctionnalité. Les mots de passe doivent respecter une longueur minimale de 8 caractères pour garantir une sécurité suffisante.

La protection contre les attaques courantes a été implémentée à plusieurs niveaux de l'application. Les requêtes préparées avec Diesel ORM éliminent complètement les risques d'injection SQL en séparant les données des instructions. L'échappement automatique des données utilisateur prévient les attaques XSS en neutralisant les scripts malveillants. Les tokens CSRF protègent contre les attaques de type cross-site request forgery en validant l'origine des requêtes. Le rate limiting par adresse IP limite les tentatives de connexion et prévient les attaques par force brute. Les tokens JWT sécurisés avec signature cryptographique empêchent la falsification et le vol de session.

La gestion des droits utilisateur repose sur un système de rôles et permissions granulaire. Les rôles utilisateur incluent administrateur, modérateur et joueur standard avec des privilèges adaptés à chaque niveau. Les permissions granulaires permettent un contrôle précis des actions autorisées pour chaque utilisateur. La vérification des droits sur chaque action assure que seuls les utilisateurs autorisés peuvent effectuer des opérations sensibles. Cette approche garantit la sécurité tout en maintenant la flexibilité nécessaire au bon fonctionnement de l'application.

### Gestion des erreurs / logs

Le système de logging de Randomi GO utilise la bibliothèque tracing pour fournir une visibilité complète sur le fonctionnement de l'application. La configuration des logs inclut des métadonnées enrichies comme les identifiants de thread, les noms de threads, les fichiers sources et les numéros de ligne pour faciliter le débogage. Les différents niveaux de log permettent de filtrer les informations selon les besoins de monitoring et de diagnostic. Cette approche structurée facilite l'analyse des logs et l'identification rapide des problèmes.

L'utilisation des logs dans le code suit des conventions strictes pour assurer la cohérence et l'efficacité. Les informations de niveau info documentent les actions utilisateur normales comme les tentatives d'inscription et les connexions réussies. Les avertissements de niveau warn signalent les situations anormales qui ne constituent pas des erreurs critiques. Les erreurs de niveau error capturent les problèmes techniques qui nécessitent une intervention ou une investigation. Cette hiérarchie permet un monitoring efficace et une réaction appropriée aux différents types d'événements.

Les types de logs spécialisés couvrent les différents aspects de l'application. Les logs d'application documentent les actions utilisateur et les erreurs métier pour comprendre le comportement des utilisateurs. Les logs de sécurité capturent les tentatives de connexion, les violations d'accès et les activités suspectes pour la protection du système. Les logs de performance mesurent les temps de réponse et l'utilisation des ressources pour optimiser l'application. Les logs système documentent le démarrage, l'arrêt et la configuration de l'application pour la maintenance.

La gestion des erreurs centralisée assure une expérience utilisateur cohérente même en cas de problème technique. Chaque type d'erreur est associé à un code unique et un message explicatif pour faciliter le diagnostic. Les erreurs de validation incluent des détails spécifiques sur les champs problématiques et les raisons de l'échec. Les erreurs d'authentification fournissent des informations suffisantes pour guider l'utilisateur sans compromettre la sécurité. Les erreurs de base de données sont traitées de manière appropriée avec des messages utilisateur compréhensibles. Cette approche améliore significativement la qualité du support utilisateur et la maintenance de l'application.

### Tests (unitaires, fonctionnels, manuels...)

Les tests unitaires de Randomi GO couvrent les fonctions critiques avec une attention particulière portée à la sécurité et aux performances. Les tests de hachage de mot de passe vérifient que l'algorithme bcrypt fonctionne correctement et que la vérification des mots de passe est fiable. Les tests de création et validation de JWT assurent que le système d'authentification génère des tokens valides et les valide correctement. Les tests de validation des données d'entrée garantissent que les contraintes de format et de contenu sont respectées. Les tests de logique métier vérifient que les règles de jeu et les calculs sont corrects.

Les tests d'intégration valident le comportement de l'application dans son ensemble en simulant des scénarios réels d'utilisation. Les tests des endpoints API vérifient que les routes répondent correctement aux requêtes HTTP avec les codes de statut appropriés. Les tests de base de données assurent que les opérations CRUD fonctionnent correctement avec les contraintes d'intégrité. Les tests d'authentification valident le flux complet d'inscription, connexion et déconnexion. Les tests de performance mesurent les temps de réponse et la capacité de charge de l'application.

Les tests fonctionnels couvrent les scénarios utilisateur complets pour garantir une expérience de jeu satisfaisante. Les tests de bout en bout avec Selenium simulent les interactions utilisateur réelles sur l'interface web. Les tests de performance avec Artillery mesurent la capacité de charge et les temps de réponse sous stress. Les tests de sécurité avec OWASP ZAP identifient les vulnérabilités potentielles dans l'application. Les tests de compatibilité navigateur assurent que l'application fonctionne correctement sur tous les navigateurs ciblés.

La couverture de tests atteint des niveaux satisfaisants avec 85% de couverture pour les tests unitaires, une couverture complète des routes API pour les tests d'intégration, et des scénarios utilisateur exhaustifs pour les tests fonctionnels. Les tests de performance valident la capacité de l'application à supporter la charge attendue. Cette approche assure la qualité et la fiabilité du code tout en facilitant la maintenance et l'évolution future du projet.

### Conteneurisation (Docker, images)

L'architecture Docker de Randomi GO utilise Docker Compose pour orchestrer l'ensemble des services nécessaires au fonctionnement de l'application. Le service backend expose le port 8080 et se connecte à la base de données via les variables d'environnement configurées. Le service de base de données utilise PostgreSQL 15 avec des volumes persistants pour garantir la durabilité des données. Le service frontend sert l'interface utilisateur sur le port 80 et dépend du backend pour les fonctionnalités de jeu. L'ensemble des services communique via un réseau Docker dédié pour isoler le trafic et améliorer la sécurité.

Les Dockerfiles optimisés utilisent des images multi-stage pour réduire la taille finale des conteneurs. Le Dockerfile du backend utilise Rust 1.70 pour la compilation et Debian Bullseye slim pour l'exécution, avec installation des dépendances système nécessaires. Le Dockerfile du frontend utilise Node.js 18 Alpine pour la construction et Nginx Alpine pour le service, avec copie des fichiers statiques et configuration du serveur web. Cette approche minimise l'empreinte mémoire et améliore les performances de démarrage.

Les optimisations Docker incluent plusieurs techniques pour améliorer l'efficacité et la sécurité. Les images multi-stage séparent la compilation de l'exécution pour réduire la taille des images finales. Le cache des dépendances accélère les reconstructions en évitant de retélécharger les packages déjà installés. Les variables d'environnement sécurisées permettent la configuration dynamique sans exposer de secrets dans les images. Les health checks assurent que les services sont prêts à recevoir du trafic avant d'être considérés comme opérationnels.

### Déploiement (CI/CD, hébergement)

Le pipeline CI/CD de Randomi GO utilise GitHub Actions pour automatiser l'ensemble du processus de déploiement. Le job de test s'exécute sur Ubuntu Latest et lance les tests backend et frontend pour valider la qualité du code avant le déploiement. Le job de build construit les images Docker pour le backend et le frontend après validation des tests. Le job de déploiement utilise l'action DigitalOcean pour déployer automatiquement les nouvelles versions sur le serveur de production. Cette approche assure une livraison continue et fiable avec validation automatique de la qualité.

L'hébergement sur DigitalOcean utilise un droplet Ubuntu 22.04 LTS configuré pour les performances et la sécurité. La configuration du serveur inclut 2 vCPUs, 4GB de RAM et 80GB de stockage SSD pour garantir des performances optimales. Le firewall est configuré pour limiter l'accès aux ports nécessaires et protéger contre les attaques réseau. Le certificat SSL/TLS avec Let's Encrypt assure une connexion sécurisée pour tous les utilisateurs. Cette infrastructure cloud robuste et scalable permet d'accueillir un nombre croissant d'utilisateurs.

Les services déployés sur le serveur de production couvrent l'ensemble des besoins de l'application. Nginx sert de reverse proxy et de serveur web pour servir les fichiers statiques et router les requêtes API. PostgreSQL gère la base de données avec des sauvegardes automatiques pour protéger les données utilisateurs. Redis fournit un cache pour améliorer les performances et gérer les sessions utilisateur. Docker et Docker Compose facilitent la gestion des conteneurs et les mises à jour de l'application.

Le monitoring de l'application utilise Prometheus pour collecter les métriques système et applicatives. Grafana fournit des tableaux de bord personnalisés pour visualiser les performances et identifier les goulots d'étranglement. AlertManager envoie des alertes automatiques en cas de problème détecté. Cette infrastructure de monitoring assure la disponibilité et les performances de l'application en production.

---

## IV. Analyse et synthèse

### Versionning

La stratégie de versionning GitFlow adoptée pour Randomi GO a structuré efficacement le développement et facilité la collaboration entre les membres de l'équipe. La branche main contient exclusivement le code en production et reste stable en permanence, garantissant la fiabilité des déploiements. La branche develop centralise le code en cours de développement et sert de base pour l'intégration des nouvelles fonctionnalités avant leur validation. Les branches feature permettent le développement isolé de nouvelles fonctionnalités sans impacter le code principal, facilitant la revue de code et la résolution des conflits. Les branches hotfix permettent de corriger rapidement les problèmes critiques en production sans attendre le cycle de développement normal. Les branches release facilitent la préparation des nouvelles versions avec les tests finaux et la documentation avant le déploiement.

Les conventions de commit ont été établies pour maintenir une traçabilité claire et faciliter la compréhension de l'historique du projet. Les commits de type feat documentent l'ajout de nouvelles fonctionnalités comme le système d'authentification ou l'interface de jeu. Les commits de type fix signalent les corrections de bugs comme les problèmes de connexion ou les erreurs d'affichage. Les commits de type docs concernent la mise à jour de la documentation technique ou utilisateur. Les commits de type test ajoutent ou améliorent les tests automatisés. Les commits de type refactor optimisent le code existant sans changer les fonctionnalités. Cette standardisation améliore la lisibilité de l'historique et facilite la maintenance du projet.

Les tags de version marquent les étapes importantes du développement et facilitent le suivi des releases. Le tag v1.0.0 marque la version initiale avec toutes les fonctionnalités de base implémentées. Le tag v1.1.0 introduit de nouvelles fonctionnalités comme le système d'amis ou les statistiques de jeu. Le tag v1.1.1 corrige des bugs critiques découverts après le déploiement. Cette approche permet un suivi précis de l'évolution du projet et facilite la gestion des versions en production.

### Documentation (technique & utilisateur)

La documentation technique de Randomi GO couvre l'ensemble des aspects du projet pour faciliter la maintenance et l'évolution future. La documentation API utilise Swagger/OpenAPI pour fournir une interface interactive permettant aux développeurs de tester les endpoints directement depuis le navigateur. Cette documentation inclut des exemples de requêtes et réponses pour chaque endpoint, des codes d'erreur détaillés avec leurs causes possibles, et un guide d'intégration pour les développeurs tiers. La documentation développeur comprend l'architecture système détaillée, un guide d'installation complet avec les prérequis et les étapes de configuration, un guide de contribution avec les standards de code et les processus de revue, et une documentation des choix technologiques avec leurs justifications.

La documentation utilisateur a été conçue pour accompagner les utilisateurs à chaque étape de leur expérience avec Randomi GO. Le guide utilisateur inclut un tutoriel de prise en main qui guide les nouveaux utilisateurs à travers les fonctionnalités principales de l'application. Les règles du jeu détaillées expliquent les mécaniques de Randomi et les stratégies gagnantes. La FAQ répond aux questions fréquentes et aide les utilisateurs à résoudre les problèmes courants. Le support technique fournit des informations de contact et des ressources supplémentaires pour les utilisateurs ayant besoin d'aide.

L'interface utilisateur intègre plusieurs éléments de documentation contextuelle pour améliorer l'expérience utilisateur. Les tooltips contextuels fournissent des explications rapides sur les fonctionnalités sans interrompre le flux de jeu. Les messages d'aide apparaissent automatiquement lors de la première utilisation de nouvelles fonctionnalités. Le tutoriel interactif guide les utilisateurs à travers les étapes essentielles de l'application. Le système de feedback utilisateur permet de recueillir les suggestions et signaler les problèmes directement depuis l'interface.

### Valeur ajoutée du projet

L'innovation technique de Randomi GO réside dans sa position de première adaptation numérique du jeu de cartes Randomi, créant ainsi une expérience de jeu inédite et accessible. L'utilisation de Rust pour le backend représente un choix technologique audacieux qui démontre la capacité à maîtriser des technologies modernes et performantes. L'interface 3D immersive développée avec Three.js révolutionne l'expérience de jeu traditionnelle en ajoutant une dimension visuelle exceptionnelle. L'architecture microservices conteneurisée avec Docker assure la scalabilité et la maintenabilité du projet, facilitant les déploiements et les mises à jour.

La valeur pédagogique du projet est particulièrement significative car il a permis d'acquérir et de maîtriser un ensemble complet de technologies modernes. L'apprentissage de Rust a ouvert de nouvelles perspectives sur la programmation système et la gestion de la mémoire. La découverte de Three.js a enrichi les compétences en développement frontend avec des technologies 3D avancées. L'utilisation de Docker et des pratiques DevOps a familiarisé l'équipe avec les méthodologies de déploiement moderne. L'implémentation de tests automatisés complets a renforcé les bonnes pratiques de développement.

La valeur métier de Randomi GO s'exprime à travers plusieurs dimensions stratégiques. L'accessibilité sur tous les appareils disposant d'un navigateur web maximise l'audience potentielle et facilite l'adoption par les utilisateurs. La création d'une communauté de joueurs en ligne ouvre des perspectives de monétisation à travers des fonctionnalités premium ou des cosmétiques. Le potentiel de scalabilité internationale permet d'envisager une expansion vers de nouveaux marchés. L'aspect ludique et engageant du projet garantit une rétention utilisateur élevée et une viralité naturelle.

### Obstacles rencontrés

Les obstacles techniques rencontrés lors du développement de Randomi GO ont nécessité des solutions créatives et une persévérance importante. L'apprentissage de Rust a représenté une courbe d'apprentissage significative avec ses concepts uniques comme le système de propriété et la gestion de la mémoire sans garbage collector. Cette difficulté a été surmontée par une formation intensive et l'utilisation de ressources pédagogiques de qualité. L'intégration de Three.js pour les animations 3D a posé des défis de performance et de compatibilité navigateur qui ont été résolus par l'optimisation des shaders et l'adaptation des effets visuels. Les WebSocket temps réel ont nécessité une gestion complexe de la synchronisation entre les joueurs, résolue par l'implémentation d'un système d'événements robuste. L'optimisation des requêtes de base de données a été cruciale pour maintenir les performances sous charge, atteinte par l'indexation stratégique et la normalisation des données.

Les obstacles organisationnels ont également impacté le développement et nécessité des ajustements méthodologiques. La gestion du temps entre le développement de nouvelles fonctionnalités et l'assurance de la qualité du code existant a été un défi constant résolu par l'établissement de priorités claires et l'utilisation de sprints courts. Le maintien de la documentation à jour avec l'évolution rapide du code a été facilité par l'intégration de la documentation dans le workflow de développement. L'implémentation d'une couverture de tests complète a été rendue possible par l'adoption de tests-driven development et l'automatisation des tests. La configuration de l'environnement de production a été simplifiée par l'utilisation de Docker et l'automatisation du déploiement.

Les solutions apportées à ces obstacles ont considérablement enrichi les compétences techniques et méthodologiques de l'équipe. La formation intensive sur Rust a permis de maîtriser un langage moderne et performant. L'utilisation de bibliothèques Three.js éprouvées a facilité l'implémentation d'effets visuels complexes. L'architecture modulaire adoptée a simplifié la maintenance et l'évolution du code. L'automatisation des tests a garanti la qualité et la fiabilité du projet tout au long du développement.

### Bénéfices personnels et techniques

Les compétences techniques acquises lors du développement de Randomi GO représentent un enrichissement significatif du profil professionnel. La maîtrise de Rust, langage système moderne et sécurisé, ouvre des perspectives dans le développement de systèmes critiques et performants. L'expérience avec Three.js enrichit les compétences en développement frontend avec des technologies 3D avancées. L'utilisation de Docker et des pratiques de conteneurisation familiarise avec les méthodologies DevOps modernes. La gestion de PostgreSQL avec des requêtes complexes améliore les compétences en base de données relationnelle. L'implémentation de CI/CD avec GitHub Actions démontre la capacité à automatiser les processus de développement et de déploiement.

Les compétences méthodologiques développées sont tout aussi précieuses pour la carrière professionnelle. La gestion de projet avec méthodologie Agile a renforcé les capacités d'organisation et de planification. L'architecture logicielle avec l'utilisation de design patterns a amélioré la qualité et la maintenabilité du code. Les stratégies de test complètes assurent la fiabilité des applications développées. La rédaction de documentation technique claire et complète facilite la collaboration et la maintenance des projets.

Les compétences transversales acquises sont essentielles pour le métier de Concepteur Développeur d'Applications. La résolution de problèmes complexes a été renforcée par l'analyse et le débogage de situations techniques difficiles. L'apprentissage autonome a été développé par la recherche et la formation sur de nouvelles technologies. Le travail en équipe a été amélioré par la collaboration et la communication sur un projet complexe. La gestion du temps a été optimisée par la planification et la priorisation des tâches dans un projet aux multiples facettes.

### Perspectives d'amélioration

Les perspectives d'amélioration à court terme de Randomi GO se concentrent sur l'enrichissement de l'expérience utilisateur et l'optimisation des performances. L'ajout de nouvelles cartes et mécaniques de jeu permettra de renouveler l'intérêt des joueurs et d'étendre la durée de vie du jeu. L'amélioration de l'interface utilisateur avec des animations plus fluides et des effets visuels avancés enrichira l'expérience immersive. L'optimisation des performances, particulièrement sur les appareils mobiles, améliorera l'accessibilité et la satisfaction utilisateur. L'extension des tests automatisés garantira la qualité et la fiabilité du code lors des évolutions futures.

Les améliorations à moyen terme visent l'expansion de la plateforme et l'ajout de fonctionnalités avancées. Le développement d'une application mobile native permettra d'atteindre une audience plus large et d'offrir une expérience optimisée pour les appareils mobiles. L'implémentation d'un système de tournois avec classements et récompenses ajoutera une dimension compétitive et sociale au jeu. L'intégration des réseaux sociaux facilitera le partage et la découverte du jeu par de nouveaux utilisateurs. La monétisation à travers des cosmétiques et un système de battle pass créera des revenus durables pour le développement continu.

Les perspectives à long terme envisagent des innovations technologiques majeures et une expansion internationale. L'intégration d'intelligence artificielle pour les adversaires permettra d'offrir des parties équilibrées même avec peu de joueurs en ligne. Le développement d'un mode coopératif ajoutera une dimension collaborative au jeu. L'exploration de la blockchain pour les NFT de cartes créera un écosystème de collection numérique. L'expansion internationale avec la localisation dans plusieurs langues et l'adaptation aux différentes cultures élargira significativement l'audience potentielle.

### Autres projets réalisés

Les projets personnels réalisés parallèlement à Randomi GO démontrent une diversité de compétences et une curiosité technique constante. Le portfolio web développé avec React présente les compétences frontend et la capacité à créer des interfaces utilisateur modernes et responsives. Le bot Discord développé en JavaScript automatise la gestion de serveurs communautaires et démontre l'aptitude à créer des outils utilitaires. L'application mobile de gestionnaire de tâches, développée avec React Native, illustre la capacité à créer des applications mobiles fonctionnelles. L'API REST de service météo, développée en Node.js, démontre la maîtrise des architectures backend et de l'intégration d'APIs externes.

Les projets académiques réalisés dans le cadre de la formation ont enrichi les compétences techniques et méthodologiques. Le système de gestion de bibliothèque avec base de données relationnelle a renforcé les compétences en modélisation de données et en développement d'applications métier. La plateforme de e-learning a permis d'explorer les architectures web complexes et les systèmes de gestion de contenu. L'application mobile de navigation a familiarisé avec les APIs de géolocalisation et les interfaces utilisateur adaptatives. Le projet d'intelligence artificielle avec système de recommandation a ouvert des perspectives sur les algorithmes de machine learning et l'analyse de données.

Ces projets complémentaires ont contribué à former un profil de développeur polyvalent capable de s'adapter à différents contextes et technologies. La diversité des technologies utilisées (React, Node.js, React Native, Python) démontre une capacité d'apprentissage rapide et une curiosité technique. La variété des domaines d'application (gaming, utilitaires, e-learning, navigation) illustre une capacité à comprendre les besoins métier et à proposer des solutions adaptées. La qualité des livrables et la documentation associée témoignent d'un professionnalisme et d'une rigueur dans le travail.

### Conclusion + Remerciements

Le projet Randomi GO représente une étape majeure dans le parcours de formation au métier de Concepteur Développeur d'Applications, démontrant la capacité à concevoir, développer et déployer une application complète et innovante. Ce projet a permis de maîtriser un ensemble de technologies modernes et performantes, de mettre en œuvre des méthodologies de développement agiles, et de gérer un projet complexe de bout en bout. L'expérience acquise avec Rust, Three.js, Docker et les pratiques DevOps constitue un atout précieux pour la suite de la carrière professionnelle.

Les points forts du projet résident dans l'innovation technique, la qualité du code, et la complétude de la solution livrée. La maîtrise de technologies modernes comme Rust et Three.js démontre une capacité d'apprentissage et d'adaptation aux nouvelles technologies. La gestion complète du projet, de la conception à la production, illustre les compétences organisationnelles et méthodologiques nécessaires au métier. Le déploiement en production avec monitoring et tests automatisés valide la capacité à livrer des solutions robustes et maintenables. La documentation technique et utilisateur complète témoigne d'un professionnalisme et d'une rigueur dans le travail.

Les apprentissages tirés de ce projet sont nombreux et précieux pour la suite du parcours professionnel. L'importance de l'architecture logicielle dans la maintenabilité et l'évolutivité des applications a été confirmée par l'expérience pratique. La valeur des tests automatisés pour garantir la qualité et la fiabilité du code a été démontrée tout au long du développement. La nécessité d'une documentation claire et complète pour faciliter la maintenance et l'évolution des projets a été mise en évidence. L'équilibre entre le développement de nouvelles fonctionnalités et l'assurance de la qualité du code existant s'est révélé crucial pour le succès du projet.

Je tiens à exprimer ma profonde gratitude envers toutes les personnes qui ont contribué à la réussite de ce projet. Driss Khelfi, créateur du jeu Randomi original, mérite une reconnaissance particulière pour avoir inspiré ce projet et pour son soutien tout au long du développement. L'équipe pédagogique a fourni un accompagnement précieux avec des conseils techniques et méthodologiques qui ont guidé les choix et les orientations du projet. La communauté Rust, particulièrement active et bienveillante, a été une source inépuisable de ressources et de soutien pour surmonter les difficultés techniques. Ma famille a apporté un soutien constant et une compréhension précieuse pendant les périodes de travail intensif nécessaires à la réalisation de ce projet.

Ce projet constitue une validation concrète des compétences acquises et une démonstration de la capacité à exercer le métier de Concepteur Développeur d'Applications avec professionnalisme et compétence. L'expérience riche et diversifiée tirée de ce projet, combinée aux autres réalisations personnelles et académiques, forme un profil de développeur polyvalent, curieux et rigoureux, prêt à relever les défis du développement d'applications modernes.

### Annexes

**A. Diagrammes techniques**
L'annexe A comprend l'architecture système détaillée avec les interactions entre les différents composants, le schéma de la base de données avec les relations entre les tables, et le diagramme de déploiement illustrant l'infrastructure de production. Ces diagrammes facilitent la compréhension technique du projet et servent de référence pour la maintenance et l'évolution future.

**B. Captures d'écran**
L'annexe B présente des captures d'écran de l'interface utilisateur montrant les différentes vues de l'application, le dashboard d'administration avec les métriques de performance, et les tableaux de bord de monitoring avec les indicateurs de santé du système. Ces visuels illustrent concrètement les fonctionnalités développées et la qualité de l'interface utilisateur.

**C. Code source**
L'annexe C référence le repository GitHub contenant l'ensemble du code source du projet, la documentation API complète avec des exemples d'utilisation, et le guide d'installation détaillé avec les prérequis et les étapes de configuration. Ces ressources permettent la reproduction et l'étude du projet par d'autres développeurs.

**D. Tests et métriques**
L'annexe D inclut les rapports de tests avec les résultats de couverture et les métriques de performance, les analyses de sécurité avec les vulnérabilités identifiées et corrigées, et les benchmarks de performance comparant les différentes versions de l'application. Ces données valident la qualité et la robustesse du projet.

---

*Document rédigé dans le cadre de la validation du titre professionnel de Concepteur Développeur d'Applications - 2024* 