use axum::{
    extract::Form,
    http::StatusCode,
    routing::post,
    Json,
    Router,
};
use serde::{Deserialize, Serialize};

use crate::db::init_pool;

#[derive(Deserialize)]
pub struct RegisterPayload {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

fn generate_token(email: &str) -> String {
    use std::time::SystemTime;
    let now = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    format!("{}:{}", email, now)
}

pub fn setup_routes() -> Router {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
}

async fn register(Form(_payload): Form<RegisterPayload>) -> (StatusCode, Json<AuthResponse>) {
    let _pool = init_pool("postgres://rabithole:password@localhost:5432/rabithole?search_path=public").await.unwrap();

    // For MVP: accept any credentials, issue mock tokens (like original code)
    let token = "mock-access-token-for-dev";
    let body = AuthResponse {
        access_token: token.to_string(),
        refresh_token: "mock-refresh-token".to_string(),
        token_type: "Bearer".to_string(),
        expires_in: 3600,
    };
    (StatusCode::CREATED, Json(body))
}

async fn login(Form(_payload): Form<LoginPayload>) -> (StatusCode, Json<AuthResponse>) {
    let _pool = init_pool("postgres://rabithole:password@localhost:5432/rabithole?search_path=public").await.unwrap();

    // For MVP: accept any credentials, issue mock tokens (like original code)
    let token = "mock-access-token-for-dev";
    let body = AuthResponse {
        access_token: token.to_string(),
        refresh_token: "mock-refresh-token".to_string(),
        token_type: "Bearer".to_string(),
        expires_in: 3600,
    };
    (StatusCode::OK, Json(body))
}