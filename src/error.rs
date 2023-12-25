use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::{error, fmt::Display, result};
use tracing::error;

pub(crate) type Result<T> = result::Result<T, Error>;

#[derive(Clone, Debug)]
pub(crate) enum Error {
    // user errors
    DeckNotFound,
    CardNotFound,
    CardDeleted,

    // model errors
    DatabaseFailure,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        error!("{:<12} - {self:?}", "ERROR");
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        response.extensions_mut().insert(self);
        response
    }
}

impl error::Error for Error {}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

