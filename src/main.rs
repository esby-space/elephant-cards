use std::env;
use tokio::net::TcpListener;
use tracing::info;

mod error;
mod model;
mod web;

use model::Model;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let url = env::var("DATABASE_URL").expect("missing DATABASE_URL env!");
    let model = Model::new(url).await;

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("listening on {}\n", listener.local_addr().unwrap());
    axum::serve(listener, web::routes(model)).await.unwrap();
}

