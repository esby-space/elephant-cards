use axum::Router;
use tower_http::services::ServeDir;

use crate::model::Model;

mod decks;
mod cards;
mod pages;

pub fn routes(model: Model) -> Router {
    Router::new()
        .merge(pages::routes())
        .nest("/decks", decks::routes())
        .nest("/decks/:deck_id/cards", cards::routes())
        .fallback_service(ServeDir::new("assets"))
        .with_state(model)
}

