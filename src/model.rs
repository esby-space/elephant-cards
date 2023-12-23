use crate::error::{Error, Result};
use std::sync::{Arc, Mutex, MutexGuard};

use serde::Deserialize;

#[derive(Clone)]
pub struct Model(Arc<Mutex<Vec<Deck>>>);

#[derive(Clone)]
pub struct Deck {
    pub id: u32,
    pub name: String,
    pub cards: Vec<Card>,
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
        Ok(self.guard()?.to_vec())
    }

    pub fn select_deck(&self, deck_id: u32) -> Result<Deck> {
        let decks = self.guard()?;
        let deck = decks
            .iter()
            .find(|deck| deck.id == deck_id)
            .ok_or(Error::DeckNotFound)?;
        Ok(deck.clone())
    }

    pub fn select_cards(&self, deck_id: u32) -> Result<Vec<Card>> {
        Ok(self.select_deck(deck_id)?.cards)
    }

    pub fn select_card(&self, deck_id: u32, card_id: u32) -> Result<Card> {
        let decks = self.guard()?;
        let deck = decks
            .iter()
            .find(|deck| deck.id == deck_id)
            .ok_or(Error::DeckNotFound)?;
        let card = deck
            .cards
            .iter()
            .find(|card| card.id == card_id)
            .ok_or(Error::CardNotFound)?;
        Ok(card.clone())
    }

    pub fn insert_card(&self, card: CardPayload, deck_id: u32) -> Result<Card> {
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

        deck.cards.push(card.clone());
        Ok(card)
    }

    pub fn edit_card(&self, card: CardPayload, card_id: u32, deck_id: u32) -> Result<Card> {
        let mut decks = self.guard()?;
        let deck = decks
            .iter_mut()
            .find(|deck| deck.id == deck_id)
            .ok_or(Error::DeckNotFound)?;
        let position = deck
            .cards
            .iter()
            .position(|card| card.id == card_id)
            .ok_or(Error::CardNotFound)?;

        let old_card = deck.cards.get_mut(position).ok_or(Error::CardNotFound)?;
        old_card.front = card.front;
        old_card.back = card.back;

        Ok(old_card.clone())
    }

    pub fn delete_card(&self, card_id: u32, deck_id: u32) -> Result<Card> {
        let mut decks = self.guard()?;
        let deck = decks
            .iter_mut()
            .find(|deck| deck.id == deck_id)
            .ok_or(Error::CardNotFound)?;
        let position = deck
            .cards
            .iter()
            .position(|card| card.id == card_id)
            .unwrap();

        Ok(deck.cards.remove(position))
    }
}

#[derive(Deserialize)]
pub struct CardPayload {
    front: String,
    back: String,
}
