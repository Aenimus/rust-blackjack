use crate::deck::Deck;
use crate::game::Game;
use crate::player::Player;

mod card;
mod deck;
mod player;
mod game;

fn main() {
    let mut game = Game::new(false);
    game.run();
}