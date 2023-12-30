use axum::{Router, middleware, response::Response};
use maud::{Markup, html, DOCTYPE};
use tower_http::services::ServeDir;

use crate::model::Model;

mod decks;
mod cards;

pub fn routes(model: Model) -> Router {
    Router::new()
        .nest("/decks", decks::routes())
        .nest("/decks/:deck_id/cards", cards::routes())
        .layer(middleware::map_response(response_mapper))
        .fallback_service(ServeDir::new("assets"))
        .with_state(model)
}

async fn response_mapper(response: Response) -> Response {
    println!();
    response
}

pub fn page_layout(page: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                link rel="stylesheet" href="/lib/material_symbols.css";
                script src="/lib/cdn.twind.style.js" crossorigin {};
                script src="/lib/htmx.min.js" {};
                title { "esby is confused" }
            }

            body hx-boost="true"
            class="min-h-screen grid place-content-center gap-8" { (page) }
        }
    }
}

