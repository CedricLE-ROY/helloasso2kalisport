use axum::{Router, routing::get, http::Method};
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{CorsLayer, Any};

pub async fn run() {
    let app = Router::new()
        // .route("/", get(root_handler)) // Ajoute tes routes ici
        .layer(CookieManagerLayer::new())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        );
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Serveur backend lancé sur http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
