# Projet spécial Rust: Systèmes hautement scalables en Rust

Le projet est séparé en couches

- Util
  - Librariries communes
- Domain
  - Modèles et messages
- Logic
  - Logique d'entreprise
- Application
  - Applications serveurs et clients
- Presentation
  - Clients de présentation (GUI)
- Tests
  - Tests d'intégration

Le docker-compose permet de lancer le serveur de matchmaking, 4 clients de test et la base de données Redis.
