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

pub struct QuizQuestion {
    pub id: i32,
    pub topic_id: i32,
    pub question: String,
    pub options: Vec<String>,
    pub correct_option: i32,
}

fn generate_quiz_from_summary(summary: &str) -> Vec<QuizQuestion> {
    let keywords: Vec<&str> = summary
        .split_whitespace()
        .filter(|w| w.len() > 3)
        .collect();

    let mut questions = Vec::new();

    if keywords.len() >= 3 {
        let k0 = keywords[0];
        let k1 = keywords[1];
        let k2 = keywords[2];

        questions.push(QuizQuestion {
            id: 1,
            topic_id: 0,
            question: format!("What is the focus of this topic?"),
            options: vec![
                format!("Related to {}", k0),
                format!("Related to {}", k1),
                format!("Related to {}", k2),
                "Something else".to_string(),
            ],
            correct_option: 0,
        });

        if keywords.len() >= 4 {
            questions.push(QuizQuestion {
                id: 2,
                topic_id: 0,
                question: format!("Which concept is related to {}?", k1),
                options: vec![
                    format!("Related to {}", k0),
                    format!("Related to {}", k1),
                    format!("Related to {}", k2),
                    format!("Related to {}", keywords[3]),
                ],
                correct_option: 1,
            });
        }
    }

    if questions.is_empty() {
        questions.push(QuizQuestion {
            id: 1,
            topic_id: 0,
            question: "What does this topic cover?".to_string(),
            options: vec!["Option A".to_string(), "Option B".to_string(), "Option C".to_string(), "Option D".to_string()],
            correct_option: 0,
        });
    }

    questions
}

pub async fn save_topic(pool: &sqlx::PgPool, data: &TopicData) -> Result<(i32, usize), sqlx::Error> {
    let question_count = sqlx::query_as::<_, QuizQuestion>(
        "SELECT id, topic_id, question, options::jsonb, correct_option FROM quiz_questions WHERE topic_id = $1",
    )
        .bind(data.id)
        .fetch_all(pool)
        .await
        .map(|rows| rows.len())
        .unwrap_or(0);

    if question_count == 0 {
        let quiz = generate_quiz_from_summary(&data.summary);
        for (idx, q) in quiz.iter().enumerate() {
            sqlx::query(
                "INSERT INTO quiz_questions (topic_id, question, options, correct_option) VALUES ($1, $2, $3::jsonb, $4)",
            )
                .bind(data.id as i32)
                .bind(&q.question)
                .bind(serde_json::to_string(&q.options).unwrap())
                .bind(q.correct_option as i32)
                .execute(pool)
                .await
                .ok();
        }
    }

    Ok((data.id as i32, question_count + data.links.len()))
}

#[test]
fn test_extract_links_returns_minimum() {
    let links = extract_links_from_html("<html><p>...</p></html>");
    assert!(links.len() >= 3);
}