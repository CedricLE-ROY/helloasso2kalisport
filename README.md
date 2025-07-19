# HelloAsso2Kalisport

Ce projet permet de relier les inscriptions HelloAsso au logiciel Kalisport. Il se compose d'un backend Axum et d'un frontend Yew.

## Lancer le projet en mode développement

```
# Compilation du backend
cargo run -p backend

# Dans un autre terminal, compilation du frontend avec Trunk
trunk serve --open
```

Le backend fonctionne sur `http://localhost:8080`. Le frontend est servi sur `http://localhost:8081` grâce au fichier `frontend/Trunk.toml` qui configure également un proxy des requêtes `/api` vers le backend.
