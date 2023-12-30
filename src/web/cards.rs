use axum::{
    extract::{Path, State},
    routing::{get, post},
    Form, Router,
};
use tracing::info;
use maud::{Markup, html};

use crate::{
    error::Result,
    model::{CardPayload, Model, Card},
};

pub fn routes() -> Router<Model> {
    Router::new()
        .route("/", post(create_card))
        .route("/:id", get(show_card).put(edit_card).delete(delete_card))
        .route("/:id/edit", get(edit_card_menu))
}

pub fn card_element(card: &Card, deck_id: i64) -> Markup {
    html! {
        div id=(format!("card-{}", card.id))
        class="group relative flex flex-col justify-evenly w-72 aspect-[5/3] p-4 shadow shadow-gray-400 rounded-lg" {
            p class="absolute bottom-4 left-4 text-gray-400" { (card.id) }
            p class="text-lg text-center font-bold" { (card.front) }
            p class="text-center" { (card.back) }

            div hx-target=(format!("#card-{}", card.id)) hx-swap="outerHTML"
            class="absolute bottom-4 right-4 flex gap-2 opacity-0 group-hover:opacity-100 transition" {
                span hx-get=(format!("/decks/{}/cards/{}/edit", deck_id, card.id))
                class="material-symbols-rounded cursor-pointer hover:text-sky-500 transition" { "edit" }

                span hx-delete=(format!("/decks/{}/cards/{}", deck_id, card.id))
                class="material-symbols-rounded cursor-pointer hover:text-red-500 transition" { "delete" }
            }
        }
    }
}

async fn show_card(
    State(model): State<Model>,
    Path((deck_id, card_id)): Path<(i64, i64)>,
) -> Result<Markup> {
    info!("{:<12} - show_card", "HANDLER");
    let card = model.select_card(card_id).await?;
    Ok(card_element(&card, deck_id))
}

async fn create_card(
    State(model): State<Model>,
    Path(deck_id): Path<i64>,
    Form(card): Form<CardPayload>,
) -> Result<Markup> {
    info!("{:<12} - create_card", "HANDLER");
    let card = model.insert_card(card, deck_id).await?;
    Ok(card_element(&card, deck_id))
}

async fn edit_card_menu(
    State(model): State<Model>,
    Path((deck_id, card_id)): Path<(i64, i64)>,
) -> Result<Markup> {
    info!("{:<12} - edit_card_menu", "HANDLER");
    let card = model.select_card(card_id).await?;

    let element = html! {
        form id=(format!("card-{}", card.id))
        class="group relative flex flex-col justify-evenly w-72 aspect-[5/3] p-4 shadow shadow-gray-400 rounded-lg" {
            p class="absolute bottom-4 left-4 text-gray-400" { (card.id) }
            input value=(card.front) name="front" class="p-1 border rounded text-lg text-center font-bold";
            input value=(card.back) name="back" class="p-1 border rounded text-center";

            div hx-target=(format!("#card-{}", card.id)) hx-swap="outerHTML"
            class="absolute bottom-4 right-4 flex gap-2 opacity-0 group-hover:opacity-100 transition" {
                span hx-put=(format!("/decks/{}/cards/{}", deck_id, card.id))
                class="material-symbols-rounded cursor-pointer hover:text-green-500 transition" { "done" }

                span hx-get=(format!("/decks/{}/cards/{}", deck_id, card.id))
                class="material-symbols-rounded cursor-pointer hover:text-red-500 transition" { "close" }
            }
        }
    };

    Ok(element)
}

async fn edit_card(
    State(model): State<Model>,
    Path((deck_id, card_id)): Path<(i64, i64)>,
    Form(card): Form<CardPayload>,
) -> Result<Markup> {
    info!("{:<12} - edit_card", "HANDLER");
    let card = model.edit_card(card, card_id).await?;
    Ok(card_element(&card, deck_id))
}

async fn delete_card(
    State(model): State<Model>,
    Path((_, card_id)): Path<(i64, i64)>,
) -> Result<()> {
    info!("{:<12} - delete_card", "HANDLER");
    model.delete_card(card_id).await?;
    Ok(())
}

