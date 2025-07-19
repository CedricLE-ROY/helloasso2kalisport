# HelloAsso2Kalisport

Ce projet permet de relier les inscriptions HelloAsso au logiciel Kalisport. Il se compose d'un backend Axum et d'un frontend Yew.

## Lancer le projet en mode développement

Le serveur backend écoute sur le **port 8080**. Deux stratégies sont possibles pour le frontend :

1. Lancer le backend en premier puis exécuter `trunk serve --open` **depuis le dossier `frontend/`**. Trunk détectera que le port 8080 est pris et se lancera automatiquement sur `http://localhost:8081`.
2. Ou spécifier directement un port libre pour Trunk :

```bash
# Backend sur http://localhost:8080
cargo run -p backend

# Frontend sur un autre port, par exemple 8081
cd frontend
trunk serve --port 8081 --open
```

Le backend fonctionne sur `http://localhost:8080`. Le frontend est servi sur `http://localhost:8081` grâce au fichier `frontend/Trunk.toml` qui configure également un proxy des requêtes `/api` vers le backend.
