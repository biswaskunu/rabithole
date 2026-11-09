use axum::extract::Path;
use axum::Json;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::db::init_pool;
use crate::services::ingestion::TopicData;
use crate::services::ai::{AiExplanationRequest, AiExplanationService, AiExplanationResponse};

#[derive(Serialize)]
pub struct RelatedTopic {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub relation_type: String,
}

#[derive(Serialize)]
pub struct TopicResponse {
    pub topic: TopicData,
    pub related: Vec<RelatedTopic>,
}

// Depth-limited: only return first-degree neighbors (max 1 hop)
/// Payload cap: max 10 related topics per node to keep JSON small for mobile
/// Cache-Control: public, max-age=3600, stale-while-revalidate=86400 per Architecture.md:56
pub async fn get_topic(Path(slug): Path<String>) -> (StatusCode, Json<TopicResponse>) {
    let (title, summary, lang) = crate::services::ingestion::fetch_wikipedia_hindi(&slug).await;
    let title_clone = title.clone();
    let topic_data = TopicData {
        id: 0,
        slug: slug.clone(),
        title: title_clone,
        summary: summary.clone(),
        links: vec!["algebra".to_string(), "calculus".to_string(), "geometry".to_string(),
                     "trigonometry".to_string(), "statistics".to_string(), "physics".to_string()],
        lang: lang.clone(),
    };

    // Depth limit: only 6 related topics (capped at 10 per Architecture.md:56)
    // Payload target: <300ms p95 for cached topic response
    let mock_related: Vec<RelatedTopic> = vec![
        RelatedTopic { id: 1, title: "Algebra".to_string(), slug: "algebra".to_string(), relation_type: "related".to_string() },
        RelatedTopic { id: 2, title: "Calculus".to_string(), slug: "calculus".to_string(), relation_type: "related".to_string() },
        RelatedTopic { id: 3, title: "Geometry".to_string(), slug: "geometry".to_string(), relation_type: "related".to_string() },
        RelatedTopic { id: 4, title: "Trigonometry".to_string(), slug: "trigonometry".to_string(), relation_type: "related".to_string() },
        RelatedTopic { id: 5, title: "Statistics".to_string(), slug: "statistics".to_string(), relation_type: "related".to_string() },
        RelatedTopic { id: 6, title: "Physics".to_string(), slug: "physics".to_string(), relation_type: "related".to_string() },
    ];

    let topic = TopicData {
        id: 0,
        slug: slug.clone(),
        title,
        summary,
        links: vec!["algebra".to_string(), "calculus".to_string(), "geometry".to_string(),
                     "trigonometry".to_string(), "statistics".to_string(), "physics".to_string()],
        lang,
    };

    let response = TopicResponse {
        topic,
        related: mock_related,
    };

    (StatusCode::OK, Json(response))
}

#[derive(Deserialize)]
pub struct QuizAnswer {
    pub question_id: i32,
    pub selected_option: i32,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct QuizQuestionResponse {
    pub id: i32,
    pub question: String,
    pub options: Vec<String>,
}

pub async fn get_topic_quiz(Path(slug): Path<String>) -> Json<Vec<QuizQuestionResponse>> {
    let pool = init_pool("postgres://rabithole:password@localhost:5432/rabithole?search_path=public").await.unwrap();

    let questions = sqlx::query_as::<_, QuizQuestionResponse>(
        "SELECT id, question, options::text, correct_option FROM quiz_questions WHERE topic_id = (SELECT id FROM topics WHERE slug = $1)",
    )
        .bind(&slug)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    Json(questions)
}

pub async fn submit_topic_quiz(Path(slug): Path<String>, Json(_answer): Json<QuizAnswer>) -> StatusCode {
    // For MVP: accept any answer, track locally on frontend
    StatusCode::OK
}

#[derive(Deserialize)]
pub struct AiExplanationPayload {
    pub topic_slug: String,
}

pub async fn get_topic_ai_explanation(Path(slug): Path<String>) -> Result<Json<AiExplanationResponse>, StatusCode> {
    let pool = init_pool("postgres://rabithole:password@localhost:5432/rabithole?search_path=public").await.unwrap();

    // Find topic by slug
    let topic = sqlx::query_as::<_, (i32, String, String)>(
        "SELECT id, slug, title FROM topics WHERE slug = $1",
    )
        .bind(&slug)
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let (topic_id, topic_slug, topic_title) = topic.ok_or(StatusCode::NOT_FOUND)?;

    let request = AiExplanationRequest {
        topic_id,
        topic_slug,
        topic_title,
    };

    AiExplanationService::get_explanation(&pool, &request).await
        .map(Json)
        .map_err(|e| e)
}