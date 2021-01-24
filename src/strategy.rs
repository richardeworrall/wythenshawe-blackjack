use crate::cards::*;
use crate::game::*;

use std::collections::HashSet;

pub trait Strategy {
    fn choose_next(&self, hand: &HashSet<Card>, log: &[Action]) -> Vec<Card>;
    fn name(&self) -> &str;
}

pub struct MinimiseScoreStrategy {}
impl Strategy for MinimiseScoreStrategy
{
    fn choose_next(&self, hand: &HashSet<Card>, log: &[Action]) -> Vec<Card>
    {
        panic!("Not implemented.")
    }
    
    fn name(&self) -> &str
    {
        "MinimiseScoreStrategy"
    }
}

fn find_best_valid<A,B>(preceding: Card, hand: &[Card], can_follow: A, score: B) 
where 
    A : Fn(Card,Card) -> bool,
    B : Fn(&[Card]) -> i32
{
    let mut best_score = 0;
    let mut best = Vec::<Card>::new();

    let mut scratch : Vec<Card> = Vec::new();
    for i in hand { scratch.push(*i); }

    let num_cards = scratch.len();

    let mut save_if_better = |chain| {
        let this_score = score(chain);
        if this_score < best_score {
            best.clear();
            best_score = this_score;
            best.extend_from_slice(chain)
        }
    };

    for f in 0..scratch.len() {
        if !can_follow(preceding, scratch[f]) { continue; }
        if (f != 0) { scratch.swap(0, f); }

        save_if_better(&scratch[0..1]);

        let mut chain_length = 1;

        loop {
            
            for i in chain_length..num_cards {
                if !can_follow(scratch[chain_length-1], scratch[i]) { continue; }
            }
        }
    }

    
}