use axum::{
    extract::Form,
    http::StatusCode,
    routing::post,
    Json,
    Router,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RegisterPayload {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

pub fn setup_routes() -> Router {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
}

async fn register(Form(_payload): Form<RegisterPayload>) -> (StatusCode, Json<AuthResponse>) {
    // TODO: hash password & persist to users table (Phase 0 schema)
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
    // TODO: verify credentials against DB
    // For MVP: accept any credentials, issue mock tokens
    let token = "mock-access-token-for-dev";
    let body = AuthResponse {
        access_token: token.to_string(),
        refresh_token: "mock-refresh-token".to_string(),
        token_type: "Bearer".to_string(),
        expires_in: 3600,
    };
    (StatusCode::OK, Json(body))
}
