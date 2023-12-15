use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use core::result;
use odbc_api::Error as OdbcError;
use std::string::FromUtf8Error;
use strum_macros::AsRefStr;

pub type Result<T> = result::Result<T, Error>;

#[derive(Clone, Debug, AsRefStr)]
pub enum Error {
    LoginFail,

    // Auth error
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,

    // odbc-api errors
    OdbcError,

    // conversion errors
    ConversionError,

    // IO Error
    IoError,
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),
            Self::AuthFailNoAuthTokenCookie | Self::AuthFailTokenWrongFormat => {
                (StatusCode::FORBIDDEN, ClientError::NO_AUTH)
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "ERROR >> INTO_RES");

        // create palceholder (or dummy) axum response
        let mut res = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // insert error into response
        res.extensions_mut().insert(self);
        res
    }
}

impl From<OdbcError> for Error {
    fn from(_value: OdbcError) -> Self {
        Self::OdbcError
    }
}

impl From<FromUtf8Error> for Error {
    fn from(_value: FromUtf8Error) -> Self {
        Self::ConversionError
    }
}

impl From<std::io::Error> for Error {
    fn from(_value: std::io::Error) -> Self {
        Self::IoError
    }
}

#[derive(Debug, AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}
