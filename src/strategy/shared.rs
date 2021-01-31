use crate::cards::*;
use crate::game::*;
use crate::blackjack::*;

use std::collections::HashSet;

const MAX_PERMUTATIONS : i32 = 10_000;

pub fn iterate_valid_chains<T>(log: &[Turn], scratch: &mut [Card], ctr: &mut i32, action: &mut T)
where
    T : for<'a> FnMut(&'a [Card])
{
    fn iterate_extensions<T>(chain_length: usize, cards: &[Card], ctr: &mut i32, action: &mut T) 
    where
        T : for<'a> FnMut(&'a [Card])
    {
        action(&cards[0..chain_length]);
        
        *ctr += 1; if *ctr > MAX_PERMUTATIONS { return; }

        let prev = cards[chain_length-1];

        let mut scratch : Vec<Card> = cards.iter().cloned().collect();

        for i in chain_length..cards.len()
        {
            if !can_link(prev, cards[i]) { continue; }
            if i != chain_length { scratch.swap(chain_length, i); }

            iterate_extensions(chain_length + 1, &scratch, ctr, action);
        }
    }
    
    for f in 0..scratch.len() {
        
        if !can_follow(log, scratch[f]) { continue; }
        if f != 0 { scratch.swap(0, f); }

        iterate_extensions(1, &scratch, ctr, action);
    }
}

pub fn find_best_valid<T>(log: &[Turn], hand: &HashSet<Card>, score_function: T) -> Vec<Card>
where 
    T : for<'a> Fn(&'a [Card]) -> f32
{
    let mut best_score : f32 = 0.0;
    let mut best = Vec::<Card>::new();
    let mut n_considered = 0;

    let mut scratch = hand.iter().cloned().collect::<Vec<Card>>();

    iterate_valid_chains(log, &mut scratch, &mut n_considered, 
        &mut |chain: &[Card]| {
        let this_score = score_function(chain);
        if this_score > best_score {
            best_score = this_score;
            best.clear();
            best.extend_from_slice(chain)
        }
    });

    best
}