mod cards;
mod player;
mod game;
mod strategies;
mod blackjack;

use game::*;

fn main() {
    Game::new(4, 0).run();
}