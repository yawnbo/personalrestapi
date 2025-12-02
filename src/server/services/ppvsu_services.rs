// all the stream related functions, im not commenting on all of them, they're pretty readable
use async_trait::async_trait;
use base64::Engine;
use flate2::read::GzDecoder;
use mockall::automock;
use regex::Regex;
use std::io::Read;
use std::sync::Arc;
use tracing::{error, info};

use crate::{
    database::stream::{DynStreamsRepository, Game, PpvsuApiResponse, PpvsuStreamDetailResponse},
    server::error::{AppResult, Error},
};

pub type DynPpvsuService = Arc<dyn PpvsuServiceTrait + Send + Sync>;

#[automock]
#[async_trait]
pub trait PpvsuServiceTrait {
    async fn fetch_and_cache_games(&self) -> AppResult<Vec<Game>>;
    async fn fetch_video_link(&self, iframe_url: &str) -> anyhow::Result<String>;
    async fn get_games_with_refresh(&self) -> AppResult<Vec<Game>>;
    async fn get_game_by_id(&self, game_id: i64) -> AppResult<Game>;
    async fn clear_cache(&self) -> AppResult<()>;
    async fn get_current_timestamp(&self) -> AppResult<i64>;
    async fn is_cache_stale(&self, cache_time: i64, current_time: i64) -> bool;
}

#[derive(Clone)]
pub struct PpvsuService {
    repository: DynStreamsRepository,
    http_client: reqwest::Client,
}

impl PpvsuService {
    pub fn new(repository: DynStreamsRepository) -> Self {
        // i like to make it look like a real browser but it's really not needed
        // if only there was a global function to do this for me .... FIXME:
        let http_client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:144.0) Gecko/20100101 Firefox/144.0")
            .timeout(std::time::Duration::from_secs(30))
            .http2_adaptive_window(true)
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        Self {
            repository,
            http_client,
        }
    }

    async fn refetch_game(&self, game_id: i64) -> anyhow::Result<Game> {
        info!("refetching game {} from ppvs.su API", game_id);

        let response = self
            .http_client
            .get(format!("https://api.ppvs.su/api/streams/{}", game_id))
            .header("Accept", "application/json, text/plain, */*")
            .header("Accept-Language", "en-US,en;q=0.9")
            .header("Referer", "https://api.ppvs.su/api/streams/")
            .header("Origin", "https://api.ppvs.su/api/streams")
            .header("Sec-Fetch-Dest", "empty")
            .header("Sec-Fetch-Mode", "cors")
            .header("Sec-Fetch-Site", "same-origin")
            .send()
            .await?;

        let detail_response: PpvsuStreamDetailResponse = response.json().await?;

        if !detail_response.success {
            return Err(anyhow::anyhow!("ppvs.su API returned success=false"));
        }

        let data = detail_response.data;

        let iframe = data
            .sources
            .first()
            .map(|s| s.data.clone())
            .ok_or_else(|| anyhow::anyhow!("no sources found for stream"))?;

        // previous logic of storing the games that were already at the pure link, instead i need
        // to return the iframe and decode it later so i don't get ip banned
        //
        // let video_link = self.fetch_video_link(&iframe).await?;

        let cache_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|_| anyhow::anyhow!("System time before UNIX epoch"))?
            .as_secs() as i64;

        let game = Game {
            id: data.id,
            name: data.name,
            poster: data.poster,
            start_time: data.start_timestamp,
            end_time: data.end_timestamp,
            cache_time,
            video_link: iframe,
            category: data.category_name.unwrap_or_else(|| "Unknown".to_string()),
        };

        self.repository.store_game("ppvsu", &game).await?;

        Ok(game)
    }
}

#[async_trait]
impl PpvsuServiceTrait for PpvsuService {
    async fn fetch_video_link(&self, iframe_url: &str) -> anyhow::Result<String> {
        info!("fetching video link from iframe: {}", iframe_url);

        let response = self
            .http_client
            .get(iframe_url)
            .header(
                "Accept",
                "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
            )
            .header("Accept-Language", "en-US,en;q=0.9")
            // this is literally the only one of these that actually matters
            .header("Referer", "https://api.ppvs.su/")
            .header("Sec-Fetch-Dest", "iframe")
            .header("Sec-Fetch-Mode", "navigate")
            .header("Sec-Fetch-Site", "cross-site")
            .send()
            .await?;

        let html = response.text().await?;

        let re = Regex::new(r#"const src = atob\("([^"]+)"\)"#)?;

        if let Some(caps) = re.captures(&html) {
            let base64_encoded = caps
                .get(1)
                .map(|m| m.as_str())
                .ok_or_else(|| anyhow::anyhow!("failed to extract base64 string"))?;

            let decoded_bytes = base64::engine::general_purpose::STANDARD
                .decode(base64_encoded)
                .map_err(|e| anyhow::anyhow!("failed to decode base64: {}", e))?;

            let video_link = String::from_utf8(decoded_bytes)
                .map_err(|e| anyhow::anyhow!("failed to convert decoded bytes to UTF-8: {}", e))?;

            info!("extracted and decoded video link: {}", video_link);
            Ok(video_link)
        } else {
            error!("no video link found in iframe html");
            Err(anyhow::anyhow!("video link not found in iframe"))
        }
    }
    async fn fetch_and_cache_games(&self) -> AppResult<Vec<Game>> {
        // this is to maybe avoid the 403s that happen when cloudflare bans the ip
        //
        // i don't actually think this does anything because i think i'm hitting a rate limit but
        // this makes it look more legitimate anyways so whatever
        //
        // also just going to drop the future here because there is no point for me to actually
        // check it
        let _ = self.http_client.get("https://api.ppvs.su/api/ping")
            .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:146.0) Gecko/20100101 Firefox/146.0")
            .header("Accept", "application/json")
            .header("Accept-Language", "en-US,en;q=0.5")
            .header("Accept-Encoding", "gzip, deflate, br, zstd")
            .header("Referer", "https://ppvs.su/")
            .header("Origin", "https://ppvs.su")
            .header("Sec-GPC", "1")
            .send();
        let response = self
            .http_client
            .get("https://api.ppvs.su/api/streams")
            .header("Accept", "application/json, text/plain, */*")
            .header("Accept-Language", "en-US,en;q=0.9")
            .header("Accept-Encoding", "gzip, deflate, br")
            .header("Referer", "https://api.ppvs.su/api/streams/")
            .header("Origin", "https://api.ppvs.su/api/streams")
            .header("DNT", "1")
            .header("Sec-Fetch-Dest", "empty")
            .header("Sec-Fetch-Mode", "cors")
            .header("Sec-Fetch-Site", "same-origin")
            .send()
            .await
            .map_err(|e| {
                error!("failed to fetch ppvs.su API: {}", e);
                crate::server::error::Error::InternalServerErrorWithContext(format!(
                    "failed to fetch ppvs.su API: {}",
                    e
                ))
            })?;

        info!(
            "received response from ppvs.su with status: {}",
            response.status()
        );

        let response_bytes = response.bytes().await.map_err(|e| {
            error!("failed to read response body: {}", e);
            crate::server::error::Error::InternalServerErrorWithContext(format!(
                "failed to read ppvs.su API response body: {}",
                e
            ))
        })?;

        let decoded_text =
            if response_bytes.len() > 2 && response_bytes[0] == 0x1f && response_bytes[1] == 0x8b {
                let mut decoder = GzDecoder::new(&response_bytes[..]);
                let mut decompressed = String::new();
                decoder.read_to_string(&mut decompressed).map_err(|e| {
                    error!("failed to decompress gzip response: {}", e);
                    crate::server::error::Error::InternalServerErrorWithContext(format!(
                        "failed to decompress gzip response: {}",
                        e
                    ))
                })?;
                decompressed
            } else {
                String::from_utf8(response_bytes.to_vec()).map_err(|e| {
                    error!("failed to convert response to UTF-8: {}", e);
                    crate::server::error::Error::InternalServerErrorWithContext(format!(
                        "failed to convert response to UTF-8: {}",
                        e
                    ))
                })?
            };

        let api_response: PpvsuApiResponse = serde_json::from_str(&decoded_text).map_err(|e| {
            error!("failed to parse JSON response: {}", e);
            crate::server::error::Error::InternalServerErrorWithContext(format!(
                "failed to parse ppvs.su API response: {}",
                e
            ))
        })?;

        if !api_response.success {
            return Err(crate::server::error::Error::InternalServerErrorWithContext(
                "ppvs.su API returned success=false".to_string(),
            ));
        }

        let cache_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|_| anyhow::anyhow!("System time before UNIX epoch"))?
            .as_secs() as i64;

        let mut games: Vec<Game> = Vec::new();
        let mut game_mem: Game;
        for category in api_response.streams {
            for stream in category.streams {
                if let Some(iframe) = stream.iframe.clone() {
                    game_mem = Game {
                        id: stream.id,
                        name: stream.name,
                        poster: stream.poster,
                        start_time: stream.starts_at,
                        end_time: stream.ends_at,
                        cache_time,
                        video_link: iframe.clone(),
                        category: category.category.clone(),
                    };
                    games.push(game_mem.clone());

                    self.repository.store_game("ppvsu", &game_mem).await?;
                }
            }
        }
        // this logic works fine if i want eagerly evaluate all the adless video links before
        // storing but this gets me ip banned which i don't really want so i'll decode it on fetch
        // instead
        // let mut fetch_tasks = Vec::new();

        // // fun part of making a million threads and praying they all work
        // for category in api_response.streams {
        //     for stream in category.streams {
        //         if let Some(iframe) = stream.iframe {
        //             info!("queueing stream: {} (id: {})", stream.name, stream.id);

        //             let service_clone = self.clone();
        //             let iframe_clone = iframe.clone();
        //             let stream_id = stream.id;
        //             let stream_name = stream.name.clone();
        //             let stream_poster = stream.poster.clone();
        //             let stream_starts_at = stream.starts_at;
        //             let stream_ends_at = stream.ends_at;
        //             let stream_category = category.category.clone();

        //             let task = tokio::spawn(async move {
        //                 match service_clone.fetch_video_link(&iframe_clone).await {
        //                     Ok(video_link) => {
        //                         info!(
        //                             "successfully fetched video link for stream: {}",
        //                             stream_name
        //                         );
        //                         let game = Game {
        //                             id: stream_id,
        //                             name: stream_name,
        //                             poster: stream_poster,
        //                             start_time: stream_starts_at,
        //                             end_time: stream_ends_at,
        //                             cache_time,
        //                             video_link,
        //                             category: stream_category,
        //                         };

        //                         // store immediately after fetch completes
        //                         if let Err(e) =
        //                             service_clone.repository.store_game("ppvsu", &game).await
        //                         {
        //                             error!("failed to store game {}: {}", game.id, e);
        //                             None
        //                         } else {
        //                             Some(game)
        //                         }
        //                     }
        //                     Err(e) => {
        //                         error!(
        //                             "failed to fetch video link for stream {}: {}",
        //                             stream_id, e
        //                         );
        //                         None
        //                     }
        //                 }
        //             });

        //             fetch_tasks.push(task);
        //         }
        //     }
        // }

        // info!("fetching video links for {} streams", fetch_tasks.len());

        // let results = futures::future::join_all(fetch_tasks).await;

        // let mut games = Vec::new();
        // for result in results {
        //     match result {
        //         Ok(Some(game)) => {
        //             games.push(game);
        //         }
        //         Ok(None) => {}
        //         Err(e) => {
        //             error!("task panicked: {}", e);
        //         }
        //     }
        // }

        info!("cached {} games from ppvs.su", games.len());
        Ok(games)
    }

    async fn get_games_with_refresh(&self) -> AppResult<Vec<Game>> {
        info!("retrieving games with refresh logic");

        let cache_time = self.repository.get_last_fetch_time("ppvsu").await?;
        let current_time = self.get_current_timestamp().await?;

        match cache_time {
            Some(last_fetch) if !self.is_cache_stale(last_fetch, current_time).await => {
                let cache_age = current_time - last_fetch;
                info!(
                    "overall cache is fresh (last fetch {} seconds ago)",
                    cache_age
                );
                self.repository.get_games("ppvsu").await.map_err(|e| {
                    error!("failed to get games from cache: {}", e);
                    crate::server::error::Error::InternalServerErrorWithContext(format!(
                        "failed to get games from cache: {}",
                        e
                    ))
                })
            }
            _ => {
                if let Some(last_fetch) = cache_time {
                    let cache_age = current_time - last_fetch;
                    info!(
                        "overall cache is stale (last fetch {} seconds ago), refetching all games",
                        cache_age
                    );
                } else {
                    info!("no cache found, fetching all games");
                }

                self.repository.clear_cache("ppvsu").await?;
                let games = self.fetch_and_cache_games().await?;
                self.repository
                    .set_last_fetch_time("ppvsu", current_time)
                    .await?;
                Ok(games)
            }
        }

        // let one_hour = 3600;
        // let games = self.repository.get_games("ppvsu").await?;

        // let mut refresh_tasks = Vec::new();
        // let mut fresh_games = Vec::new();

        // for game in games {
        //     let cache_age = current_time - game.cache_time;

        //     if cache_age > one_hour {
        //         info!(
        //             "game {} is stale (cached {} seconds ago), queueing for refetch",
        //             game.id, cache_age
        //         );

        //         let service_clone = self.clone();
        //         let game_id = game.id;
        //         let old_game = game.clone();

        //         let task = tokio::spawn(async move {
        //             match service_clone.refetch_game(game_id).await {
        //                 Ok(new_game) => (Some(new_game), None, None),
        //                 Err(e) => {
        //                     error!("failed to refetch game {}: {}", game_id, e);

        //                     if e.to_string().contains("404") || e.to_string().contains("not found")
        //                     {
        //                         info!("game {} no longer exists, marking for deletion", game_id);
        //                         (None, Some(game_id), None)
        //                     } else {
        //                         info!("keeping old version of game {}", game_id);
        //                         (None, None, Some(old_game))
        //                     }
        //                 }
        //             }
        //         });

        //         refresh_tasks.push(task);
        //     } else {
        //         fresh_games.push(game);
        //     }
        // }

        // info!(
        //     "refetching {} stale games concurrently",
        //     refresh_tasks.len()
        // );

        // let results = futures::future::join_all(refresh_tasks).await;

        // let mut refreshed_games = fresh_games;
        // for result in results {
        //     match result {
        //         Ok((Some(new_game), _, _)) => {
        //             refreshed_games.push(new_game);
        //         }
        //         Ok((None, Some(game_id_to_delete), None)) => {
        //             if let Err(del_err) = self
        //                 .repository
        //                 .delete_game("ppvsu", game_id_to_delete)
        //                 .await
        //             {
        //                 error!("failed to delete game {}: {}", game_id_to_delete, del_err);
        //             }
        //         }
        //         Ok((None, None, Some(old_game))) => {
        //             refreshed_games.push(old_game);
        //         }
        //         Ok(_) => {}
        //         Err(e) => {
        //             error!("refresh task panicked: {}", e);
        //         }
        //     }
        // }

        // Ok(refreshed_games)
    }

    async fn get_game_by_id(&self, game_id: i64) -> AppResult<Game> {
        info!("fetching game {} from cache or API", game_id);

        if let Some(cached_game) = self.repository.get_game("ppvsu", game_id).await? {
            let current_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|_| anyhow::anyhow!("System time before UNIX epoch"))?
                .as_secs() as i64;

            let cache_age = current_time - cached_game.cache_time;
            let one_hour = 3600;

            if cache_age <= one_hour {
                info!(
                    "returning cached game {} (age: {} seconds)",
                    game_id, cache_age
                );
                return Ok(cached_game);
            }

            info!(
                "cached game {} is stale (age: {} seconds), refetching",
                game_id, cache_age
            );
        } else {
            info!("game {} not in cache, fetching from API", game_id);
        }

        let game = self.refetch_game(game_id).await.map_err(|e| {
            crate::server::error::Error::NotFound(format!("game {} not found: {}", game_id, e))
        })?;

        Ok(game)
    }

    async fn clear_cache(&self) -> AppResult<()> {
        info!("clearing ppvsu cache");

        self.repository.clear_cache("ppvsu").await.map_err(|e| {
            error!("failed to clear ppvsu cache: {}", e);
            crate::server::error::Error::InternalServerErrorWithContext(format!(
                "failed to clear cache: {}",
                e
            ))
        })?;

        info!("ppvsu cache cleared successfully");
        Ok(())
    }
    async fn get_current_timestamp(&self) -> AppResult<i64> {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .map_err(|_| Error::AnyhowError(anyhow::anyhow!("System time before UNIX epoch")))
    }

    async fn is_cache_stale(&self, cache_time: i64, current_time: i64) -> bool {
        const ONE_HOUR: i64 = 3600;
        current_time - cache_time > ONE_HOUR
    }
}
