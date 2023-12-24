use crate::error::{Error, Result};
use serde::Deserialize;
use std::sync::{Arc, Mutex, MutexGuard};
use tracing::info;

#[derive(Clone)]
pub struct Model(Arc<Mutex<Vec<Deck>>>);

#[derive(Clone)]
pub struct Deck {
    pub id: u32,
    pub name: String,
    pub cards: Vec<Option<Card>>,
}

#[derive(Clone)]
pub struct Card {
    pub id: u32,
    pub front: String,
    pub back: String,
}

impl Model {
    pub fn new(decks: Vec<Deck>) -> Self {
        Self(Arc::new(Mutex::new(decks)))
    }

    fn guard(&self) -> Result<MutexGuard<Vec<Deck>>> {
        Ok(self.0.lock().map_err(|_| Error::MutexLockFail)?)
    }
}

impl Model {
    pub fn select_decks(&self) -> Result<Vec<Deck>> {
        info!("{:<12} - select_decks", "MODEL");
        Ok(self.guard()?.to_vec())
    }

    pub fn select_deck(&self, deck_id: u32) -> Result<Deck> {
        info!("{:<12} - select_deck", "MODEL");
        let decks = self.guard()?;
        let deck = decks
            .iter()
            .find(|deck| deck.id == deck_id)
            .ok_or(Error::DeckNotFound)?;

        Ok(deck.clone())
    }

    pub fn select_cards(&self, deck_id: u32) -> Result<Vec<Card>> {
        info!("{:<12} - select_cards", "MODEL");
        let decks = self.guard()?;
        let deck = decks
            .iter()
            .find(|deck| deck.id == deck_id)
            .ok_or(Error::DeckNotFound)?;

        let cards = deck.cards.iter().filter_map(|card| card.clone()).collect();

        Ok(cards)
    }

    pub fn select_card(&self, deck_id: u32, card_id: u32) -> Result<Card> {
        info!("{:<12} - select_card", "MODEL");
        let decks = self.guard()?;
        let deck = decks
            .iter()
            .find(|deck| deck.id == deck_id)
            .ok_or(Error::DeckNotFound)?;

        let card = deck
            .cards
            .get(card_id as usize)
            .cloned()
            .ok_or(Error::CardNotFound)?
            .ok_or(Error::CardDeleted)?;

        Ok(card)
    }

    pub fn insert_card(&self, card: CardPayload, deck_id: u32) -> Result<Card> {
        info!("{:<12} - insert_card", "MODEL");
        let mut decks = self.guard()?;
        let deck = decks
            .iter_mut()
            .find(|deck| deck.id == deck_id)
            .ok_or(Error::DeckNotFound)?;

        let card = Card {
            id: deck.cards.len() as u32,
            front: card.front,
            back: card.back,
        };

        deck.cards.push(Some(card.clone()));
        Ok(card)
    }

    pub fn edit_card(&self, card: CardPayload, card_id: u32, deck_id: u32) -> Result<Card> {
        info!("{:<12} - edit_card", "MODEL");
        let mut decks = self.guard()?;
        let deck = decks
            .iter_mut()
            .find(|deck| deck.id == deck_id)
            .ok_or(Error::DeckNotFound)?;

        let old_card = deck
            .cards
            .get_mut(card_id as usize)
            .ok_or(Error::CardNotFound)?;

        *old_card = Some(Card {
            front: card.front,
            back: card.back,
            id: card_id,
        });

        Ok(old_card.clone().unwrap())
    }

    pub fn delete_card(&self, card_id: u32, deck_id: u32) -> Result<Card> {
        info!("{:<12} - delete_card", "MODEL");
        let mut decks = self.guard()?;
        let deck = decks
            .iter_mut()
            .find(|deck| deck.id == deck_id)
            .ok_or(Error::DeckNotFound)?;

        let old_card = deck
            .cards
            .get_mut(card_id as usize)
            .ok_or(Error::CardNotFound)?
            .take()
            .ok_or(Error::CardDeleted)?;

        Ok(old_card.clone())
    }
}

#[derive(Deserialize)]
pub struct CardPayload {
    front: String,
    back: String,
}
