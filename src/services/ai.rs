use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AiExplanationRequest {
    pub topic_id: i32,
    pub topic_slug: String,
    pub topic_title: String,
}

#[derive(Debug, Serialize)]
pub struct AiExplanationResponse {
    pub topic_id: i32,
    pub topic_slug: String,
    pub topic_title: String,
    pub explanation: String,
    pub usage_count: i32,
    pub tier_limit: i32,
    pub remaining: i32,
}

#[derive(Debug, Serialize)]
pub struct AiUsageResponse {
    pub total_used: i32,
    pub tier_limit: i32,
    pub remaining: i32,
    pub is_unlimited: bool,
}

pub struct AiExplanationService;

impl AiExplanationService {
    pub async fn get_explanation(
        pool: &sqlx::PgPool,
        request: &AiExplanationRequest,
        user_id: i32,
    ) -> Result<AiExplanationResponse, StatusCode> {
        // Check user's tier and usage limits

        let tier: String = sqlx::query_scalar("SELECT tier FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_one(pool)
            .await
            .unwrap_or("free".to_string());

        // Determine usage limits based on tier
        let (tier_limit, is_unlimited) = if tier == "paid" {
            (0, true) // Paid: unlimited
        } else {
            (3, false) // Free: 3 explanations per day
        };

        // Check current usage for today
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let total_used: i32 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM ai_explanations WHERE topic_id = $1 AND created_at >= $2",
        )
            .bind(request.topic_id)
            .bind(&today)
            .fetch_one(pool)
            .await
            .unwrap_or(0);

        if !is_unlimited && total_used >= tier_limit {
            return Err(StatusCode::TOO_MANY_REQUESTS);
        }

        // Generate AI explanation (mock for MVP - in production calls OpenAI/Gemini API)
        let explanation = Self::generate_mock_explanation(&request.topic_title);

        // Record the explanation
        let _ = sqlx::query(
            "INSERT INTO ai_explanations (topic_id, user_id, explanation, created_at) VALUES ($1, $2, $3, NOW())",
        )
            .bind(request.topic_id as i32)
            .bind(user_id)
            .bind(&explanation)
            .execute(pool)
            .await;

        let remaining = if is_unlimited { -1 } else { tier_limit - total_used - 1 };

        Ok(AiExplanationResponse {
            topic_id: request.topic_id,
            topic_slug: request.topic_slug.clone(),
            topic_title: request.topic_title.clone(),
            explanation,
            usage_count: total_used + 1,
            tier_limit,
            remaining,
        })
    }

    fn generate_mock_explanation(title: &str) -> String {
        format!("AI-generated explanation about {}: This topic explores the fundamental concepts and principles related to {}, providing insights into how it works and its applications in various contexts.", title, title)
    }

    pub async fn get_usage(
        pool: &sqlx::PgPool,
        user_id: i32,
    ) -> Result<AiUsageResponse, StatusCode> {
        // Check user's tier and usage limits
        let tier: String = sqlx::query_scalar("SELECT tier FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_one(pool)
            .await
            .unwrap_or("free".to_string());

        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let total_used: i32 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM ai_explanations WHERE user_id = $1 AND created_at >= $2",
        )
            .bind(user_id)
            .bind(&today)
            .fetch_one(pool)
            .await
            .unwrap_or(0);

        let (tier_limit, is_unlimited) = if tier == "paid" {
            (-1, true)
        } else {
            (3, false)
        };

        let remaining = if is_unlimited {
            -1
        } else {
            tier_limit - total_used
        };

        Ok(AiUsageResponse {
            total_used,
            tier_limit: if is_unlimited { -1 } else { tier_limit },
            remaining,
            is_unlimited,
        })
    }
}