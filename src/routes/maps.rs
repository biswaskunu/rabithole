use axum::{
    extract::Query,
    http::StatusCode,
    Json,
    routing::{get, post, delete},
};
use serde::Deserialize;

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

pub fn setup_routes() -> Router {
    Router::new()
        .route("/maps", post(save_map))
        .route("/maps", get(list_maps))
        .route("/maps/:id", delete(delete_map))
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
    // Mock: enforce 5-map limit
    let current_count = 3; // mock: user has 3 maps
    let limit = 5;

    if current_count >= limit {
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
                current_count: current_count,
                limit,
            }),
        );
    }

    // Mock: save the map
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
            current_count: current_count + 1,
            limit,
        }),
    )
}

async fn list_maps() -> (StatusCode, Json<MapListsResponse>) {
    // Mock: return user's maps
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
            current_count: 2,
            limit: 5,
        }),
    )
}

async fn delete_map() -> StatusCode {
    // Mock: delete map successfully
    StatusCode::OK
}