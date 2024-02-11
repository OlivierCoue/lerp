use crate::proto::{HttpError, HttpLoginInput, HttpLoginResponse, HttpRegisterInput};

pub enum ApiAuthRequest {
    Login(HttpLoginInput),
    Register(HttpRegisterInput),
}

pub enum ApiAuthResponse {
    Login(Result<HttpLoginResponse, HttpError>),
    Register(Result<bool, HttpError>),
}
