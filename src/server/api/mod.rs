pub mod movie_controller;
pub mod proxy_controller;
pub mod stream_controller;
pub mod user_controller;

pub async fn health() -> &'static str {
    "server is ok i think"
}
