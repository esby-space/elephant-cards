
use tokio::net::TcpListener;

mod model;
mod web;
mod templates;
mod error;

use model::{Card, Deck, Model};

#[tokio::main]
async fn main() {
    let model = {
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

        Model::new(Vec::from([deck]))
    };

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, web::routes(model)).await.unwrap();
}

