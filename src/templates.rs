use askama::Template;
use crate::model::{Card, Deck};

#[derive(Template)]
#[template(path = "deck.html")]
pub struct DeckTemplate {
    pub deck: Deck
}

#[derive(Template)]
#[template(path = "decks.html")]
pub struct DecksTemplate {
    pub decks: Vec<Deck>
}

#[derive(Template)]
#[template(path = "card.html")]
pub struct CardTemplate {
    pub deck_id: u32,
    pub card: Card
}

#[derive(Template)]
#[template(path = "cards.html")]
pub struct CardsTemplate {
    pub deck_id: u32,
    pub cards: Vec<Card>
}

#[derive(Template)]
#[template(path = "edit_card.html")]
pub struct EditCardTemplate {
    pub deck_id: u32,
    pub card: Card
}

