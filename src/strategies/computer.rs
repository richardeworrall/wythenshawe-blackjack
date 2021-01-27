use crate::cards::*;
use crate::game::*;
use crate::blackjack::*;
use crate::player::*;

use std::collections::{HashSet,HashMap};

pub struct ComputerStrategy {}

impl Strategy for ComputerStrategy
{
    fn choose_next(&self, hand: &HashSet<Card>, log: &[Turn]) -> Vec<Card>
    {
        find_best_valid(log, hand)
    }
    
    fn choose_suit(&self, hand: &HashSet<Card>, _: &[Turn]) -> Suit
    {
        choose_suit(hand)
    }

    fn name(&self) -> &str { "ComputerStrategy" }
}

fn choose_suit(hand: &HashSet<Card>) -> Suit
{
    let mut counts = HashMap::new();
   
    for card in hand {
        *counts.entry(card.suit).or_insert(0) += 1;
    }

    return *counts.iter()
                .max_by(|a, b| a.1.cmp(&b.1))
                .map(|(k, _v)| k).unwrap()
}

fn outstanding_penalty(log: &[Turn]) -> i32
{
    let mut penalty = 0;

    for turn in log.iter().rev() {
        match &turn.action {
            Action::Played(chain) => {
                for i in (0..chain.len()).rev() {
                    let card = chain[i];
                    if let Some(p) = penalty_value(card) {
                        penalty += p;
                    } else {
                        return penalty;
                    }
                }
            },
            _ => { return penalty; }
        }
    }

    return penalty;
}

const PICK_UP_WEIGHTING : f32 = 10.0;

fn score(outstanding_penalty: i32, chain: &[Card]) -> f32
{
    let mut score = chain_score(chain.iter()) as f32;

    for i in (0..chain.len()).rev() {
        let card = chain[i];
        if let Some(penalty) = penalty_value(card) {
            score += penalty as f32 * PICK_UP_WEIGHTING;
        } else {
            return score;
        }
    }

    if outstanding_penalty > 0 {
        score += outstanding_penalty as f32 * PICK_UP_WEIGHTING;
    }

    return score;
}

fn find_best_valid(log: &[Turn], hand: &HashSet<Card>) -> Vec<Card>
{
    let mut best_score : f32 = 0.0;
    
    let mut best = Vec::<Card>::new();
    let mut n_considered = 0;

    let mut scratch = hand.iter().cloned().collect::<Vec<Card>>();

    let outstanding_penalty = outstanding_penalty(log);

    println!("Outstanding penalty: {}", outstanding_penalty);

    let mut save_if_better = |chain: &[Card]| {
        n_considered += 1;
        let this_score = score(outstanding_penalty, chain);
        if this_score > best_score {
            best_score = this_score;
            best.clear();
            best.extend_from_slice(chain)
        }
    };

    fn iterate_extensions<T>(chain_length: usize, cards: &[Card], save: &mut T) 
    where 
        T : for<'a> FnMut(&'a [Card])
    {
        save(&cards[0..chain_length]);

        let prev = cards[chain_length-1];

        let mut scratch : Vec<Card> = cards.iter().cloned().collect();

        for i in chain_length..cards.len()
        {
            if !can_link(prev, cards[i]) { continue; }
            if i != chain_length { scratch.swap(chain_length, i); }

            iterate_extensions(chain_length+1, &scratch, save);
        }
    }
    
    for f in 0..scratch.len() {
        
        if !can_follow(log, scratch[f]) { continue; }
        if f != 0 { scratch.swap(0, f); }

        iterate_extensions(1, &scratch, &mut save_if_better);
    }

    best
}