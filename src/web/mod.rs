use axum::{Router, middleware, response::Response};
use tower_http::services::ServeDir;

use crate::model::Model;

mod decks;
mod cards;

pub fn routes(model: Model) -> Router {
    Router::new()
        .nest("/decks", decks::routes())
        .nest("/decks/:deck_id/cards", cards::routes())
        .layer(middleware::map_response(response_mapper))
        .fallback_service(ServeDir::new("assets"))
        .with_state(model)
}

async fn response_mapper(response: Response) -> Response {
    println!();
    response
}

