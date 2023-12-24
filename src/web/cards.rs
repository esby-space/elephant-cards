use axum::{
    extract::{Path, State},
    routing::{get, post},
    Form, Router,
};
use tracing::info;

use crate::{
    error::Result,
    model::{CardPayload, Model},
    templates::{CardTemplate, EditCardTemplate},
};

pub fn routes() -> Router<Model> {
    Router::new()
        .route("/", post(create_card))
        .route("/:id", get(show_card).put(edit_card).delete(delete_card))
        .route("/:id/edit", get(edit_card_menu))
}

async fn show_card(
    State(model): State<Model>,
    Path((deck_id, card_id)): Path<(u32, u32)>,
) -> Result<CardTemplate> {
    info!("{:<12} - list_card", "HANDLER");
    Ok(CardTemplate {
        card: model.select_card(deck_id, card_id)?,
        deck_id,
    })
}

async fn create_card(
    State(model): State<Model>,
    Path(deck_id): Path<u32>,
    Form(card): Form<CardPayload>,
) -> Result<CardTemplate> {
    info!("{:<12} - create_card", "HANDLER");
    Ok(CardTemplate {
        card: model.insert_card(card, deck_id)?,
        deck_id,
    })
}

async fn edit_card_menu(
    State(model): State<Model>,
    Path((deck_id, card_id)): Path<(u32, u32)>,
) -> Result<EditCardTemplate> {
    info!("{:<12} - edit_card_menu", "HANDLER");
    Ok(EditCardTemplate {
        card: model.select_card(deck_id, card_id)?,
        deck_id,
    })
}

async fn edit_card(
    State(model): State<Model>,
    Path((deck_id, card_id)): Path<(u32, u32)>,
    Form(card): Form<CardPayload>,
) -> Result<CardTemplate> {
    info!("{:<12} - edit_card", "HANDLER");
    Ok(CardTemplate {
        card: model.edit_card(card, card_id, deck_id)?,
        deck_id,
    })
}

async fn delete_card(
    State(model): State<Model>,
    Path((deck_id, card_id)): Path<(u32, u32)>,
) -> Result<()> {
    info!("{:<12} - delete_card", "HANDLER");
    model.delete_card(card_id, deck_id)?;
    Ok(())
}
