use crate::{
    api_common::{send_request, FAKE_PING, HEADER_AUTH_TOKEN_KEY},
    proto::{
        HttpError, HttpLoginInput, HttpLoginResponse, HttpLogoutResponse, HttpRegisterInput,
        HttpRegisterResponse, HttpUserGetCurrentResponse,
    },
};
use prost::Message;
use tokio::time::sleep;

fn get_server_auth_url() -> String {
    match env!("TARGET_ENV") {
        "local" => "http://127.0.0.1:3000/lambda-url/rust-lambda-auth".to_string(),
        "dev" => "https://gnmtmvv2qe6mk6fx3vjzmxs3e40aknel.lambda-url.eu-west-3.on.aws".to_string(),
        _ => panic!("Invalid TARGET_ENV value"),
    }
}

pub enum ServerAuthRoute {
    Register,
    Login,
    Logout,
    UserGetCurrent,
}
impl ServerAuthRoute {
    pub fn as_string(&self) -> String {
        match self {
            ServerAuthRoute::Register => String::from("/register"),
            ServerAuthRoute::Login => String::from("/login"),
            ServerAuthRoute::Logout => String::from("/logout"),
            ServerAuthRoute::UserGetCurrent => String::from("/user-get-current"),
        }
    }
}

pub enum ApiAuthRequest {
    Login(HttpLoginInput),
    Register(HttpRegisterInput),
}

pub enum ApiAuthResponse {
    Login(Result<HttpLoginResponse, HttpError>),
    Register(Result<HttpRegisterResponse, HttpError>),
}

pub struct AuthApi {}
impl AuthApi {
    pub async fn register(
        client: &reqwest::Client,
        input: HttpRegisterInput,
    ) -> Result<HttpRegisterResponse, HttpError> {
        sleep(FAKE_PING).await;
        let mut body_bytes = Vec::with_capacity(input.encoded_len());
        input.encode(&mut body_bytes).unwrap();

        send_request::<HttpRegisterResponse>(
            client
                .post(get_server_auth_url() + &ServerAuthRoute::Register.as_string())
                .body(body_bytes),
        )
        .await
    }

    pub async fn login(
        client: &reqwest::Client,
        input: HttpLoginInput,
    ) -> Result<HttpLoginResponse, HttpError> {
        sleep(FAKE_PING).await;
        let mut body_bytes = Vec::with_capacity(input.encoded_len());
        input.encode(&mut body_bytes).unwrap();

        send_request::<HttpLoginResponse>(
            client
                .post(get_server_auth_url() + &ServerAuthRoute::Login.as_string())
                .body(body_bytes),
        )
        .await
    }

    pub async fn logout(
        client: &reqwest::Client,
        auth_token: String,
    ) -> Result<HttpLogoutResponse, HttpError> {
        sleep(FAKE_PING).await;
        send_request::<HttpLogoutResponse>(
            client
                .post(get_server_auth_url() + &ServerAuthRoute::Logout.as_string())
                .header(HEADER_AUTH_TOKEN_KEY, auth_token),
        )
        .await
    }

    pub async fn user_get_current(
        client: &reqwest::Client,
        auth_token: String,
    ) -> Result<HttpUserGetCurrentResponse, HttpError> {
        sleep(FAKE_PING).await;
        send_request::<HttpUserGetCurrentResponse>(
            client
                .post(get_server_auth_url() + &ServerAuthRoute::UserGetCurrent.as_string())
                .header(HEADER_AUTH_TOKEN_KEY, auth_token),
        )
        .await
    }
}
