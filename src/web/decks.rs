use super::page_layout;
use axum::{
    extract::{Path, State},
    routing::get,
    Form, Router,
};
use maud::{html, Markup};
use tracing::info;

use crate::{
    error::Result,
    model::{Deck, DeckPayload, Model},
    web::cards::card_element,
};

pub fn routes() -> Router<Model> {
    Router::new()
        .route("/", get(decks_page).post(create_deck))
        .route("/:id", get(deck_page).put(edit_deck).delete(delete_deck))
        .route("/:id/edit", get(edit_deck_menu))
        .route("/:id/element", get(show_deck))
}

async fn decks_page(State(model): State<Model>) -> Result<Markup> {
    info!("{:<12} - show_decks", "HANDLER");
    let decks = model.select_decks().await?;

    let element = page_layout(html! {
        h1 class="text-3xl text-center font-bold" { "elephant cards" }

        div class="grid gap-2" {
            h2 class="pb-2 text-xl text-center font-bold" { "create new deck" }
            form hx-post="/decks" hx-target="#decks" hx-swap="beforeend"
            hx-on:after-request="this.reset()" class="grid grid-cols-3 gap-2 max-w-max mx-auto" {
                label for="name" class="text-right" { "deck name" }
                input id="name" name="name" class="col-span-2 p-1 border rounded";
                button class="col-start-2 p-1 bg-black text-white rounded" { "create!" }
            }
        }

        div class="grid gap-2" {
            h2 class="pb-2 text-xl text-center font-bold" { "decks" }
            div id="decks" class="grid grid-cols-3 gap-4" {
                @for deck in decks.iter() {
                    (deck_element(deck))
                }
            }
        }
    });

    Ok(element)
}

async fn create_deck(State(model): State<Model>, Form(deck): Form<DeckPayload>) -> Result<Markup> {
    info!("{:<12} - create_deck", "HANDLER");
    let deck = model.insert_deck(deck).await?;
    Ok(deck_element(&deck))
}

async fn delete_deck(State(model): State<Model>, Path(id): Path<i64>) -> Result<()> {
    info!("{:<12} - delete_deck", "HANDLER");
    model.delete_deck(id).await?;
    Ok(())
}

async fn edit_deck(
    State(model): State<Model>,
    Path(id): Path<i64>,
    Form(deck): Form<DeckPayload>,
) -> Result<Markup> {
    info!("{:<12} - edit_deck", "HANDLER");
    let deck = model.edit_deck(deck, id).await?;
    Ok(deck_element(&deck))
}

async fn edit_deck_menu(State(model): State<Model>, Path(id): Path<i64>) -> Result<Markup> {
    info!("{:<12} - edit_deck_menu", "HANDLER");
    let deck = model.select_deck(id).await?;

    let element = html! {
        form id=(format!("deck-{}", deck.id))
        class="group relative flex flex-col justify-evenly w-72 aspect-[5/3] p-4 shadow shadow-gray-400 rounded-lg" {
            p class="absolute bottom-4 left-4 text-gray-400" { (deck.id) }
            input value=(deck.name) name="name" class="p-1 border rounded text-lg text-center font-bold";

            div hx-target=(format!("#deck-{}", deck.id)) hx-swap="outerHTML"
            class="absolute bottom-4 right-4 flex gap-2 opacity-0 group-hover:opacity-100 transition" {
                span hx-put=(format!("/decks/{}", deck.id)) hx-trigger="click consume"
                class="material-symbols-rounded cursor-pointer hover:text-green-500 transition" { "done" }

                span hx-get=(format!("/decks/{}/element", deck.id)) hx-trigger="click consume"
                class="material-symbols-rounded cursor-pointer hover:text-red-500 transition" { "close" }
            }
        }
    };

    Ok(element)
}

async fn show_deck(State(model): State<Model>, Path(id): Path<i64>) -> Result<Markup> {
    info!("{:<12} - show_deck", "HANDLER");
    let deck = model.select_deck(id).await?;
    Ok(deck_element(&deck))
}

fn deck_element(deck: &Deck) -> Markup {
    html! {
        div id=(format!("deck-{}", deck.id)) hx-get=(format!("/decks/{}", deck.id)) hx-target="body" hx-swap="innerHTML" hx-push-url="true"
        class="group relative flex flex-col justify-evenly w-72 aspect-[5/3] shadow shadow-gray-400 rounded-lg cursor-pointer" {
            p class="absolute bottom-4 left-4 text-gray-400" { (deck.id) }
            p class="text-lg text-center font-bold" { (deck.name) }

            div hx-target=(format!("#deck-{}", deck.id)) hx-swap="outerHTML"
            class="absolute bottom-4 right-4 flex gap-2 opacity-0 group-hover:opacity-100 transition" {
                span hx-get=(format!("/decks/{}/edit", deck.id)) hx-trigger="click consume" hx-push-url="unset"
                class="material-symbols-rounded cursor-pointer hover:text-sky-500 transition" { "edit" }

                span hx-delete=(format!("/decks/{}", deck.id)) hx-trigger="click consume" hx-push-url="unset"
                class="material-symbols-rounded cursor-pointer hover:text-red-500 transition" { "delete" }
            }
        }
    }
}

async fn deck_page(State(model): State<Model>, Path(id): Path<i64>) -> Result<Markup> {
    info!("{:<12} - show_deck", "HANDLER");
    let deck = model.select_deck(id).await?;
    let cards = model.select_cards(id).await?;

    let element = page_layout(html! {
        h1 class="text-3xl text-center font-bold" { (deck.name) }

        div class="grid gap-2" {
            h2 class="pb-2 text-xl text-center font-bold" { "create new card" }
            form hx-post=(format!("/decks/{}/cards", deck.id)) hx-target="#cards" hx-swap="beforeend"
            hx-on:after-request="this.reset()" class="grid grid-cols-3 gap-2 max-w-max mx-auto" {
                label for="front" class="text-right" { "front text" }
                input id="front" name="front" class="col-span-2 p-1 border rounded";
                label for="back" class="text-right" { "back text" }
                input id="back" name="back" class="col-span-2 p-1 border rounded";
                button class="col-start-2 p-1 bg-black text-white rounded" { "create!" }
            }
        }

        div class="grid gap-2" {
            h2 class="pb-2 text-xl text-center font-bold" { "cards" }
            div id="cards" class="grid grid-cols-3 gap-4" {
                @for card in cards.iter() {
                    (card_element(card, deck.id))
                }
            }
        }
    });

    Ok(element)
}
