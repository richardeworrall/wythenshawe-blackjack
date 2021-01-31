use crate::blackjack::*;
use crate::cards::*;
use crate::game::*;
use crate::strategy::*;

use std::collections::{HashSet,HashMap};
use std::iter::FromIterator;

pub struct ComputerStrategyV2 
{
    live_cards: HashSet<Card>,
    dead_cards: HashSet<Card>,
    log_processed: usize,
    deck_count: usize
}

impl ComputerStrategyV2 
{
    pub fn new(players: &[StrategyType]) -> ComputerStrategyV2
    {
        let deck_count = Card::full_deck().len() - players.len() * STARTING_CARD_COUNT;

        ComputerStrategyV2 
        {
            live_cards: HashSet::from_iter(Card::full_deck()),
            dead_cards: HashSet::new(),
            log_processed: 0,
            deck_count
        }
    }

    pub fn process_new_events(&mut self, log: &[Turn])
    {
        for e in log[self.log_processed..log.len()].iter()
        {
            match &e.action {
                Action::Played(chain) => {
                    for c in chain {
                        self.dead_cards.insert(*c);
                        self.live_cards.remove(c);
                    }
                },
                Action::PickedUp(n) => {
                    if self.deck_count - n < 0
                    {
                        let discard_pile_size = self.dead_cards.len();
                        self.dead_cards.clear();
                        self.live_cards.extend(Card::full_deck());
                        self.deck_count += discard_pile_size - n;
                    }
                },
                Action::First(c) => {
                    self.dead_cards.insert(*c);
                    self.live_cards.remove(c);
                    self.deck_count -= 1;
                },
                | Action::Nominated(_) 
                | Action::Skipped => {
                    continue;
                }
            }
        }

        self.log_processed = log.len();
    }
}

impl Strategy for ComputerStrategyV2
{
    fn choose_next(&mut self, hand: &HashSet<Card>, log: &[Turn]) -> Vec<Card>
    {
        self.process_new_events(log);

        let outstanding_penalty = outstanding_penalty(log);
        
        shared::find_best_valid(log, hand, |chain| {
            score(outstanding_penalty, chain)
        })
    }
    
    fn choose_suit(&mut self, hand: &HashSet<Card>, _: &[Turn]) -> Suit
    {
        choose_suit(hand)
    }

    fn name(&self) -> &str { "Computer (v2)" }
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