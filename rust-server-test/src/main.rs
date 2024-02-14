use env::init_env;
use rust_common::{
    api_auth::AuthApi,
    proto::{HttpLoginInput, HttpRegisterInput},
};
use uuid::Uuid;

mod env;

#[tokio::main]
async fn main() -> Result<(), String> {
    init_env();

    println!("[Test] Starting");

    let client = reqwest::Client::new();

    let username = Uuid::new_v4().to_string();

    // Register
    let input = HttpRegisterInput {
        username: username.clone(),
        password: "test".into(),
    };

    match AuthApi::register(&client, input).await {
        Ok(res) => res,
        Err(err) => {
            println!("{}", err.message);
            panic!("[Auth][register] failed")
        }
    };

    // Login
    let input = HttpLoginInput {
        username: username.clone(),
        password: "test".into(),
    };

    let res = match AuthApi::login(&client, input).await {
        Ok(res) => res,
        Err(err) => {
            println!("{}", err.message);
            panic!("[Auth][login] failed")
        }
    };

    let auth_token = res.auth_token;

    // User get current
    let user = AuthApi::user_get_current(&client, auth_token.clone())
        .await
        .unwrap();

    assert_eq!(user.username, username);

    // Logout
    AuthApi::logout(&client, auth_token.clone()).await.unwrap();

    // User get current (expected to fail)
    if AuthApi::user_get_current(&client, auth_token.clone())
        .await
        .is_ok()
    {
        return Err("Expected user_get_current to fail after logout".to_string());
    }

    println!("[Test] Passed");

    Ok(())
}
