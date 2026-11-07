use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TopicData {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub links: Vec<String>,
}

pub async fn fetch_wikipedia(_topic: &str) -> Result<TopicData, String> {
    // Stub: returns sample data for compilation
    // Actual implementation will fetch from Wikipedia API
    Ok(TopicData {
        slug: "jee-mathematics".to_string(),
        title: "JEE Mathematics".to_string(),
        summary: "Sample summary from Wikipedia".to_string(),
        links: vec!["algebra".to_string(), "calculus".to_string(), "geometry".to_string(),
                    "trigonometry".to_string(), "statistics".to_string()],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_returns_data() {
        let data = fetch_wikipedia("test").unwrap();
        assert!(!data.slug.is_empty());
        assert!(!data.title.is_empty());
        assert!(!data.summary.is_empty());
        assert!(data.links.len() >= 3);
    }
}
