use axum::extract::Query;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;

use crate::services::ingestion::TopicData;

#[derive(Deserialize)]
pub struct SearchParams {
    pub q: String,
}

#[derive(Serialize)]
pub struct SearchResult {
    pub query: String,
    pub results: Vec<TopicData>,
}

pub async fn search_topics(Query(params): Query<SearchParams>) -> Json<SearchResult> {
    let query = params.q.clone();
    // Stub: return mock results matching the query
    let mut results = Vec::new();

    if query.to_lowercase().contains("jee") {
        results.push(TopicData {
            id: 0,
            slug: "jee-mathematics".to_string(),
            title: "JEE Mathematics".to_string(),
            summary: "Joint Entrance Examination Mathematics".to_string(),
            links: vec!["algebra".to_string(), "calculus".to_string()],
        });
    }

    if query.to_lowercase().contains("physics") {
        results.push(TopicData {
            id: 0,
            slug: "physics-mechanics".to_string(),
            title: "Physics Mechanics".to_string(),
            summary: "Laws of motion and forces".to_string(),
            links: vec!["newton".to_string(), "kinematics".to_string()],
        });
    }

    if results.is_empty() {
        results.push(TopicData {
            id: 0,
            slug: "general".to_string(),
            title: "General Search".to_string(),
            summary: "No direct matches found".to_string(),
            links: vec![],
        });
    }

    Json(SearchResult { query, results })
}
