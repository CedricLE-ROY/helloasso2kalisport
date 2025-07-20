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

Le backend fonctionne sur `http://localhost:8080`. Le frontend est servi sur `http://localhost:8081` grâce au fichier `frontend/Trunk.toml`.

Veillez à exécuter Trunk **depuis le dossier `frontend`** (ou avec `-c frontend/Trunk.toml`) pour que la configuration suivante soit appliquée :

```toml
[serve]
port = 8081

[[proxy]]
backend = "http://localhost:8080/api"
rewrite = "/api"
```

## Configuration HelloAsso

Le backend nécessite plusieurs variables d'environnement pour interagir avec l'API HelloAsso :

```bash
HELLOASSO_CLIENT_ID="votre client id"
HELLOASSO_CLIENT_SECRET="votre client secret"
HELLOASSO_ORGANIZATION_SLUG="mon-association"
```

Ces variables peuvent être placées dans un fichier `.env` à la racine du projet.
Une route de test est exposée sur `/api/helloasso/forms` pour récupérer la liste
des formulaires de l'organisation configurée.
