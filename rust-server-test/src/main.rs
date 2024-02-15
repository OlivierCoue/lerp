use std::fmt::format;

use aes_gcm_siv::{
    aead::{Aead, KeyInit, OsRng},
    Aes256GcmSiv, Nonce,
};
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

    let key_1 = Aes256GcmSiv::generate_key(&mut OsRng);

    println!("{:?}", key_1);

    let string_key = hex::encode(key_1);
    println!("{}", string_key);
    let from_string_key = hex::decode(string_key).unwrap();
    println!("{:?}", from_string_key);

    let cipher = Aes256GcmSiv::new_from_slice(&from_string_key.to_vec()[..]).unwrap();
    let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message
    let ciphertext = cipher
        .encrypt(nonce, b"plaintext message".as_ref())
        .unwrap();
    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref()).unwrap();
    assert_eq!(&plaintext, b"plaintext message");

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
