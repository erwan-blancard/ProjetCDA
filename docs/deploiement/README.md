# Documentation de Déploiement - Randomi GO

## Vue d'ensemble

Cette documentation décrit l'ensemble du processus de déploiement de l'application Randomi GO, incluant la configuration CI/CD, l'infrastructure de production et les procédures de mise en œuvre.

## Architecture de Déploiement

### Environnements
- **Développement** : Environnement local avec Docker Compose
- **Tests d'intégration (SIT)** : Environnement de validation automatisée
- **Tests d'acceptation (UAT)** : Environnement de validation fonctionnelle
- **Production** : Serveur DigitalOcean avec infrastructure complète

### Stack Technologique
- **CI/CD** : GitHub Actions
- **Conteneurisation** : Docker & Docker Compose
- **Hébergement** : DigitalOcean Droplet
- **Reverse Proxy** : Nginx
- **Base de données** : PostgreSQL
- **Cache** : Redis
- **Monitoring** : Prometheus + Grafana + AlertManager
- **SSL/TLS** : Let's Encrypt

## Structure de la Documentation

1. [Procédure de Déploiement](./procedure-deploiement.md)
2. [Scripts de Déploiement](./scripts/)
3. [Configuration CI/CD](./ci-cd/)
4. [Infrastructure de Production](./infrastructure/)
5. [Tests et Validation](./tests/)
6. [Monitoring et Maintenance](./monitoring/)

## Dépendances et Versions

### Backend (Rust)
- Rust 1.84.1
- Diesel ORM
- PostgreSQL 15+
- Redis 7+

### Frontend (JavaScript)
- Node.js 18+
- Vite 5+
- Modern JavaScript (ES2022+)

### Infrastructure
- Ubuntu 22.04 LTS
- Docker 24+
- Docker Compose 2+
- Nginx 1.24+

## Sécurité

- Firewall configuré (ports 22, 80, 443, 8080)
- Certificats SSL/TLS automatiques
- Variables d'environnement sécurisées
- Sauvegardes automatiques de la base de données
- Monitoring des tentatives d'intrusion

## Performance

- Serveur : 2 vCPUs, 4GB RAM, 80GB SSD
- Cache Redis pour les sessions et données fréquentes
- Optimisation des images Docker
- Compression gzip pour les assets statiques 