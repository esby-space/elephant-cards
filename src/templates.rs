use askama::Template;

use crate::model::Card;

#[derive(Template)]
#[template(path = "card.html")]
pub struct CardTemplate {
    pub card: Card
}

#[derive(Template)]
#[template(path = "cards.html")]
pub struct CardsTemplate {
    pub cards: Vec<Card>
}

