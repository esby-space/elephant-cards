use crate::error::Result;
use serde::Deserialize;
use sqlx::{prelude::FromRow, SqlitePool};
use tracing::info;

#[derive(Clone)]
pub struct Model {
    connection: SqlitePool,
}

#[derive(Clone, Debug, FromRow)]
pub struct Deck {
    pub id: i64,
    pub name: String,
}

#[derive(Clone, Debug, FromRow)]
pub struct Card {
    pub id: i64,
    pub front: String,
    pub back: String,
}

#[derive(Deserialize)]
pub struct CardPayload {
    front: String,
    back: String,
}

#[derive(Deserialize)]
pub struct DeckPayload {
    name: String,
}

impl Model {
    pub async fn new(url: String) -> Model {
        let connection = SqlitePool::connect(&url)
            .await
            .expect("failed starting database connection!");
        Model { connection }
    }
}

impl Model {
    pub async fn select_decks(&self) -> Result<Vec<Deck>> {
        info!("{:<12} - select_decks", "MODEL");
        let decks: Vec<Deck> = sqlx::query_as("SELECT id, name FROM decks")
            .fetch_all(&self.connection)
            .await?;

        Ok(decks)
    }

    pub async fn select_deck(&self, deck_id: i64) -> Result<Deck> {
        info!("{:<12} - select_deck", "MODEL");
        let deck: Deck = sqlx::query_as(
            r#"
                SELECT id, name FROM decks
                WHERE id = ?
            "#,
        )
        .bind(deck_id)
        .fetch_one(&self.connection)
        .await?;

        Ok(Deck {
            id: deck.id,
            name: deck.name,
        })
    }

    pub async fn insert_deck(&self, deck: DeckPayload) -> Result<Deck> {
        info!("{:<12} - insert_deck", "MODEL");
        let id = sqlx::query(
            r#"
                INSERT INTO decks (name)
                VALUES (?);
            "#,
        )
        .bind(deck.name)
        .execute(&self.connection)
        .await?
        .last_insert_rowid();

        let deck = self.select_deck(id).await?;
        Ok(deck)
    }

    pub async fn edit_deck(&self, deck: DeckPayload, deck_id: i64) -> Result<Deck> {
        info!("{:<12} - edit_deck", "MODEL");
        let deck: Deck = sqlx::query_as(
            r#"
                UPDATE decks SET name = ?
                WHERE id = ?;

                SELECT id, name FROM decks
                WHERE id = ?
            "#
        )
        .bind(deck.name)
        .bind(deck_id)
        .bind(deck_id)
        .fetch_one(&self.connection)
        .await?;

        Ok(deck)
    }

    pub async fn delete_deck(&self, deck_id: i64) -> Result<()> {
        info!("{:<12} - delete_deck", "MODEL");
        sqlx::query(
            r#"
                DELETE FROM decks
                WHERE decks.id = ?
            "#,
        )
        .bind(deck_id)
        .execute(&self.connection)
        .await?;

        Ok(())
    }

    pub async fn select_cards(&self, deck_id: i64) -> Result<Vec<Card>> {
        info!("{:<12} - select_cards", "MODEL");
        let cards: Vec<Card> = sqlx::query_as(
            r#"
                SELECT id, front, back FROM cards
                WHERE cards.deckID = ?
                ORDER BY id
            "#,
        )
        .bind(deck_id)
        .fetch_all(&self.connection)
        .await?;

        Ok(cards)
    }

    pub async fn select_card(&self, card_id: i64) -> Result<Card> {
        info!("{:<12} - select_card", "MODEL");
        let card: Card = sqlx::query_as(
            r#"
                SELECT id, front, back FROM cards
                WHERE cards.id = ?
            "#,
        )
        .bind(card_id)
        .fetch_one(&self.connection)
        .await?;

        Ok(card)
    }

    pub async fn insert_card(&self, card: CardPayload, deck_id: i64) -> Result<Card> {
        info!("{:<12} - insert_card", "MODEL");
        let id = sqlx::query(
            r#"
                INSERT INTO cards (front, back, deckID)
                VALUES (?, ?, ?)
            "#,
        )
        .bind(card.front)
        .bind(card.back)
        .bind(deck_id)
        .execute(&self.connection)
        .await?
        .last_insert_rowid();

        let card = self.select_card(id).await?;
        Ok(card)
    }

    pub async fn edit_card(&self, card: CardPayload, card_id: i64) -> Result<Card> {
        info!("{:<12} - edit_card", "MODEL");
        let card: Card = sqlx::query_as(
            r#"
                UPDATE cards SET front = ?, back = ?
                WHERE id = ?;

                SELECT id, front, back FROM cards
                WHERE id = ?;
            "#,
        )
        .bind(card.front)
        .bind(card.back)
        .bind(card_id)
        .bind(card_id)
        .fetch_one(&self.connection)
        .await?;

        Ok(card)
    }

    pub async fn delete_card(&self, card_id: i64) -> Result<()> {
        info!("{:<12} - delete_card", "MODEL");
        sqlx::query(
            r#"
                DELETE FROM cards
                WHERE cards.id = ?
            "#,
        )
        .bind(card_id)
        .execute(&self.connection)
        .await?;

        Ok(())
    }
}
