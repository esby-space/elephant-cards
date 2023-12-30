use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::result;
use thiserror::Error;
use tracing::error;

pub(crate) type Result<T> = result::Result<T, ElephantError>;

#[derive(Debug, Error)]
pub(crate) enum ElephantError {
    // model errors
    #[error("internal database error occured")]
    SQLx(#[from] sqlx::Error),
}

impl IntoResponse for ElephantError {
    fn into_response(self) -> Response {
        error!("{:<12} - {self:?}", "ERROR");
        (StatusCode::INTERNAL_SERVER_ERROR, format!("{self:?}")).into_response()
    }
}

