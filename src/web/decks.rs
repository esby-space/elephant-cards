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
    Ok(DecksTemplate {
        decks: model.select_decks()?,
    })
}

async fn show_deck(State(model): State<Model>, Path(id): Path<u32>) -> Result<DeckTemplate> {
    info!("{:<12} - list_deck", "HANDLER");
    let deck = model.select_deck(id)?;
    let cards = model.select_cards(id)?;

    Ok(DeckTemplate {
        id,
        name: deck.name,
        cards,
    })
}
