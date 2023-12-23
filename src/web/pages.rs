use crate::{error::Result, model::Model, templates::HomeTemplate};
use axum::{extract::State, routing::get, Router};

pub fn routes() -> Router<Model> {
    Router::new().route("/", get(home_page))
}

async fn home_page(State(model): State<Model>) -> Result<HomeTemplate> {
    Ok(HomeTemplate {
        decks: model.select_decks()?,
    })
}
