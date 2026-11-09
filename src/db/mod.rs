use sqlx::{PgPool, Row};
use serde::Serialize;

pub async fn init_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = sqlx::PgPool::connect(database_url).await?;
    sqlx::query(
        "INSERT INTO streaks (user_id, current_streak, longest_streak, last_active_date) SELECT 0, 0, 0, CURRENT_DATE WHERE NOT EXISTS (SELECT 1 FROM streaks LIMIT 1)",
    )
        .execute(&pool)
        .await
        .ok();
    Ok(pool)
}

pub async fn health_check(pool: &PgPool) -> bool {
    sqlx::query("SELECT 1 AS one")
        .fetch_one(pool)
        .await
        .is_ok()
}

#[derive(Serialize, sqlx::FromRow)]
pub struct DailyTrailTopic {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub summary: String,
}

#[derive(Serialize)]
pub struct DailyTrailResponse {
    pub date: String,
    pub topic_ids: Vec<i32>,
    pub topics: Vec<DailyTrailTopic>,
}

pub async fn generate_daily_trail(pool: &PgPool, user_id: i32) -> Result<DailyTrailResponse, sqlx::Error> {
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();

    let existing = sqlx::query(
        "SELECT date, topic_ids FROM daily_trails WHERE date = $1",
    )
        .bind(&today)
        .fetch_optional(pool)
        .await
        .ok()
        .flatten();

    let topic_ids: Vec<i32> = if let Some(ref row) = existing {
        row.get::<Vec<i32>, _>("topic_ids")
    } else {
        let all_topics: Vec<(i32, String, String)> = sqlx::query_as(
            "SELECT id, slug, title FROM topics ORDER BY random() LIMIT 50",
        )
            .fetch_all(pool)
            .await
            .unwrap_or_default();

        let mut selected: Vec<i32> = Vec::new();
        let mut seen: std::collections::HashSet<i32> = std::collections::HashSet::new();

        for topic in &all_topics {
            if !seen.contains(&topic.0) && selected.len() < 5 {
                selected.push(topic.0);
                seen.insert(topic.0);
            }
            if selected.len() >= 5 {
                break;
            }
        }
        if selected.len() < 5 {
            all_topics.iter().take(5).map(|t| t.0).collect()
        } else {
            selected
        }
    };

    if existing.is_some() {
        sqlx::query(
            "UPDATE daily_trails SET topic_ids = $1::integer[], generated_at = NOW() WHERE date = $2",
        )
            .bind(&topic_ids)
            .bind(&today)
            .execute(pool)
            .await
            .ok();
    } else {
        sqlx::query(
            "INSERT INTO daily_trails (date, topic_ids) VALUES ($1::date, $2::integer[])",
        )
            .bind(&today)
            .bind(&topic_ids)
            .execute(pool)
            .await
            .ok();
    }

    let topics: Vec<DailyTrailTopic> = if topic_ids.is_empty() {
        Vec::new()
    } else {
        let placeholders: String = topic_ids.iter()
            .map(|_| "$1")
            .collect::<Vec<_>>()
            .join(", ");

        let query = if topic_ids.len() == 1 {
            format!("SELECT id, slug, title, summary FROM topics WHERE id = $1")
        } else {
            format!("SELECT id, slug, title, summary FROM topics WHERE id IN ({})", placeholders)
        };

        let params: Vec<i32> = topic_ids.iter().map(|id| *id).collect();
        let rows = sqlx::query_as::<_, DailyTrailTopic>(&query)
            .fetch_all(pool)
            .await
            .unwrap_or_default();
        rows
    };

    Ok(DailyTrailResponse {
        date: today,
        topic_ids,
        topics,
    })
}

pub async fn update_streak(pool: &PgPool, user_id: i32, completed_today: bool) -> Result<(i32, i32), sqlx::Error> {
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();

    let streak: (i32, i32, Option<String>) = sqlx::query_as(
        "SELECT current_streak, longest_streak, last_active_date FROM streaks WHERE user_id = $1",
    )
        .bind(user_id)
        .fetch_one(pool)
        .await
        .ok()
        .unwrap_or((0, 0, None));

    let (current_streak, longest_streak, last_active) = streak;

    let (new_current, new_longest) = if let Some(last_date) = last_active {
        if last_date == today {
            (current_streak + 1, longest_streak.max(current_streak + 1))
        } else if !completed_today {
            (0, longest_streak)
        } else {
            (current_streak + 1, longest_streak.max(current_streak + 1))
        }
    } else {
        if completed_today {
            (1, 1)
        } else {
            (0, 0)
        }
    };

    sqlx::query(
        "UPDATE streaks SET current_streak = $1, longest_streak = $2, last_active_date = $3 WHERE user_id = $4",
    )
        .bind(new_current)
        .bind(new_longest)
        .bind(&today)
        .bind(user_id)
        .execute(pool)
        .await
        .ok();

    sqlx::query(
        "INSERT INTO streaks (user_id, current_streak, longest_streak, last_active_date) SELECT $1, $2, $3, $4 WHERE NOT EXISTS (SELECT 1 FROM streaks WHERE user_id = $5)",
    )
        .bind(user_id as i32)
        .bind(new_current)
        .bind(new_longest)
        .bind(&today)
        .bind(user_id as i32)
        .execute(pool)
        .await
        .ok();

    Ok((new_current, new_longest))
}

pub async fn save_map_from_daily_trail(pool: &PgPool, user_id: i32, topic_ids: &[i32]) -> Result<i32, sqlx::Error> {
    let map_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM maps WHERE user_id = $1")
        .bind(user_id)
        .fetch_one(pool)
        .await
        .map(|(c,)| c)
        .unwrap_or((0,));

    if map_count >= (5,) {
        return Err(sqlx::Error::RowNotFound);
    }

    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let map_title = format!("Daily Trail - {}", today);

    let map_id: i32 = sqlx::query_scalar(
        "INSERT INTO maps (user_id, title, created_at, is_public) VALUES ($1, $2, NOW(), FALSE) RETURNING id",
    )
        .bind(user_id)
        .bind(&map_title)
        .fetch_one(pool)
        .await
        .map(|id| id)
        .unwrap_or(0);

    if map_id > 0 {
        for (i, &topic_id) in topic_ids.iter().enumerate() {
            sqlx::query(
                "INSERT INTO map_nodes (map_id, topic_id, position_x, position_y) VALUES ($1, $2, $3, $4)",
            )
                .bind(map_id)
                .bind(topic_id)
                .bind(i as f32 * 100.0)
                .bind(0.0)
                .execute(pool)
                .await
                .ok();

            if i > 0 {
                sqlx::query(
                    "INSERT INTO topic_edges (from_topic_id, to_topic_id, relation_type) VALUES ($1, $2, 'related')",
                )
                    .bind(topic_ids[i - 1])
                    .bind(topic_id)
                    .execute(pool)
                    .await
                    .ok();
            }
        }
    }

    Ok(map_id)
}

pub async fn find_related_topics(pool: &PgPool, from_topic_id: i32, hops: i32) -> Result<Vec<i32>, sqlx::Error> {
    let rows = sqlx::query(
        "WITH RECURSIVE traversal AS (
            SELECT to_topic_id AS id FROM topic_edges WHERE from_topic_id = $1
            UNION ALL
            SELECT te.to_topic_id FROM topic_edges te JOIN traversal t ON te.from_topic_id = t.id
            WHERE t.hops < $2
        )
        SELECT DISTINCT id FROM traversal",
    )
        .bind(from_topic_id)
        .bind(hops)
        .fetch_all(pool)
        .await?;

    let topic_ids: Vec<i32> = rows.into_iter().map(|row| row.get::<i32, _>("id")).collect();
    Ok(topic_ids)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    #[tokio::test]
    async fn test_find_related_topics_requires_hops() {
        let pool = init_pool("postgres://rabithole:password@localhost:5432/rabithole?search_path=public")
            .await
            .expect("Failed to create pool");
        let result = find_related_topics(&pool, 1, 2).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_init_pool_creates_connection() {
        let pool = init_pool("postgres://rabithole:password@localhost:5432/rabithole?search_path=public")
            .await
            .expect("Failed to create pool");
        let healthy = health_check(&pool).await;
        assert!(healthy);
    }
}
