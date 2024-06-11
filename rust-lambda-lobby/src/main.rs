use axum::{debug_handler, extract::State, http::StatusCode, routing::post, Router};
use axum_extra::protobuf::Protobuf;
use lambda_http::{run, Error};
use rust_common::{
    api_lobby::ServerLobbyRoute,
    proto::{HttpError, HttpGetGameServerResponse},
};
use rust_lambda_common::{internal_error, ExtractUser, ENV_POSTGRES_DATABASE_URL};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

#[debug_handler]
async fn get_game_server(
    State(pg_pool): State<PgPool>,
    ExtractUser(_): ExtractUser,
) -> Result<Protobuf<HttpGetGameServerResponse>, (StatusCode, Protobuf<HttpError>)> {
    struct PgResult {
        udp_port: i32,
        aes_key: String,
        aes_nonce: String,
    }

    let game_server = match sqlx::query_as!(
        PgResult,
        "SELECT udp_port, aes_key, aes_nonce from game_servers ORDER BY RANDOM() LIMIT 1;"
    )
    .fetch_optional(&pg_pool)
    .await
    .map_err(internal_error)
    {
        Ok(game_server) => game_server,
        Err(err) => return Err(err),
    };

    let Some(game_server) = game_server else {
        return Err((
            StatusCode::BAD_REQUEST,
            Protobuf(HttpError {
                message: "No game server available.".into(),
            }),
        ));
    };

    let response = HttpGetGameServerResponse {
        udp_port: game_server.udp_port,
        aes_key: game_server.aes_key,
        aes_nonce: game_server.aes_nonce,
    };

    Ok(Protobuf(response))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let pg_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(
            std::env::var(ENV_POSTGRES_DATABASE_URL)
                .unwrap_or_else(|_| panic!("env var {ENV_POSTGRES_DATABASE_URL} is not set"))
                .as_str(),
        )
        .await
        .unwrap();

    let app = Router::new()
        .route(
            &ServerLobbyRoute::GetGameServer.as_string(),
            post(get_game_server),
        )
        .with_state(pg_pool);

    run(app).await
}
