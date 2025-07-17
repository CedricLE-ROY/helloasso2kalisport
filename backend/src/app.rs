use axum::{Router, http::Method};
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{Any, CorsLayer};

pub async fn run() {
    let app = Router::new().layer(CookieManagerLayer::new()).layer(
        CorsLayer::new().allow_origin(Any).allow_methods([
            Method::GET,
            Method::POST,
            Method::OPTIONS,
        ]),
    );
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Serveur backend lancé sur http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
