use axum::{debug_handler, extract::State, http::StatusCode, routing::post, Router};
use axum_extra::protobuf::Protobuf;
use lambda_http::{run, Error};
use rust_common::proto::{HttpError, HttpLoginInput, HttpLoginResponse, HttpRegisterInput};
use sqlx::{postgres::PgPoolOptions, PgPool};
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

    let response = HttpLoginResponse {
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

    let login_api = Router::new().route("/", post(login));
    let register_api = Router::new().route("/", post(register));
    let app = Router::new()
        .nest("/login", login_api)
        .nest("/register", register_api)
        .with_state(pg_pool);

    run(app).await
}
