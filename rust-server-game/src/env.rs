pub const ENV_POSTGRES_DATABASE_URL: &str = "DATABASE_URL";
pub const ENV_UDP_ADDRESS: &str = "UDP_ADDRESS";
pub const ENV_UDP_PORT: &str = "UDP_PORT";

pub fn init_env() {
    if dotenvy::dotenv().is_err() {
        println!("init_env: failed to load .env file, if you see this message in production you can ignore it")
    }
}
