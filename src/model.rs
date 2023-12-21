use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Clone)]
pub struct Decks(Arc<Mutex<Vec<Deck>>>);

#[derive(Clone)]
pub struct Deck {
    pub id: u32,
    pub name: String,
    pub cards: Vec<Card>
}

impl Decks {
    pub fn new(decks: Vec<Deck>) -> Self {
        Self(Arc::new(Mutex::new(decks))) 
    }

    pub fn unwrap(&self) -> MutexGuard<Vec<Deck>> {
        self.0.lock().unwrap()
    }
}

#[derive(Clone)]
pub struct Card {
    pub id: u32,
    pub front: String,
    pub back: String,
}

