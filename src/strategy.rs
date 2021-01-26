use crate::cards::*;
use crate::game::*;
use crate::blackjack::*;

use std::collections::{HashSet,HashMap};

pub trait Strategy {
    fn choose_next(&self, hand: &HashSet<Card>, log: &[Turn]) -> Vec<Card>;
    fn choose_suit(&self, hand: &HashSet<Card>, log: &[Turn]) -> Suit;
    fn name(&self) -> &str;
}

pub struct MinimiseScoreStrategy {}

impl MinimiseScoreStrategy
{
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

    fn find_best_valid(log: &[Turn], hand: &HashSet<Card>) -> Vec<Card>
    {
        let mut best_score = 0;
        let mut best = Vec::<Card>::new();
        let mut n_considered = 0;

        let mut scratch = hand.iter().cloned().collect::<Vec<Card>>();

        let mut save_if_better = |chain: &[Card]| {
            n_considered += 1;
            let this_score = chain_score(chain.iter());
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
            if can_end_with(cards[chain_length-1]) { save(&cards[0..chain_length]); }

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
}

impl Strategy for MinimiseScoreStrategy
{
    fn choose_next(&self, hand: &HashSet<Card>, log: &[Turn]) -> Vec<Card>
    {
        MinimiseScoreStrategy::find_best_valid(log, hand)
    }
    
    fn choose_suit(&self, hand: &HashSet<Card>, _: &[Turn]) -> Suit
    {
        MinimiseScoreStrategy::choose_suit(hand)
    }

    fn name(&self) -> &str { "MinimiseScoreStrategy" }
}