use tokio::net::TcpListener;
use tracing::info;

mod error;
mod model;
mod templates;
mod web;

use model::{Card, Deck, Model};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let model = {
        let cards = Vec::from([
            Some(Card {
                id: 0,
                front: String::from("then the bird got together"),
                back: String::from("and made a beeline to the south"),
            }),
            Some(Card {
                id: 1,
                front: String::from("ask the birds and the trees"),
                back: String::from("la de da, de da de dum, ti's autumn"),
            }),
            Some(Card {
                id: 2,
                front: String::from("it wouldn't be make believe"),
                back: String::from("if you believed in me"),
            }),
        ]);

        let deck = Deck {
            id: 0,
            name: String::from("first deck"),
            cards,
        };

        Model::new(Vec::from([deck]))
    };

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, web::routes(model)).await.unwrap();
}
