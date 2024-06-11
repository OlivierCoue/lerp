use prost::Message;
use std::time::Duration;

use reqwest::RequestBuilder;

use crate::proto::HttpError;

pub const HEADER_AUTH_TOKEN_KEY: &str = "auth-token";
pub const FAKE_PING: Duration = Duration::from_millis(0);

pub async fn send_request<T>(request_builder: RequestBuilder) -> Result<T, HttpError>
where
    T: prost::Message + Default,
{
    let response = match request_builder.send().await {
        Ok(response) => response,
        Err(err) => {
            return Err(HttpError {
                message: err.to_string(),
            })
        }
    };

    if response.status().is_success() {
        return Ok(T::decode(response.bytes().await.unwrap()).unwrap());
    }

    Err(HttpError::decode(response.bytes().await.unwrap()).unwrap())
}
