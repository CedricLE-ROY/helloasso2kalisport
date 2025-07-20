use axum::{
    Json, Router,
    extract::{Path, State},
    http::{Method, StatusCode},
    routing::get,
};
use chrono::NaiveDate;
use serde_json::from_value;
use shared::{Adherent, HelloAssoForms, Saison};
use std::{collections::HashMap, sync::Arc};
use tokio::{net::TcpListener, sync::RwLock};
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{Any, CorsLayer};

use crate::helloasso::HelloAssoClient;

pub struct AppState {
    pub saisons: RwLock<Vec<Saison>>,
    pub adherents: RwLock<HashMap<u32, Vec<Adherent>>>,
    pub helloasso: HelloAssoClient,
    pub org_slug: String,
}

async fn list_saisons(State(state): State<Arc<AppState>>) -> Json<Vec<Saison>> {
    let saisons = state.saisons.read().await.clone();
    Json(saisons)
}

async fn list_adherents(
    Path(id): Path<u32>,
    State(state): State<Arc<AppState>>,
) -> Json<Vec<Adherent>> {
    let adherents = state
        .adherents
        .read()
        .await
        .get(&id)
        .cloned()
        .unwrap_or_default();
    Json(adherents)
}

async fn list_forms(
    State(state): State<Arc<AppState>>,
) -> Result<Json<HelloAssoForms>, StatusCode> {
    let path = format!("/v5/organizations/{}/forms", state.org_slug);
    match state.helloasso.get_json(&path).await {
        Ok(v) => from_value::<HelloAssoForms>(v)
            .map(Json)
            .map_err(|_| StatusCode::BAD_GATEWAY),
        Err(_) => Err(StatusCode::BAD_GATEWAY),
    }
}

fn example_data() -> (Vec<Saison>, HashMap<u32, Vec<Adherent>>) {
    let saisons = vec![
        Saison {
            id: 1,
            nom: "Saison 2023/2024".into(),
        },
        Saison {
            id: 2,
            nom: "Saison 2024/2025".into(),
        },
    ];

    let mut adherents = HashMap::new();
    adherents.insert(
        1,
        vec![
            Adherent {
                nom: "Doe".into(),
                prenom: "John".into(),
                date_naissance: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
                email: "john.doe@example.com".into(),
                deja_exporte: false,
            },
            Adherent {
                nom: "Dupont".into(),
                prenom: "Jeanne".into(),
                date_naissance: NaiveDate::from_ymd_opt(1985, 5, 12).unwrap(),
                email: "jeanne.dupont@example.com".into(),
                deja_exporte: false,
            },
        ],
    );
    adherents.insert(
        2,
        vec![Adherent {
            nom: "Durand".into(),
            prenom: "Luc".into(),
            date_naissance: NaiveDate::from_ymd_opt(1992, 3, 4).unwrap(),
            email: "luc.durand@example.com".into(),
            deja_exporte: false,
        }],
    );
    (saisons, adherents)
}

pub async fn run() {
    let (saisons, adherents) = example_data();
    let helloasso = HelloAssoClient::new_from_env().expect("HELLOASSO credentials");
    let org_slug = std::env::var("HELLOASSO_ORGANIZATION_SLUG").unwrap_or_default();
    let state = Arc::new(AppState {
        saisons: RwLock::new(saisons),
        adherents: RwLock::new(adherents),
        helloasso,
        org_slug,
    });

    let app = Router::new()
        .route("/api/saisons", get(list_saisons))
        .route("/api/saisons/:id/adhesions", get(list_adherents))
        .route("/api/helloasso/forms", get(list_forms))
        .with_state(state)
        .layer(CookieManagerLayer::new())
        .layer(CorsLayer::new().allow_origin(Any).allow_methods([
            Method::GET,
            Method::POST,
            Method::OPTIONS,
        ]));
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Serveur backend lancé sur http://{addr}");

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
