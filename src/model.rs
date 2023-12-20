use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Clone)]
pub struct Cards(Arc<Mutex<Vec<Card>>>);

#[derive(Clone)]
pub struct Card {
    pub id: u32,
    pub front: String,
    pub back: String,
}

impl Cards {
    pub fn new(cards: Vec<Card>) -> Self {
        Self(Arc::new(Mutex::new(cards))) 
    }

    pub fn unwrap(&self) -> MutexGuard<Vec<Card>> {
        self.0.lock().unwrap()
    }
}

