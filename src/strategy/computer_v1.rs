use crate::blackjack::*;
use crate::cards::*;
use crate::game::*;
use crate::strategy::*;

use std::collections::{HashSet,HashMap};

pub struct ComputerStrategyV1 {}

impl Strategy for ComputerStrategyV1
{
    fn choose_next(&mut self, hand: &HashSet<Card>, log: &[Turn]) -> Vec<Card>
    {
        let outstanding_penalty = outstanding_penalty(log);
        
        shared::find_best_valid(log, hand, |chain| {
            score(outstanding_penalty, chain)
        })
    }
    
    fn choose_suit(&mut self, hand: &HashSet<Card>, _: &[Turn]) -> Suit
    {
        choose_suit(hand)
    }

    fn name(&self) -> &str { "Computer (v1)" }
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

fn score(outstanding_penalty: usize, chain: &[Card]) -> f32
{
    let mut score = chain.iter().map(card_score).sum::<i32>() as f32;

    for i in (0..chain.len()).rev() {
        let card = chain[i];
        score += penalty_value(card) as f32 * PICK_UP_WEIGHTING;
    }

    if outstanding_penalty > 0 {
        score += outstanding_penalty as f32 * PICK_UP_WEIGHTING;
    }

    return score;
}