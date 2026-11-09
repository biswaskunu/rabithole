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
    // Use ingestion service to get topic data
    let mut results = Vec::new();

    if query.to_lowercase().contains("jee") {
        let (title, summary, lang) = crate::services::ingestion::fetch_wikipedia_hindi("jee").await;
        results.push(TopicData {
            id: 0,
            slug: "jee-mathematics".to_string(),
            title,
            summary,
            links: crate::services::ingestion::extract_links_from_html(""),
            lang,
        });
    }

    if query.to_lowercase().contains("physics") {
        let (title, summary, lang) = crate::services::ingestion::fetch_wikipedia_hindi("physics").await;
        results.push(TopicData {
            id: 0,
            slug: "physics-mechanics".to_string(),
            title,
            summary,
            links: crate::services::ingestion::extract_links_from_html(""),
            lang,
        });
    }

    if results.is_empty() {
        results.push(TopicData {
            id: 0,
            slug: "general".to_string(),
            title: "General Search".to_string(),
            summary: "No direct matches found".to_string(),
            links: vec![],
            lang: "en".to_string(),
        });
    }

    Json(SearchResult { query, results })
}
