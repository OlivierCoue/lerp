pub const ENV_SERVER_AUTH_URL: &str = "SERVER_AUTH_URL";

pub fn init_env() {
    dotenvy::dotenv().unwrap();
}

pub fn env_server_auth_url() -> String {
    std::env::var(ENV_SERVER_AUTH_URL).expect("env var SERVER_AUTH_URL is not set")
}
