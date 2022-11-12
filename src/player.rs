use crate::card::Card;

pub enum Receiver {
    PlayerFirstHand,
    DealerFaceDown,
    DealerFaceUp,
    PlayerSecondHand,
}

pub struct Player {
    pub has_blackjack: [bool; 2],
    hands: [Vec<Card>; 2],
    hand_values: [u8; 2],
    high_aces: [u8; 2],
    has_split: bool,
}

impl Player {
    pub fn new() -> Self {
        Self { has_blackjack: [false; 2], hands: [vec![], vec![]],
            hand_values: [0, 0], high_aces: [0, 0], has_split: false,  }
    }

    pub fn has_split(&self) -> bool {
        self.has_split
    }

    pub fn receive_card(&mut self, hand_index: usize, card: Card) {
        let value = card.value();
        if value == 11 {
            self.high_aces[hand_index] += 1;
        }
        self.hand_values[hand_index] += value;
        self.hands[hand_index].push(card);
    }

    pub fn read_initial_hand(&self, name: String, hand_index: usize) -> bool {
        println!("{} has the {} and the {} for a value of {}.",
                 name, self.hands[hand_index][0].to_string(),
                 self.hands[hand_index][1].to_string(), self.hand_values[hand_index]);
        if self.hand_values[hand_index] == 21 {
            println!("{} scores a blackjack!", name);
            return true;
        }
        false
    }

    pub fn start_round(&mut self) {
        self.has_split = false;
        self.has_blackjack = [false, false];
        self.hands = [vec![], vec![]];
        self.hand_values = [0, 0];
        self.high_aces = [0, 0];
    }

    pub fn hand_value(&self, hand_index: usize) -> u8 {
        self.hand_values[hand_index]
    }

    pub fn try_to_devalue_ace(&mut self, hand_index: usize) -> bool {
        if self.high_aces[hand_index] > 0 {
            self.high_aces[hand_index] -= 1;
            self.hand_values[hand_index] -= 10;
            return true;
        }
        false
    }

    pub fn can_continue(&mut self, hand_index: usize) -> bool {
        while self.hand_values[hand_index] > 21 {
            if !self.try_to_devalue_ace(hand_index) {
                return false;
            }
        }
        if self.hand_values[hand_index] < 21 {
            return true;
        }
        return false
    }

    pub fn split_hand(&mut self) {
        let card = match self.hands[0].pop() {
            None => panic!("Failed to receive a dealt card."), // @TODO: handle this
            Some(card) => card
        };
        self.hands[1].push(card);
        self.hand_values[0] /= 2;
        self.hand_values[1] = self.hand_values[0];
        self.has_split = true;
    }

    pub fn busted_hands(&self) -> usize {
        let mut busted_hands = 0;
        for hand_value in self.hand_values.iter() {
            if hand_value > &21 {
                busted_hands += 1
            }
        }
        busted_hands
    }
}