use axum::{
    extract::{Path, State},
    routing::get,
    Router,
};
use tracing::info;

use crate::{
    error::Result,
    model::Model,
    templates::{DeckTemplate, DecksTemplate},
};

pub fn routes() -> Router<Model> {
    Router::new()
        .route("/", get(show_decks))
        .route("/:id", get(show_deck))
}

async fn show_decks(State(model): State<Model>) -> Result<DecksTemplate> {
    info!("{:<12} - show_decks", "HANDLER");
    Ok(DecksTemplate {
        decks: model.select_decks().await?,
    })
}

async fn show_deck(State(model): State<Model>, Path(id): Path<i64>) -> Result<DeckTemplate> {
    info!("{:<12} - show_deck", "HANDLER");
    Ok(DeckTemplate {
        deck: model.select_deck(id).await?
    })
}

