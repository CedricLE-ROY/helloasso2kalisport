use reqwest::Client;
use serde::Deserialize;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
struct Token {
    value: String,
    expires_at: Instant,
}

#[derive(Debug)]
pub struct HelloAssoClient {
    http: Client,
    client_id: String,
    client_secret: String,
    token: RwLock<Option<Token>>,
}

impl HelloAssoClient {
    pub fn new_from_env() -> Result<Self, std::env::VarError> {
        let client_id = std::env::var("HELLOASSO_CLIENT_ID")?;
        let client_secret = std::env::var("HELLOASSO_CLIENT_SECRET")?;
        Ok(Self {
            http: Client::new(),
            client_id,
            client_secret,
            token: RwLock::new(None),
        })
    }

    async fn ensure_token(&self) -> reqwest::Result<String> {
        if let Some(tok) = self.token.read().await.as_ref() {
            if tok.expires_at > Instant::now() {
                return Ok(tok.value.clone());
            }
        }
        #[derive(Deserialize)]
        struct TokenResp {
            access_token: String,
            expires_in: u64,
        }
        let resp: TokenResp = self
            .http
            .post("https://api.helloasso.com/oauth2/token")
            .form(&[
                ("grant_type", "client_credentials"),
                ("client_id", &self.client_id),
                ("client_secret", &self.client_secret),
            ])
            .send()
            .await?
            .json()
            .await?;
        let token = Token {
            value: resp.access_token.clone(),
            expires_at: Instant::now() + Duration::from_secs(resp.expires_in),
        };
        *self.token.write().await = Some(token);
        Ok(resp.access_token)
    }

    pub async fn get_json(&self, path: &str) -> reqwest::Result<serde_json::Value> {
        let token = self.ensure_token().await?;
        let url = format!("https://api.helloasso.com{path}");
        self.http
            .get(url)
            .bearer_auth(token)
            .send()
            .await?
            .json()
            .await
    }
}
