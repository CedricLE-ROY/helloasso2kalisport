mod app;
mod helloasso;

use dotenvy::dotenv;

// Load environment variables from a .env file if present
fn init_env() {
    let _ = dotenv();
}

#[tokio::main]
async fn main() {
    init_env();
    app::run().await;
}
