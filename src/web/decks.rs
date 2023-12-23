use axum::{
    extract::{Path, State},
    routing::get,
    Router,
};

use crate::{
    error::Result,
    model::Model,
    templates::DeckTemplate,
};

pub fn routes() -> Router<Model> {
    Router::new().route("/:id", get(list_deck))
}

async fn list_deck(State(model): State<Model>, Path(id): Path<u32>) -> Result<DeckTemplate> {
    Ok(DeckTemplate {
        deck: model.select_deck(id)?,
    })
}
