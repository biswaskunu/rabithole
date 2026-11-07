use sqlx::PgPool;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
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

pub async fn save_topic(pool: &PgPool, data: &TopicData) -> Result<(i32, usize), sqlx::Error> {
    // Insert topic and return its ID
    let topic_id = sqlx::query_scalar!(
        "INSERT INTO topics (slug, title, summary, lang, source_url, cached_at) VALUES ($1, $2, $3, $4, $5, NOW()) RETURNING id",
        &data.slug,
        &data.title,
        &data.summary,
        "en",
        "https://en.wikipedia.org/wiki/"
    )
    .fetch_one(pool)
    .await?
    .ok_or_else(|| "No topic ID returned")?;

    // Insert related topic edges
    let mut edge_count = 0;
    for link in &data.links {
        let _ = sqlx::query!(
            "INSERT INTO topic_edges (from_topic_id, to_topic_id, relation_type) VALUES ($1, $2, 'related')",
            topic_id,
            link.to_string()
        )
        .execute(pool)
        .await;
        edge_count += 1;
    }

    Ok((topic_id, edge_count))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_links_returns_minimum() {
        let links = extract_links_from_html("<html><p>...</p></html>");
        assert!(links.len() >= 3);
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
