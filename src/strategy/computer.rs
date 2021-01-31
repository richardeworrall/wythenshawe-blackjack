use crate::blackjack::*;
use crate::cards::*;
use crate::game::*;
use crate::strategy::*;

use std::collections::{HashSet,HashMap};

pub struct ComputerStrategy {}

impl Strategy for ComputerStrategy
{
    fn choose_next(&self, hand: &HashSet<Card>, log: &[Turn]) -> Vec<Card>
    {
        let outstanding_penalty = outstanding_penalty(log);
        
        shared::find_best_valid(log, hand, |chain| {
            score(outstanding_penalty, chain)
        })
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