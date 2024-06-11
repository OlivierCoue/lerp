use crate::{
    api_common::{send_request, FAKE_PING, HEADER_AUTH_TOKEN_KEY},
    proto::{HttpError, HttpGetGameServerInput, HttpGetGameServerResponse},
};
use prost::Message;
use tokio::time::sleep;

fn get_server_lobby_url() -> String {
    match env!("TARGET_ENV") {
        "local" => "http://127.0.0.1:3000/lambda-url/rust-lambda-lobby".to_string(),
        "dev" => "https://gnmtmvv2qe6mk6fx3vjzmxs3e40aknel.lambda-url.eu-west-3.on.aws".to_string(),
        _ => panic!("Invalid TARGET_ENV value"),
    }
}

pub enum ServerLobbyRoute {
    GetGameServer,
}
impl ServerLobbyRoute {
    pub fn as_string(&self) -> String {
        match self {
            ServerLobbyRoute::GetGameServer => String::from("/get-game-server"),
        }
    }
}

pub enum ApiLobbyRequest {
    GetGameServer(HttpGetGameServerInput),
}

pub enum ApiLobbyResponse {
    GetGameServer(Result<HttpGetGameServerResponse, HttpError>),
}

pub struct LobbyApi {}
impl LobbyApi {
    pub async fn get_game_server(
        client: &reqwest::Client,
        auth_token: String,
        input: HttpGetGameServerInput,
    ) -> Result<HttpGetGameServerResponse, HttpError> {
        sleep(FAKE_PING).await;
        let mut body_bytes = Vec::with_capacity(input.encoded_len());
        input.encode(&mut body_bytes).unwrap();

        send_request::<HttpGetGameServerResponse>(
            client
                .post(get_server_lobby_url() + &ServerLobbyRoute::GetGameServer.as_string())
                .header(HEADER_AUTH_TOKEN_KEY, auth_token)
                .body(body_bytes),
        )
        .await
    }
}
