use bevy::log::Level;
use http_api::start_http_api;
use tracing_subscriber::EnvFilter;

pub(crate) mod game;
mod http_api;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::builder().parse_lossy(format!(
            "{},{}",
            Level::INFO,
            "wgpu=error,bevy_render=info,bevy_ecs=warn"
        )))
        .init();

    start_http_api().await;
}
