mod cards;
mod player;
mod game;
mod strategy;
mod blackjack;

use game::*;

fn main() {

    let mut game = Game::new(2, 0);
    
    println!("{:?}", game);

    game.run();
}

use std::io::{stdin, stdout, Read, Write};

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}