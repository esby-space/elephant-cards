use axum::{
    extract::{Path, State},
    routing::{get, post},
    Form, Router,
};

use crate::{
    error::Result,
    model::{CardPayload, Model},
    templates::{CardTemplate, EditCardTemplate},
};

pub fn routes() -> Router<Model> {
    Router::new()
        .route("/", post(create_card))
        .route("/:id", get(list_card).put(edit_card).delete(delete_card))
        .route("/:id/edit", get(edit_card_menu))
}

async fn list_card(
    State(model): State<Model>,
    Path((deck_id, card_id)): Path<(u32, u32)>,
) -> Result<CardTemplate> {
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
    Ok(CardTemplate {
        card: model.insert_card(card, deck_id)?,
        deck_id,
    })
}

async fn edit_card_menu(
    State(model): State<Model>,
    Path((deck_id, card_id)): Path<(u32, u32)>,
) -> Result<EditCardTemplate> {
    Ok(EditCardTemplate {
        card: model.select_card(card_id, deck_id)?,
        deck_id,
    })
}

async fn edit_card(
    State(model): State<Model>,
    Path((deck_id, card_id)): Path<(u32, u32)>,
    Form(card): Form<CardPayload>,
) -> Result<CardTemplate> {
    Ok(CardTemplate {
        card: model.edit_card(card, card_id, deck_id)?,
        deck_id,
    })
}

async fn delete_card(
    State(model): State<Model>,
    Path((deck_id, card_id)): Path<(u32, u32)>,
) -> Result<()> {
    model.delete_card(card_id, deck_id)?;
    Ok(())
}
