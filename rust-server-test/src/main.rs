use env::init_env;
use prost::Message;
use reqwest::StatusCode;
use rust_common::proto::{HttpLoginInput, HttpLoginResponse, HttpRegisterInput};

use crate::env::env_server_auth_url;

mod env;

#[tokio::main]
async fn main() -> Result<(), ()> {
    init_env();

    println!("[Test] Starting");

    let client = reqwest::Client::new();

    // Register
    let body = HttpRegisterInput {
        username: "Olivier12".into(),
        password: "test".into(),
    };
    let mut body_bytes = Vec::with_capacity(body.encoded_len());
    body.encode(&mut body_bytes).unwrap();

    let res = client
        .post(env_server_auth_url() + "/register")
        .body(body_bytes)
        .send()
        .await
        .unwrap();

    if res.status() != StatusCode::OK {
        println!("error");
    }

    // Login
    let body = HttpLoginInput {
        username: "Olivier".into(),
        password: "test".into(),
    };
    let mut body_bytes = Vec::with_capacity(body.encoded_len());
    body.encode(&mut body_bytes).unwrap();

    let res = client
        .post(env_server_auth_url() + "/login")
        .body(body_bytes)
        .send()
        .await
        .unwrap();

    let parsed_response = HttpLoginResponse::decode(res.bytes().await.unwrap()).unwrap();
    println!("{:#?}", parsed_response);

    Ok(())
}
