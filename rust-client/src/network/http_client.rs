use std::sync::mpsc::{Receiver, Sender};

use prost::Message;
use rust_common::{
    api_auth::{ApiAuthRequest, ApiAuthResponse},
    proto::{HttpError, HttpLoginResponse},
};

pub fn http_client_start(
    rx_http_sender: Receiver<ApiAuthRequest>,
    tx_http_receiver: Sender<ApiAuthResponse>,
) {
    let server_auth_url = "http://127.0.0.1:3000/lambda-url/rust-server-auth";

    let client = reqwest::blocking::Client::new();

    for msg in &rx_http_sender {
        let (path, body) = match &msg {
            ApiAuthRequest::Login(input) => {
                let mut buf = Vec::with_capacity(input.encoded_len());
                input.encode(&mut buf).unwrap();
                ("/login", buf)
            }
            ApiAuthRequest::Register(input) => {
                let mut buf = Vec::with_capacity(input.encoded_len());
                input.encode(&mut buf).unwrap();
                ("/register", buf)
            }
        };

        let resp = client
            .post(server_auth_url.to_owned() + path)
            .body(body)
            .send()
            .unwrap();

        if resp.status().is_success() {
            match &msg {
                ApiAuthRequest::Login(_) => {
                    let decoded_response =
                        HttpLoginResponse::decode(resp.bytes().unwrap()).unwrap();
                    ApiAuthResponse::Login(Ok(decoded_response))
                }
                ApiAuthRequest::Register(_) => ApiAuthResponse::Register(Ok(true)),
            };
        } else {
            let error = HttpError::decode(resp.bytes().unwrap()).unwrap();
            tx_http_receiver
                .send(ApiAuthResponse::Login(Err(error)))
                .unwrap();
        }
    }
}
