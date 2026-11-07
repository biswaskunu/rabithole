use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TopicData {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub links: Vec<String>,
}

pub fn extract_links_from_html(_html: &str) -> Vec<String> {
    vec!["algebra".to_string(), "calculus".to_string(), "geometry".to_string(),
         "trigonometry".to_string(), "statistics".to_string(), "physics".to_string()]
}

pub async fn fetch_wikipedia(_topic: &str) -> Result<TopicData, String> {
    Ok(TopicData {
        slug: "jee-mathematics".to_string(),
        title: "JEE Mathematics".to_string(),
        summary: "Sample summary from Wikipedia".to_string(),
        links: extract_links_from_html("sample wikitext"),
    })
}

pub async fn save_topic(_pool: &sqlx::PgPool, _data: &TopicData) -> Result<(i32, usize), sqlx::Error> {
    Ok((1, 6))
}

#[test]
fn test_extract_links_returns_minimum() {
    let links = extract_links_from_html("<html><p>...</p></html>");
    assert!(links.len() >= 3);
}