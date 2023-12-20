use axum::{
    extract::{Path, State},
    routing::get,
    Router, Form,
};
use templates::{CardTemplate, CardsTemplate};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use serde::Deserialize;

mod model;
mod templates;

use model::{Card, Cards};

#[tokio::main]
async fn main() {
    let initial_cards = {
        let initial = vec![
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
        ];

        Cards::new(initial)
    };

    let app = Router::new()
        .nest_service("/", ServeDir::new("assets"))
        .route("/cards", get(cards).post(create_card))
        .route("/card/:id", get(card))
        .with_state(initial_cards);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn cards(State(cards): State<Cards>) -> CardsTemplate {
    let cards = cards.unwrap().to_vec();
    CardsTemplate { cards }
}

async fn card(State(cards): State<Cards>, Path(id): Path<u32>) -> CardTemplate {
    let cards = cards.unwrap();
    let card = cards.iter().find(|card| card.id == id).unwrap().clone();
    CardTemplate { card }
}

#[derive(Deserialize)]
struct NewCard {
    front: String,
    back: String
}

async fn create_card(State(cards): State<Cards>, Form(card): Form<NewCard>) -> CardTemplate {
    let mut cards = cards.unwrap();
    let card = Card {
        id: cards.len() as u32,
        front: card.front,
        back: card.back
    };

    cards.push(card);
    let card = cards.last().unwrap().clone();
    CardTemplate { card }
}

