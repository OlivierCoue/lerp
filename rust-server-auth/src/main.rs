use aes_gcm_siv::{aead::OsRng, Aes256GcmSiv, KeyInit};
use axum::{
    async_trait, debug_handler,
    extract::{FromRequestParts, State},
    http::{request::Parts, StatusCode},
    routing::post,
    Router,
};
use axum_extra::protobuf::Protobuf;
use lambda_http::{run, Error};
use rust_common::{
    api_auth::{ServerAuthRoute, HEADER_AUTH_TOKEN_KEY},
    proto::{
        HttpError, HttpLoginInput, HttpLoginResponse, HttpLogoutResponse, HttpRegisterInput,
        HttpUserGetCurrentResponse,
    },
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::str::FromStr;
use tracing_subscriber::filter::{EnvFilter, LevelFilter};
use uuid::Uuid;

pub const ENV_POSTGRES_DATABASE_URL: &str = "DATABASE_URL";

fn internal_error<E>(err: E) -> (StatusCode, Protobuf<HttpError>)
where
    E: std::error::Error,
{
    let http_error = HttpError {
        message: err.to_string(),
    };

    (StatusCode::INTERNAL_SERVER_ERROR, Protobuf(http_error))
}

struct ContextUser {
    uuid: Uuid,
}

struct ExtractUser(ContextUser);

#[async_trait]
impl FromRequestParts<PgPool> for ExtractUser {
    type Rejection = (StatusCode, Protobuf<HttpError>);

    async fn from_request_parts(
        parts: &mut Parts,
        pg_pool: &PgPool,
    ) -> Result<Self, Self::Rejection> {
        let Some(auth_token) = parts.headers.get(HEADER_AUTH_TOKEN_KEY) else {
            return Err((
                StatusCode::UNAUTHORIZED,
                Protobuf(HttpError {
                    message: "Missing auth-token.".into(),
                }),
            ));
        };

        let Some(context_user) = sqlx::query_as!(
            ContextUser,
            "SELECT uuid FROM users WHERE auth_token = $1",
            Uuid::from_str(auth_token.to_str().unwrap()).unwrap()
        )
        .fetch_optional(pg_pool)
        .await
        .unwrap() else {
            return Err((
                StatusCode::UNAUTHORIZED,
                Protobuf(HttpError {
                    message: "Invalid auth-token.".into(),
                }),
            ));
        };

        Ok(ExtractUser(context_user))
    }
}

#[debug_handler]
async fn register(
    State(pg_pool): State<PgPool>,
    Protobuf(input): Protobuf<HttpRegisterInput>,
) -> Result<String, (StatusCode, Protobuf<HttpError>)> {
    if input.username.is_empty() || input.password.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Protobuf(HttpError {
                message: "Invalid input.".into(),
            }),
        ));
    }

    let existing_user = sqlx::query!(
        "SELECT uuid FROM users WHERE upper(username) = upper($1)",
        input.username
    )
    .fetch_optional(&pg_pool)
    .await
    .map_err(internal_error)?;

    if existing_user.is_some() {
        return Err((
            StatusCode::BAD_REQUEST,
            Protobuf(HttpError {
                message: "Username already in use.".into(),
            }),
        ));
    }

    let user_uuid = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO users (uuid, username) VALUES ($1, $2);",
        user_uuid,
        input.username,
    )
    .execute(&pg_pool)
    .await
    .map_err(internal_error)?;

    Ok("".into())
}

#[debug_handler]
async fn login(
    State(pg_pool): State<PgPool>,
    Protobuf(input): Protobuf<HttpLoginInput>,
) -> Result<Protobuf<HttpLoginResponse>, (StatusCode, Protobuf<HttpError>)> {
    if input.username.is_empty() || input.password.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Protobuf(HttpError {
                message: "Invalid username or password.".into(),
            }),
        ));
    }

    struct PgResult {
        uuid: Uuid,
        username: String,
    }

    let user = match sqlx::query_as!(
        PgResult,
        "SELECT uuid, username from users WHERE upper(username) = upper($1);",
        input.username
    )
    .fetch_optional(&pg_pool)
    .await
    .map_err(internal_error)
    {
        Ok(user) => user,
        Err(err) => return Err(err),
    };

    let Some(user) = user else {
        return Err((
            StatusCode::BAD_REQUEST,
            Protobuf(HttpError {
                message: "Invalid username or password.".into(),
            }),
        ));
    };

    let auth_token = Uuid::new_v4();
    let game_server_aes_key = hex::encode(Aes256GcmSiv::generate_key(&mut OsRng));
    let game_server_aes_nonce = "aaaaaaaaaaaa".to_string();
    let game_server_handshake_challenge = Uuid::new_v4();

    sqlx::query!(
        "UPDATE users SET 
        auth_token = $1,
        game_server_aes_key = $2,
        game_server_aes_nonce = $3,
        game_server_handshake_challenge = $4
        WHERE uuid = $5; ",
        auth_token,
        game_server_aes_key,
        game_server_aes_nonce,
        game_server_handshake_challenge,
        user.uuid
    )
    .execute(&pg_pool)
    .await
    .map_err(internal_error)?;

    let response = HttpLoginResponse {
        uuid: user.uuid.to_string(),
        username: user.username,
        auth_token: auth_token.to_string(),
        game_server_aes_key,
        game_server_aes_nonce,
        game_server_handshake_challenge: game_server_handshake_challenge.to_string(),
    };

    Ok(Protobuf(response))
}

#[debug_handler]
async fn logout(
    State(pg_pool): State<PgPool>,
    ExtractUser(ctx_user): ExtractUser,
) -> Result<Protobuf<HttpLogoutResponse>, (StatusCode, Protobuf<HttpError>)> {
    sqlx::query!(
        "UPDATE users SET auth_token = NULL WHERE uuid = $1; ",
        ctx_user.uuid
    )
    .execute(&pg_pool)
    .await
    .map_err(internal_error)?;

    Ok(Protobuf(HttpLogoutResponse {}))
}

#[debug_handler]
async fn user_get_current(
    State(pg_pool): State<PgPool>,
    ExtractUser(ctx_user): ExtractUser,
) -> Result<Protobuf<HttpUserGetCurrentResponse>, (StatusCode, Protobuf<HttpError>)> {
    struct PgResult {
        uuid: Uuid,
        username: String,
    }

    let user = match sqlx::query_as!(
        PgResult,
        "SELECT uuid, username from users WHERE uuid = $1;",
        ctx_user.uuid
    )
    .fetch_one(&pg_pool)
    .await
    .map_err(internal_error)
    {
        Ok(user) => user,
        Err(err) => return Err(err),
    };

    let response = HttpUserGetCurrentResponse {
        uuid: user.uuid.to_string(),
        username: user.username,
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
        .route(&ServerAuthRoute::Register.as_string(), post(register))
        .route(&ServerAuthRoute::Login.as_string(), post(login))
        .route(&ServerAuthRoute::Logout.as_string(), post(logout))
        .route(
            &ServerAuthRoute::UserGetCurrent.as_string(),
            post(user_get_current),
        )
        .with_state(pg_pool);

    run(app).await
}
