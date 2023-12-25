use crate::error::{Error, Result};
use serde::Deserialize;
use sqlx::{prelude::FromRow, SqlitePool};
use tracing::info;

#[derive(Clone)]
pub struct Model {
    connection: SqlitePool,
}

#[derive(Clone, Debug)]
pub struct Deck {
    pub id: i64,
    pub name: String,
    pub cards: Vec<Card>,
}

#[derive(FromRow)]
struct ID(i64);

#[derive(FromRow)]
pub struct PartialDeck {
    id: i64,
    name: String,
}

#[derive(Clone, Debug, FromRow)]
pub struct Card {
    pub id: i64,
    pub front: String,
    pub back: String,
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
        let ids: Vec<ID> = sqlx::query_as("SELECT id FROM decks")
            .fetch_all(&self.connection)
            .await
            .map_err(|_| Error::DatabaseFailure)?;

        let mut decks = Vec::new();
        for ID(id) in ids {
            let deck = self.select_deck(id).await?;
            decks.push(deck);
        }

        Ok(decks)
    }

    pub async fn select_deck(&self, deck_id: i64) -> Result<Deck> {
        info!("{:<12} - select_deck", "MODEL");

        let cards = self.select_cards(deck_id).await?;
        let deck: PartialDeck = sqlx::query_as(
            r#"
                SELECT id, name FROM decks
                WHERE id = ?
            "#,
        )
        .bind(deck_id)
        .fetch_one(&self.connection)
        .await
        .map_err(|_| Error::DatabaseFailure)?;

        Ok(Deck {
            id: deck.id,
            name: deck.name,
            cards,
        })
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
        .await
        .map_err(|_| Error::DatabaseFailure)?;

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
        .await
        .map_err(|_| Error::DatabaseFailure)?;

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
        .await
        .map_err(|_| Error::DatabaseFailure)?
        .last_insert_rowid();

        let card = self.select_card(id).await?;
        Ok(card)
    }

    pub async fn edit_card(&self, card: CardPayload, card_id: i64) -> Result<Card> {
        info!("{:<12} - edit_card", "MODEL");
        sqlx::query(
            r#"
                UPDATE cards
                SET front = ?,
                    back = ?
                WHERE id = ?
            "#,
        )
        .bind(card.front)
        .bind(card.back)
        .bind(card_id)
        .execute(&self.connection)
        .await
        .map_err(|_| Error::DatabaseFailure)?;

        let card = self.select_card(card_id).await?;
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
        .await
        .map_err(|_| Error::DatabaseFailure)?;

        Ok(())
    }
}

#[derive(Deserialize)]
pub struct CardPayload {
    front: String,
    back: String,
}

