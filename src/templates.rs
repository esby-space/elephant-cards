use askama::Template;
use crate::model::{Card, Deck};

#[derive(Template)]
#[template(path = "index.html")]
pub struct DecksTemplate {
    pub decks: Vec<Deck>
}

#[derive(Template)]
#[template(path = "deck.html")]
pub struct DeckTemplate {
    pub deck: Deck
}

#[derive(Template)]
#[template(path = "card.html")]
pub struct CardTemplate {
    pub deck_id: i64,
    pub card: Card
}

#[derive(Template)]
#[template(path = "edit_card.html")]
pub struct EditCardTemplate {
    pub deck_id: i64,
    pub card: Card
}

