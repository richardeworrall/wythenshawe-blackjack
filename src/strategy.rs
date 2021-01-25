use crate::cards::*;
use crate::game::*;
use crate::blackjack::*;

use std::collections::HashSet;

pub trait Strategy {
    fn choose_next(&self, hand: &HashSet<Card>, log: &[Action]) -> Vec<Card>;
    fn name(&self) -> &str;
}

pub struct MinimiseScoreStrategy {}

impl MinimiseScoreStrategy
{
    fn find_best_valid<A,B,C>(preceding: Card, hand: &[Card], can_follow: A, can_link: &mut B, score: &C) -> Vec<Card>
    where 
        A : Fn(Card,Card) -> bool,
        B : Fn(Card,Card) -> bool,
        C : Fn(&[Card]) -> i32
    {
        let mut best_score = 0;
        let mut best = Vec::<Card>::new();

        let mut scratch = hand.iter().cloned().collect::<Vec<Card>>();

        //println!("scratch: {:?}", scratch);

        let mut save_if_better = |chain: &[Card]| {
            //println!("Saving: {:?}", chain);
            //pause();
            let this_score = score(chain);
            if this_score > best_score {
                best_score = this_score;
                best.clear();
                best.extend_from_slice(chain)
            }
        };

        fn iterate_extensions<T,B>(chain_length: usize, cards: &[Card], save: &mut T, can_link: &mut B) 
        where 
            T : for<'a> FnMut(&'a [Card]),
            B : Fn(Card,Card) -> bool
        {
            //println!("chain: {:?}, float: {:?}", &cards[0..chain_length], &cards[chain_length..cards.len()]);

            save(&cards[0..chain_length]);

            let prev = cards[chain_length-1];

            let mut scratch : Vec<Card> = cards.iter().cloned().collect();

            for i in chain_length..cards.len()
            {
                if !can_link(prev, cards[i]) { continue; }
                if i != chain_length { scratch.swap(chain_length, i); }

                iterate_extensions(chain_length+1, &scratch, save, can_link);
            }
        }
        
        for f in 0..scratch.len() {
            
            if !can_follow(preceding, scratch[f]) { continue; }
            if f != 0 { scratch.swap(0, f); }

            iterate_extensions(1, &scratch, &mut save_if_better, can_link);
        }

        best
    }
}

impl Strategy for MinimiseScoreStrategy
{
    fn choose_next(&self, hand: &HashSet<Card>, log: &[Action]) -> Vec<Card>
    {
        let mut hand_slice = Vec::new();
        hand_slice.extend(hand.iter().cloned());
        let score = |c: &[Card]| chain_score(c.iter());
        MinimiseScoreStrategy::find_best_valid(log.last().unwrap().1, &hand_slice, can_follow, &mut can_link, &score)
    }
    
    fn name(&self) -> &str { "MinimiseScoreStrategy" }
}