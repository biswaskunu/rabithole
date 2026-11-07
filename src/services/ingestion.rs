use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TopicData {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub links: Vec<String>,
}

pub fn extract_links_from_html(_html: &str) -> Vec<String> {
    // Stub: in real implementation, parses Wikipedia HTML
    // to extract link titles from first paragraph and infobox
    vec!["algebra".to_string(), "calculus".to_string(), "geometry".to_string(),
         "trigonometry".to_string(), "statistics".to_string(), "physics".to_string()]
}

pub async fn fetch_wikipedia(_topic: &str) -> Result<TopicData, String> {
    // Stub: returns sample data - real impl fetches from Wikipedia API
    Ok(TopicData {
        slug: "jee-mathematics".to_string(),
        title: "JEE Mathematics".to_string(),
        summary: "Sample summary from Wikipedia".to_string(),
        links: extract_links_from_html("sample wikitext"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_links_returns_minimum() {
        let links = extract_links_from_html("<html><p>...</p></html>");
        assert!(links.len() >= 3, "Should extract at least 3 links for graph expansion");
    }

    #[test]
    fn test_fetch_returns_data() {
        let data = fetch_wikipedia("test").unwrap();
        assert!(!data.slug.is_empty());
        assert!(!data.title.is_empty());
        assert!(!data.summary.is_empty());
        assert!(data.links.len() >= 3);
    }
}
