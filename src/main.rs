mod cards;
mod player;
mod game;
mod strategy;
mod blackjack;

use game::*;

fn main() {

    let game = Game::new(4, 0);
    
    println!("{:?}", game);
}