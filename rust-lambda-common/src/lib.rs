use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use axum_extra::protobuf::Protobuf;
use rust_common::{api_common::HEADER_AUTH_TOKEN_KEY, proto::HttpError};
use sqlx::PgPool;
use std::str::FromStr;
use uuid::Uuid;

pub const ENV_POSTGRES_DATABASE_URL: &str = "DATABASE_URL";

pub fn internal_error<E>(err: E) -> (StatusCode, Protobuf<HttpError>)
where
    E: std::error::Error,
{
    let http_error = HttpError {
        message: err.to_string(),
    };

    (StatusCode::INTERNAL_SERVER_ERROR, Protobuf(http_error))
}

pub struct ContextUser {
    pub uuid: Uuid,
}

pub struct ExtractUser(pub ContextUser);

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
