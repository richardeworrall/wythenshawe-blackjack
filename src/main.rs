mod cards;
mod player;
mod game;
mod strategy;
mod blackjack;

use game::*;
use strategy::*;

const PRINT : bool = false;

const NUM_PLAYERS : usize = 4;
const NUM_GAMES : usize = 30_000;

fn main() {

    let mut scores = vec![0; NUM_PLAYERS];

    let now = std::time::Instant::now();

    let player_types = vec![StrategyType::Computer; 4];

    for _ in 0..NUM_GAMES
    {
        let round_scores = Game::new(&player_types).run();

        for i in 0..NUM_PLAYERS {
            scores[i] += round_scores[i];
        }
    }
    
    println!("Games finished in {:?}", now.elapsed());
    
    let avg_scores : Vec<f64> = 
                scores
                .iter()
                .map(|s| (*s as f64) / (NUM_GAMES as f64))
                .collect();

    println!("Average scores {:?}", avg_scores);
}