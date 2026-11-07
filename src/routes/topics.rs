use axum::extract::Path;
use axum::Json;
use serde::Serialize;

use crate::services::ingestion::TopicData;

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

pub async fn get_topic(Path(slug): Path<String>) -> Json<TopicResponse> {
    let mock_topic = TopicData {
        slug: slug.clone(),
        title: format!("Topic: {}", slug),
        summary: "Sample summary".to_string(),
        links: vec!["link1".to_string(), "link2".to_string(), "link3".to_string(),
                    "link4".to_string(), "link5".to_string(), "link6".to_string()],
    };

    let mock_related = vec![
        RelatedTopic { id: 1, title: "Algebra".to_string(), slug: "algebra".to_string(), relation_type: "related".to_string() },
        RelatedTopic { id: 2, title: "Calculus".to_string(), slug: "calculus".to_string(), relation_type: "related".to_string() },
        RelatedTopic { id: 3, title: "Geometry".to_string(), slug: "geometry".to_string(), relation_type: "related".to_string() },
        RelatedTopic { id: 4, title: "Trigonometry".to_string(), slug: "trigonometry".to_string(), relation_type: "related".to_string() },
        RelatedTopic { id: 5, title: "Statistics".to_string(), slug: "statistics".to_string(), relation_type: "related".to_string() },
        RelatedTopic { id: 6, title: "Physics".to_string(), slug: "physics".to_string(), relation_type: "related".to_string() },
    ];

    Json(TopicResponse {
        topic: mock_topic,
        related: mock_related,
    })
}
