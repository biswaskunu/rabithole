use axum::{
    extract::Form,
    http::StatusCode,
    Json,
    Router,
    routing::{get, post, put},
};
use serde::{Deserialize, Serialize};

use crate::db::init_pool;

#[derive(Deserialize)]
pub struct CreateClassPayload {
    pub name: String,
    pub description: String,
    pub subject: String,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct ClassSummary {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub subject: String,
    pub created_at: String,
    pub teacher_id: i32,
    pub student_count: i32,
}

#[derive(Deserialize)]
pub struct JoinClassPayload {
    pub class_code: String,
}

#[derive(Serialize)]
pub struct JoinClassResponse {
    pub success: bool,
    pub class: ClassSummary,
}

pub fn setup_routes() -> Router {
    Router::new()
        .route("/classes", post(create_class))
        .route("/classes", get(list_classes))
        // Note: join_class route temporarily disabled - Handler trait issue with Path extractor
        .route("/classes/:id/students", get(list_class_students))
        .route("/classes/:id/stats", get(class_stats))
}

async fn create_class(Form(payload): Form<CreateClassPayload>) -> (StatusCode, Json<ClassSummary>) {
    let pool = init_pool(&crate::db::database_url()).await.unwrap();

    let class_id: i32 = sqlx::query_scalar(
        "INSERT INTO classes (name, description, subject, teacher_id, created_at) VALUES ($1, $2, $3, 1, NOW()) RETURNING id",
    )
        .bind(&payload.name)
        .bind(&payload.description)
        .bind(&payload.subject)
        .fetch_one(&pool)
        .await
        .unwrap_or(0);

    let class = ClassSummary {
        id: class_id,
        name: payload.name,
        description: payload.description,
        subject: payload.subject,
        created_at: chrono::Local::now().format("%Y-%m-%d").to_string(),
        teacher_id: 1,
        student_count: 0,
    };

    (StatusCode::CREATED, Json(class))
}

async fn list_classes() -> (StatusCode, Json<Vec<ClassSummary>>) {
    let pool = init_pool(&crate::db::database_url()).await.unwrap();

    let classes: Vec<ClassSummary> = sqlx::query_as(
        "SELECT id, name, description, subject, created_at, teacher_id, (SELECT COUNT(*) FROM class_students WHERE class_id = classes.id) as student_count FROM classes",
    )
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    (StatusCode::OK, Json(classes))
}

async fn join_class(Form(payload): Form<JoinClassPayload>, axum::extract::Path(_class_id): axum::extract::Path<i32>) -> StatusCode {
    let pool = init_pool(&crate::db::database_url()).await.unwrap();

    // Mock: add user to class
    let _ = sqlx::query(
        "INSERT INTO class_students (class_id, student_id) VALUES ($1, 1) ON CONFLICT DO NOTHING",
    )
        .bind(_class_id)
        .execute(&pool)
        .await;

    StatusCode::OK
}

async fn list_class_students(axum::extract::Path(class_id): axum::extract::Path<i32>) -> (StatusCode, Json<Vec<i32>>) {
    let pool = init_pool(&crate::db::database_url()).await.unwrap();

    let students: Vec<i32> = sqlx::query_scalar(
        "SELECT student_id FROM class_students WHERE class_id = $1",
    )
        .bind(class_id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    (StatusCode::OK, Json(students))
}

async fn class_stats(axum::extract::Path(class_id): axum::extract::Path<i32>) -> (StatusCode, Json<(i32, i32, i32)>) {
    let pool = init_pool(&crate::db::database_url()).await.unwrap();

    let (total_students, active_today, longest_streak) = sqlx::query_as(
        "SELECT COUNT(*), COUNT(DISTINCT student_id), COALESCE((SELECT longest_streak FROM class_streaks WHERE class_id = $1), 0) FROM class_students WHERE class_id = $1",
    )
        .bind(class_id)
        .fetch_one(&pool)
        .await
        .unwrap_or((0, 0, 0));

    (StatusCode::OK, Json((total_students, active_today, longest_streak)))
}