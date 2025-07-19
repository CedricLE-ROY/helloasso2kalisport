use axum::{Json, Router, extract::State, http::Method, routing::get};
use chrono::NaiveDate;
use shared::Adherent;
use std::sync::Arc;
use tokio::{net::TcpListener, sync::RwLock};
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{Any, CorsLayer};

#[derive(Default)]
pub struct AppState {
    pub adhesions: RwLock<Vec<Adherent>>,
}

async fn list_adhesions(State(state): State<Arc<AppState>>) -> Json<Vec<Adherent>> {
    let adhesions = state.adhesions.read().await.clone();
    Json(adhesions)
}

fn example_adhesions() -> Vec<Adherent> {
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
    ]
}

pub async fn run() {
    let state = Arc::new(AppState {
        adhesions: RwLock::new(example_adhesions()),
    });

    let app = Router::new()
        .route("/api/adhesions", get(list_adhesions))
        .with_state(state)
        .layer(CookieManagerLayer::new())
        .layer(CorsLayer::new().allow_origin(Any).allow_methods([
            Method::GET,
            Method::POST,
            Method::OPTIONS,
        ]));
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8081));
    println!("Serveur backend lancé sur http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
