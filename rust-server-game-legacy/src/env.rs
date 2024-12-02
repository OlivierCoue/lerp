use std::env;

pub const ENV_POSTGRES_DATABASE_URL: &str = "DATABASE_URL";
pub const ENV_UDP_PORT: &str = "UDP_PORT";

pub fn init_env() {
    if dotenvy::dotenv().is_err() {
        println!("init_env: failed to load .env file, if you see this message in production you can ignore it")
    }
}

pub fn env_aes_key() -> String {
    env::var("AES_KEY").unwrap()
}

pub fn env_aes_nonce() -> String {
    env::var("AES_NONCE").unwrap()
}
