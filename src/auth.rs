use axum::{
    extract::Form,
    http::StatusCode,
    routing::get,
    routing::post,
    Json,
    Router,
};
use crate::routes::search;
use crate::routes::topics;
use serde::{Deserialize, Serialize};

use crate::db::{init_pool, generate_daily_trail, DailyTrailResponse, save_map_from_daily_trail};
use crate::services::ai;
use crate::services::classroom;
use crate::services::payments;

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

#[derive(Deserialize)]
pub struct WaitlistPayload {
    pub email: String,
}

#[derive(Serialize)]
pub struct WaitlistResponse {
    pub success: bool,
    pub message: String,
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
        .route("/search", get(search::search_topics))
        .route("/topics/:slug", get(topics::get_topic))
        .route("/topics/:slug/quiz", get(topics::get_topic_quiz))
        .route("/topics/:slug/quiz/submit", post(topics::submit_topic_quiz))
        .route("/topics/:slug/ai-explanation", get(topics::get_topic_ai_explanation))
        .route("/daily-trail", get(get_daily_trail))
        .route("/streak", get(get_streak))
        .route("/save-map-from-trail", post(save_map_from_trail))
        .route("/waitlist", post(waitlist))
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .nest("/classroom", classroom::setup_routes())
        .nest("/payments", payments::setup_routes())
}

#[axum::debug_handler]
async fn get_daily_trail() -> (StatusCode, Json<DailyTrailResponse>) {
    let pool = init_pool(&crate::db::database_url()).await.unwrap();
    // Use user_id 1 for MVP (will be replaced with real user auth later)
    let trail = generate_daily_trail(&pool, 1).await.unwrap_or_else(|_| DailyTrailResponse {
        date: chrono::Local::now().format("%Y-%m-%d").to_string(),
        topic_ids: Vec::new(),
        topics: Vec::new(),
    });
    (StatusCode::OK, Json(trail))
}

async fn get_streak() -> (StatusCode, Json<(i32, i32, String)>) {
    let pool = init_pool(&crate::db::database_url()).await.unwrap();

    let streak: (i32, i32, Option<String>) = sqlx::query_as(
        "SELECT current_streak, longest_streak, last_active_date FROM streaks WHERE user_id = 1",
    )
        .fetch_one(&pool)
        .await
        .ok()
        .unwrap_or((0, 0, None));

    let (current, longest, last) = streak;
    let last_active = last.unwrap_or_else(|| chrono::Local::now().format("%Y-%m-%d").to_string());

    (StatusCode::OK, Json((current, longest, last_active)))
}

async fn save_map_from_trail() -> StatusCode {
    let pool = init_pool(&crate::db::database_url()).await.unwrap();
    // Use user_id 1 for MVP
    let trail = generate_daily_trail(&pool, 1).await.unwrap_or_else(|_| crate::db::DailyTrailResponse {
        date: chrono::Local::now().format("%Y-%m-%d").to_string(),
        topic_ids: Vec::new(),
        topics: Vec::new(),
    });

    if trail.topic_ids.is_empty() {
        return StatusCode::BAD_REQUEST;
    }

    let map_id = save_map_from_daily_trail(&pool, 1, &trail.topic_ids).await;
    // If map_count >= 5, we still return OK but the map wasn't created
    // The frontend can handle the 5-map limit message
    StatusCode::OK
}

async fn register(Form(_payload): Form<RegisterPayload>) -> (StatusCode, Json<AuthResponse>) {
    let _pool = init_pool(&crate::db::database_url()).await.unwrap();

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
    let _pool = init_pool(&crate::db::database_url()).await.unwrap();

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

async fn waitlist(Json(_payload): Json<WaitlistPayload>) -> (StatusCode, Json<WaitlistResponse>) {
    let _pool = init_pool(&crate::db::database_url()).await.unwrap();

    // For MVP: accept any email, store for later (mock implementation)
    let response = WaitlistResponse {
        success: true,
        message: "Thank you! You've been added to the waitlist.".to_string(),
    };
    (StatusCode::OK, Json(response))
}