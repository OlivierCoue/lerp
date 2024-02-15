use sqlx::{postgres::PgPoolOptions, Postgres};

use crate::env::ENV_POSTGRES_DATABASE_URL;

pub async fn pg_pool_init() -> sqlx::Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(std::env::var(ENV_POSTGRES_DATABASE_URL).unwrap().as_str())
        .await
        .unwrap()
}
