use askama::Template;

use crate::model::Card;

fn card_id(id: &u32) -> String {
    format!("card-{}", id)
}

#[derive(Template)]
#[template(path = "cards.html")]
pub struct CardsTemplate {
    pub cards: Vec<Card>
}

#[derive(Template)]
#[template(path = "card.html")]
pub struct CardTemplate {
    pub card: Card
}

#[derive(Template)]
#[template(path = "edit_card.html")]
pub struct EditCardTemplate {
    pub card: Card
}

