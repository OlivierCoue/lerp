pub const ENV_POSTGRES_DATABASE_URL: &str = "DATABASE_URL";

pub fn init_env() {
    dotenvy::dotenv().unwrap();
}
