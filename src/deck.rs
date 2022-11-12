use rand::prelude::ThreadRng;
use rand::thread_rng;
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;

use crate::card::*;

fn new_full_suit(suit: &Suit) -> Vec<Card> {
    let mut full_suit: Vec<Card> = Vec::with_capacity(13);
    for rank in Rank::iter() {
        full_suit.push(Card::new(*suit, rank));
    }
    full_suit
}

fn new_deck() -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::with_capacity(52);
    for suit in Suit::iter() {
        cards.append(&mut new_full_suit(&suit));
    }
    cards
}

pub struct Deck {
    pub cards: Vec<Card>,
    rng: ThreadRng,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = new_deck();
        let mut rng = thread_rng();
        cards.shuffle(&mut rng);
        Self { cards, rng }
    }

    pub fn deal(&mut self) -> Card {
        match self.cards.pop() {
            None => panic!("Failed to receive a dealt card."), // @TODO: handle this
            Some(card) => card
        }
    }

    pub fn start_round(&mut self) {
        if self.cards.len() < 27 {
            println!("Play continues with a new, shuffled deck.");
            self.cards = new_deck();
        }
        self.cards.shuffle(&mut self.rng);
    }
}