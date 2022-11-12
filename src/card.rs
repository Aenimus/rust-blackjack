use std::string::ToString;
use strum_macros::Display;
use strum_macros::EnumIter;

#[derive(Clone, Copy, Display, EnumIter)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Copy, Clone, Display, EnumIter)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Self { suit, rank }
    }

    pub fn value(&self) -> u8 {
        match &self.rank {
            Rank::Ace => 11,
            Rank::King | Rank::Queen| Rank::Jack | Rank::Ten => 10,
            Rank::Nine => 9,
            Rank::Eight => 8,
            Rank::Seven => 7,
            Rank::Six => 6,
            Rank::Five => 5,
            Rank::Four => 4,
            Rank::Three => 3,
            _ => 2
        }
    }

    pub fn to_string(&self) -> String {
        String::from(format!("{} of {}", &self.rank.to_string(), &self.suit.to_string()))
    }
}