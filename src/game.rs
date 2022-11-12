use text_io::read;

use crate::player::Receiver;
use crate::{Deck, Player};

const YES1: &str = "y";
const YES2: &str = "yes";
const HIT1: &str = "h";
const HIT2: &str = "hit";
const SPLIT1: &str = "s";
const SPLIT2: &str = "split";

pub struct Game {
    player: Player,
    dealer: Player,
    deck: Deck,
    record: Vec<u8>,
    h17: bool,
}

impl Game {
    pub fn new(h17: bool) -> Self {
        let player = Player::new();
        let dealer = Player::new();
        let deck = Deck::new();
        Self {
            player,
            dealer,
            deck,
            record: vec![0; 3],
            h17,
        }
    }
    fn start_round(&mut self) {
        self.deck.start_round();
        self.player.start_round();
        self.dealer.start_round();
    }

    fn deal_to(&mut self, receiver: Receiver) {
        let card = self.deck.deal();
        match receiver {
            Receiver::PlayerFirstHand => {
                println!("The dealer deals the player the {}.", &card.to_string());
                self.player.receive_card(0, card);
            }
            Receiver::PlayerSecondHand => {
                println!(
                    "The dealer deals the player's second hand the {}.",
                    &card.to_string()
                );
                self.player.receive_card(1, card);
            }
            Receiver::DealerFaceUp => {
                println!("The dealer deals themselves the {}.", &card.to_string());
                self.dealer.receive_card(0, card);
            }
            Receiver::DealerFaceDown => {
                self.dealer.receive_card(0, card);
                println!("The dealer deals themselves a card face down.");
            }
        }
    }

    fn deal_round(&mut self) {
        self.deal_to(Receiver::PlayerFirstHand);
        let first_value = self.player.hand_value(0);
        self.deal_to(Receiver::DealerFaceDown);
        self.deal_to(Receiver::PlayerFirstHand);
        self.deal_to(Receiver::DealerFaceUp);
        let hand_value = self.player.hand_value(0);
        if !self.handle_blackjack(0, hand_value) && hand_value == first_value * 2 {
            self.handle_split();
        }
    }

    fn handle_blackjack(&mut self, player_index: usize, hand_value: u8) -> bool {
        self.player.has_blackjack[player_index] = hand_value == 21;
        if self.player.has_blackjack[player_index] {
            println!("The player scores a Blackjack!");
            return true;
        }
        false
    }

    fn handle_player_turn(&mut self, hand_index: usize) -> bool {
        let player = if hand_index == 0 {
            Receiver::PlayerFirstHand
        } else {
            Receiver::PlayerSecondHand
        };
        println!(
            "The player's hand has a value of {}.",
            self.player.hand_value(hand_index)
        );
        println!("Please type: [h]it or [s]tand?");
        let choice: String = read!();
        match &choice.to_lowercase()[..] {
            YES1 | YES2 | HIT1 | HIT2 => {
                self.deal_to(player);
                false
            }
            _ => {
                println!("The player stands.");
                true
            }
        }
    }

    fn handle_split(&mut self) {
        println!("The player is dealt a pair. Please type: [s]plit or [n]o?");
        let choice: String = read!();
        match &choice.to_lowercase()[..] {
            SPLIT1 | SPLIT2 => {
                self.player.split_hand();
                self.deal_to(Receiver::PlayerFirstHand);
                self.player.has_blackjack[0] = self.player.hand_value(0) == 21;
                self.deal_to(Receiver::PlayerSecondHand);
                self.player.has_blackjack[1] = self.player.hand_value(1) == 21;
            }
            _ => println!("The player decides not to split."),
        }
    }

    fn handle_win(&mut self) {
        self.record[0] += 1;
        println!("The player wins!");
    }

    fn handle_draw(&mut self) {
        self.record[1] += 1;
        println!("The round is a draw.");
    }

    fn handle_loss(&mut self) {
        self.record[2] += 1;
        println!("The player loses.");
    }

    fn read_record(&self) {
        println!(
            "The player's record is {}W {}D {}L.",
            self.record[0], self.record[1], self.record[2]
        );
    }

    fn handle_dealer_turn(&mut self, hands: usize) {
        let dealer_blackjack = self.dealer.read_initial_hand("The dealer".to_string(), 0);
        let mut resolutions: Vec<usize> = vec![];
        for hand_index in 0..hands {
            let player_blackjack = self.player.has_blackjack[hand_index];
            if !dealer_blackjack && !player_blackjack{
                continue
            }
            println!("Result of player's hand number {}:", hand_index + 1);
            if dealer_blackjack && player_blackjack {
                println!("Both players have scored a blackjack.");
                self.handle_draw();
            } else if dealer_blackjack {
                println!("The player fails to match the dealer's blackjack.");
                self.handle_loss();
            } else {
                println!("The dealer fails to match the player's blackjack.");
                self.handle_win();
            }
            resolutions.push(hand_index);
        }
        if self.h17 && self.dealer.hand_value(0) == 17 {
            self.dealer.try_to_devalue_ace(0);
        }
        while self.dealer.hand_value(0) < 17 && self.dealer.can_continue(0) {
            self.deal_to(Receiver::DealerFaceUp);
        }
        self.handle_result(&resolutions, hands);
    }

    fn handle_result(&mut self, resolutions: &Vec<usize>, hands: usize) {
        let start_range = if resolutions.len() == 0 {
            0
        } else {
            1 - resolutions[0]
        };
        let end_range = if start_range > 0 || resolutions.len() == 0 {
            hands
        } else {
            1
        };
        let dealer_hand_value = self.dealer.hand_value(0);
        for hand_index in start_range..end_range {
            let player_hand_value = self.player.hand_value(hand_index);
            println!("Result of player's hand number {}:", hand_index + 1);
            println!(
                "The dealer's hand has a value of {}, and the player's hand has a value of {}.",
                dealer_hand_value, player_hand_value
            );
            if dealer_hand_value > 21 || dealer_hand_value < player_hand_value {
                self.handle_win();
            } else if dealer_hand_value == player_hand_value {
                self.handle_draw();
            } else {
                self.handle_loss();
            }
        }
    }

    fn restart(&self) -> bool {
        self.read_record();
        println!("Play again? Please type: [y]es or [n]o");
        let choice: String = read!();
        match &choice.to_lowercase()[..] {
            YES1 | YES2 | HIT1 | HIT2 => true,
            _ => false,
        }
    }

    pub fn run(&mut self) {
        loop {
            self.start_round();
            self.deal_round();
            let hands: usize = if self.player.has_split() { 2 } else { 1 };
            let mut hand_index: usize = 0;
            while hand_index < hands {
                if self.player.has_blackjack[hand_index]
                    || self.handle_player_turn(hand_index)
                    || !self.player.can_continue(hand_index)
                {
                    hand_index += 1;
                }
            }
            let busted_hands = self.player.busted_hands();
            match busted_hands {
                2 => {
                    println!("The player busts both hands.");
                    self.record[2] += 2
                }
                _ => {
                    if busted_hands == hands {
                        self.handle_loss();
                    } else {
                        if busted_hands > 0 {
                            println!("The player busts one of their hands.");
                            self.record[2] += 1
                        }
                        self.handle_dealer_turn(hands);
                    }
                }
            }
            if !self.restart() {
                println!("Thanks for playing.");
                break;
            }
        }
    }
}
