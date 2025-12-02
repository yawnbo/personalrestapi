use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// type hell
///
/// defaults should probably be made for Stream and Game but whatever
/// these are all based on the https://api.ppvs.su/api/streams/api/streams response structure.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stream {
    pub provider: String,
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub id: i64,
    pub name: String,
    pub poster: String,
    pub start_time: i64,
    pub end_time: i64,
    pub cache_time: i64,
    pub video_link: String,
    pub category: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PpvsuApiResponse {
    pub success: bool,
    pub streams: Vec<PpvsuCategory>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PpvsuCategory {
    pub category: String,
    pub streams: Vec<PpvsuStream>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PpvsuStream {
    pub id: i64,
    pub name: String,
    pub poster: String,
    pub starts_at: i64,
    pub ends_at: i64,
    pub iframe: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PpvsuStreamDetailResponse {
    pub success: bool,
    pub data: PpvsuStreamDetail,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PpvsuStreamDetail {
    pub id: i64,
    pub name: String,
    pub poster: String,
    pub start_timestamp: i64,
    pub end_timestamp: i64,
    pub sources: Vec<PpvsuSource>,
    pub category_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PpvsuSource {
    pub data: String,
}

pub type DynStreamsRepository = Arc<dyn StreamsRepository + Send + Sync>;

#[async_trait::async_trait]
pub trait StreamsRepository {
    async fn get_stream(&self, provider: &str) -> Result<Option<Stream>>;
    async fn get_all_streams(&self) -> Result<Vec<Stream>>;
    async fn store_game(&self, provider: &str, game: &Game) -> Result<()>;
    async fn get_game(&self, provider: &str, game_id: i64) -> Result<Option<Game>>;
    async fn get_games(&self, provider: &str) -> Result<Vec<Game>>;
    async fn delete_game(&self, provider: &str, game_id: i64) -> Result<()>;
    async fn clear_cache(&self, provider: &str) -> Result<()>;
    async fn set_last_fetch_time(&self, provider: &str, timestamp: i64) -> Result<()>;
    async fn get_last_fetch_time(&self, provider: &str) -> Result<Option<i64>>;
}
