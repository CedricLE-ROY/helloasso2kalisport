# HelloAsso2Kalisport

Ce projet permet de relier les inscriptions HelloAsso au logiciel Kalisport. Il se compose d'un backend Axum et d'un frontend Yew.

## Lancer le projet en mode développement

```
# Compilation du backend
cargo run -p backend

# Dans un autre terminal, compilation du frontend avec Trunk
trunk serve --open
```

La page d'accueil s'affichera alors sur `http://localhost:8080` pour le backend et `http://localhost:8080` (ou port utilisé par Trunk) pour le frontend.
