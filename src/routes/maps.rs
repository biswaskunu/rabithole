use axum::{
    extract::Query,
    http::StatusCode,
    Json,
    routing::{get, post, delete},
};
use serde::Deserialize;

use crate::db::init_pool;
use crate::services::ingestion::TopicData;

#[derive(Deserialize)]
pub struct SaveMapPayload {
    pub title: String,
}

pub struct MapSummary {
    pub id: String,
    pub title: String,
    pub node_count: usize,
}

type SavedMap = {
    id: String,
    title: String,
    nodes: Vec<String>,
    edges: Vec<String>,
    createdAt: u64,
};

type MapList = Vec<MapSummary>;

#[derive(Serialize)]
pub struct MapSaveResponse {
    map: SavedMap,
    current_count: usize,
    limit: usize,
}

#[derive(Serialize)]
pub struct MapsListResponse {
    maps: MapList,
    current_count: usize,
    limit: usize,
}

async fn save_map() -> (StatusCode, Json<MapSaveResponse>) {
    let pool = init_pool("postgres://rabithole:password@localhost:5432/rabithole?search_path=public").await.unwrap();

    // Check user's tier and map count
    let user_id: i32 = 1; // MVP: using user_id 1 (will be replaced with real auth)

    let map_count: i64 = sqlx::query_as("SELECT COUNT(*) FROM maps WHERE user_id = $1")
        .bind(user_id)
        .fetch_one(&pool)
        .await
        .unwrap_or((0,));

    // Check tier - paid tier has unlimited maps
    let tier: String = sqlx::query_scalar("SELECT tier FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(&pool)
        .await
        .unwrap_or("free".to_string());

    let limit = if tier == "paid" { 0 } else { 5 };

    if map_count >= limit && limit > 0 {
        return (
            StatusCode::CONFLICT,
            Json(MapSaveResponse {
                map: {
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
                    SavedMap {
                        id: format!("map-{}", now),
                        title: "Untitled Map".to_string(),
                        nodes: vec![],
                        edges: vec![],
                        createdAt: now,
                    }
                },
                current_count: map_count as usize,
                limit,
            }),
        );
    }

    // Save the map
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let map: SavedMap = {
        SavedMap {
            id: format!("map-{}", now),
            title: "New Map".to_string(),
            nodes: vec![],
            edges: vec![],
            createdAt: now,
        }
    };

    (
        StatusCode::CREATED,
        Json(MapSaveResponse {
            map,
            current_count: map_count as usize + 1,
            limit,
        }),
    )
}

async fn list_maps() -> (StatusCode, Json<MapListsResponse>) {
    let pool = init_pool("postgres://rabithole:password@localhost:5432/rabithole?search_path=public").await.unwrap();

    let user_id: i32 = 1; // MVP: using user_id 1

    let map_count: i64 = sqlx::query_as("SELECT COUNT(*) FROM maps WHERE user_id = $1")
        .bind(user_id)
        .fetch_one(&pool)
        .await
        .unwrap_or((0,));

    let tier: String = sqlx::query_scalar("SELECT tier FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(&pool)
        .await
        .unwrap_or("free".to_string());

    let limit = if tier == "paid" { 0 } else { 5 };

    let maps: MapList = vec![
        MapSummary {
            id: "map-1".to_string(),
            title: "My First Map".to_string(),
            node_count: 5,
        },
        MapSummary {
            id: "map-2".to_string(),
            title: "Science Map".to_string(),
            node_count: 3,
        },
    ];

    (
        StatusCode::OK,
        Json(MapListsResponse {
            maps,
            current_count: map_count as usize,
            limit,
        }),
    )
}

async fn delete_map() -> StatusCode {
    // Mock: delete map successfully
    StatusCode::OK
}

async fn export_map() -> (StatusCode, String) {
    // Mock: return SVG representation of the map for sharing
    let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="400" height="400">
  <rect width="400" height="400" fill="nightsky"/>
  <circle cx="100" cy="100" r="40" fill="amber" />
  <circle cx="200" cy="200" r="40" fill="amber" />
  <line x1="100" y1="100" x2="200" y2="200" stroke="white" stroke-width="2"/>
  <text x="200" y="390" text-anchor="middle" fill="white" font-family="sans-serif" font-size="14">My RabitHole Map</text>
</svg>"#
    .to_string();
    (StatusCode::OK, svg)
}