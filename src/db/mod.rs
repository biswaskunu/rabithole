use sqlx::PgPool;

pub async fn init_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    // Use database URL with search_path=public to ensure tables are found
    let pool = sqlx::PgPool::connect(database_url).await?;
    // Ensure streaks row exists for new users
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

pub async fn generate_daily_trail(pool: &PgPool, user_id: i32) -> Result<DailyTrailResponse, sqlx::Error> {
    // Check if today's trail already exists
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    
    let existing: Option<sqlx::Row> = sqlx::query_as(
        "SELECT date, topic_ids FROM daily_trails WHERE date = $1",
    )
        .bind(&today)
        .fetch_optional(pool)
        .await
        .ok()
        .flatten();

    let topic_ids: Vec<i32> = if let Some(row) = existing {
        row.get::<Vec<i32>, _>("topic_ids")
    } else {
        // Pick 5 topics: prioritize topics user hasn't seen recently,
        // mix of familiar and new, based on topic_edges connectivity
        let all_topics: Vec<(i32, String, String)> = sqlx::query_as(
            "SELECT id, slug, title FROM topics ORDER BY random() LIMIT 50",
        )
            .fetch_all(pool)
            .await
            .unwrap_or_default();

        // Simple algorithm: pick 5 random topics, ensure variety
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
            // Fallback: just take first 5
            all_topics.iter().take(5).map(|t| t.0).collect()
        } else {
            selected
        }
    };

    // Insert or update today's trail
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

    // Fetch topic details for response
    let topics: Vec<DailyTrailTopic> = if topic_ids.is_empty() {
        Vec::new()
    } else {
        let placeholders: String = topic_ids.iter()
            .map(|_| "$")
            .enumerate()
            .map(|(i, _)| format!("${}", i + 1))
            .collect::<Vec<_>>()
            .join(", ");

        let query = if topic_ids.len() == 1 {
            format!("SELECT id, slug, title, summary FROM topics WHERE id IN ({})", placeholders)
        } else {
            format!("SELECT id, slug, title, summary FROM topics WHERE id IN ({})", placeholders)
        };

        let params: Vec<&(dyn sqlx::Encode<sqlx::Postgres, i32>)> = topic_ids.iter().map(|id| id as &i32).collect();
        let rows = sqlx::query_as::<_, DailyTrailTopic>(&query)
            .fetch_all_by_idx(pool, &params)
            .await
            .unwrap_or_default()
    };

    Ok(DailyTrailResponse {
        date: today,
        topic_ids,
        topics,
    })
}

pub async fn update_streak(pool: &PgPool, user_id: i32, completed_today: bool) -> Result<(i32, i32), sqlx::Error> {
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();

    // Get current streak state
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
            // User was active today, increment streak
            (current_streak + 1, longest_streak.max(current_streak + 1))
        } else if !completed_today {
            // User missed a day, reset streak
            (0, longest_streak)
        } else {
            // User completed yesterday but not today yet (edge case)
            (current_streak + 1, longest_streak.max(current_streak + 1))
        }
    } else {
        // First time, set to 1 if completed_today
        if completed_today {
            (1, 1)
        } else {
            (0, 0)
        }
    };

    // Update streaks table
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

    // Ensure row exists
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

#[derive(Serialize)]
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