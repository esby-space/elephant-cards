use axum::{
    extract::{Path, State},
    routing::get,
    Form, Router,
};
use serde::Deserialize;
use templates::{CardTemplate, CardsTemplate, DeckTemplate, DecksTemplate, EditCardTemplate};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

mod model;
mod templates;

use model::{Card, Deck, Decks};

#[tokio::main]
async fn main() {
    let initial_decks = {
        let cards = Vec::from([
            Card {
                id: 0,
                front: String::from("then the bird got together"),
                back: String::from("and made a beeline to the south"),
            },
            Card {
                id: 1,
                front: String::from("ask the birds and the trees"),
                back: String::from("la de da, de da de dum, ti's autumn"),
            },
            Card {
                id: 2,
                front: String::from("it wouldn't be make believe"),
                back: String::from("if you believed in me"),
            },
        ]);

        let deck = Deck {
            id: 0,
            name: String::from("first deck"),
            cards,
        };

        Decks::new(Vec::from([deck]))
    };

    let app = Router::new()
        .route("/decks", get(decks))
        .route("/decks/:deck_id", get(deck))
        .route("/decks/:deck_id/cards", get(cards).post(create_card))
        .route(
            "/decks/:deck_id/cards/:card_id",
            get(card).delete(delete_card).put(edit_card),
        )
        .route("/decks/:deck_id/cards/:card_id/edit", get(edit_card_menu))
        .fallback_service(ServeDir::new("assets"))
        .with_state(initial_decks);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn decks(State(decks): State<Decks>) -> DecksTemplate {
    let decks = decks.unwrap().to_vec();
    DecksTemplate { decks }
}

async fn deck(State(decks): State<Decks>, Path(deck_id): Path<u32>) -> DeckTemplate {
    let decks = decks.unwrap();
    let deck = decks
        .iter()
        .find(|deck| deck.id == deck_id)
        .unwrap()
        .clone();
    DeckTemplate { deck }
}

async fn cards(State(decks): State<Decks>, Path(deck_id): Path<u32>) -> CardsTemplate {
    println!("hi");
    let decks = decks.unwrap();
    let deck = decks.iter().find(|deck| deck.id == deck_id).unwrap();
    let cards = deck.cards.clone();
    CardsTemplate { cards, deck_id }
}

#[derive(Deserialize)]
struct NewCard {
    front: String,
    back: String,
}

async fn create_card(
    State(decks): State<Decks>,
    Path(deck_id): Path<u32>,
    Form(card): Form<NewCard>,
) -> CardTemplate {
    let mut decks = decks.unwrap();
    let deck = decks.iter_mut().find(|deck| deck.id == deck_id).unwrap();
    let cards = &mut deck.cards;

    let card = Card {
        id: cards.len() as u32,
        front: card.front,
        back: card.back,
    };

    cards.push(card);
    let card = cards.last().unwrap().clone();
    CardTemplate { card, deck_id }
}

async fn card(
    State(decks): State<Decks>,
    Path((deck_id, card_id)): Path<(u32, u32)>,
) -> CardTemplate {
    let decks = decks.unwrap();
    let deck = decks.iter().find(|deck| deck.id == deck_id).unwrap();
    let card = deck
        .cards
        .iter()
        .find(|card| card.id == card_id)
        .unwrap()
        .clone();
    CardTemplate { card, deck_id }
}

async fn edit_card_menu(
    State(decks): State<Decks>,
    Path((deck_id, card_id)): Path<(u32, u32)>,
) -> EditCardTemplate {
    let decks = decks.unwrap();
    let deck = decks.iter().find(|deck| deck.id == deck_id).unwrap();
    let card = deck
        .cards
        .iter()
        .find(|card| card.id == card_id)
        .unwrap()
        .clone();
    EditCardTemplate { card, deck_id }
}

async fn edit_card(
    State(decks): State<Decks>,
    Path((deck_id, card_id)): Path<(u32, u32)>,
    Form(card): Form<NewCard>,
) -> CardTemplate {
    let mut decks = decks.unwrap();
    let deck = decks.iter_mut().find(|deck| deck.id == deck_id).unwrap();
    let cards = &mut deck.cards;

    let position = cards.iter().position(|card| card.id == card_id).unwrap();
    let old_card = cards.get_mut(position).unwrap();
    old_card.front = card.front;
    old_card.back = card.back;

    let card = cards.get(position).unwrap().clone();
    CardTemplate { card, deck_id }
}

async fn delete_card(State(decks): State<Decks>, Path((deck_id, card_id)): Path<(u32, u32)>) {
    let mut decks = decks.unwrap();
    let deck = decks.iter_mut().find(|deck| deck.id == deck_id).unwrap();
    let cards = &mut deck.cards;

    let position = cards.iter().position(|card| card.id == card_id).unwrap();
    cards.remove(position);
}
