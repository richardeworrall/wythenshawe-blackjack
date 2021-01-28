use crate::cards::*;
use crate::game::*;

use std::collections::HashSet;

use streaming_iterator::*;

pub const STARTING_CARD_COUNT : usize = 7;

pub fn penalty_value(card: Card) -> Option<i32>
{
    match card {
        Card { rank: Rank::Jack, suit: s } if s.is_black() => Some(5),
        Card { rank: Rank::Val(2), suit: _ } => Some(2),
        _ => None
    }
}

pub fn outstanding_penalty(log: &[Turn]) -> i32
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
            Action::First(card) => {
                if let Some(p) = penalty_value(*card) {
                    penalty += p;
                }
                return penalty;
            }
            _ => { return penalty; }
        }
    }

    return penalty;
}

pub fn card_score(card: &Card) -> i32
{
    match card {
        Card { rank: Rank::Ace, suit: _ } => 25,
        Card { rank: Rank::Val(8), suit: _ } => 20,
        Card { rank: Rank::Val(2), suit: _ } => 20,
        Card { rank: Rank::Jack, suit: s } if s.is_black() => 15,
        Card { rank: r, suit: _ } => r.face_value() 
    }
}

pub fn chain_score<'a, T>(chain: T) -> i32
where 
    T : Iterator<Item = &'a Card>
{
    chain.map(|c| card_score(c)).sum()
}

pub fn can_follow(log: &[Turn], next: Card) -> bool
{
    fn can_follow_card(active: bool, prev: Card, next: Card) -> bool
    {
        fn can_follow_nominal(prev: Card, next: Card) -> bool
        {
            next.rank == Rank::Ace 
            || prev.suit == next.suit 
            || prev.rank == next.rank
        }
        
        if active {
            match prev {
                Card { rank: Rank::Jack, suit: s } if s.is_black() => {
                    match next { 
                        Card { rank: Rank::Jack, suit: _ } => true,
                        Card { rank: Rank::Val(2), suit: s2 } if s == s2  => true,
                        _ => false
                    }
                },
                Card { rank: Rank::Val(2), suit: _ } => { next.rank == Rank::Val(2) },
                Card { rank: Rank::Val(8), suit: _ } => { next.rank == Rank::Val(8) },
                _ => {
                    can_follow_nominal(prev, next)
                }
            }
        } else {
            can_follow_nominal(prev, next)
        }
    }

    let mut idx = log.len()-1;
    let mut is_active = true;

    loop {
        match &log[idx].action {
            Action::First(prev) => { 
                return can_follow_card(is_active, *prev, next); 
            },
            Action::Played(chain) => { 
                return can_follow_card(is_active, *chain.last().unwrap(), next) 
            },
            Action::Nominated(s) => { 
                return next.suit == *s || next.rank == Rank::Ace
            },
            Action::PickedUp(_) | Action::Skipped => {
                is_active = false;
                idx -= 1;
                continue;
            }
        }
    }
}

pub fn can_link(prev: Card, next: Card) -> bool
{
    if (prev.rank == Rank::Ace) ^ (next.rank == Rank::Ace) {
        return false;
    }

    prev.rank == next.rank
    || (prev.suit == next.suit && Rank::adjacent(prev.rank, next.rank))
    || (prev.rank == Rank::King
        && prev.suit == next.suit 
        && match next.rank {
            Rank::Val(2) | Rank::Val(8) | Rank::Ace => false,
            Rank::Jack => next.suit.is_red(),
            _ => true
        })
}

pub fn can_go(log: &[Turn], hand: &HashSet<Card>) -> bool
{
    hand.iter().any(|card| can_follow(log, *card))
}

pub fn is_valid(log: &[Turn], chain: &[Card]) -> bool
{
    if !can_follow(log, chain[0]) { return false; }

    for i in 1..chain.len()
    {
        if !can_link(chain[i-1], chain[i]) { return false; }
    }

    true
}

pub fn make_chain_iterator<'a>(hand: &HashSet<Card>) -> impl StreamingIterator<Item = &'a [Card]>
{
    use std::marker;

    struct ChainIterator<'a>
    {
        pub stack: Vec<Vec<Card>>,
        pub chain_length: usize,
        pub idx: usize,
        _marker: marker::PhantomData<&'a Card>
    }

    impl<'a> StreamingIterator for ChainIterator<'a> {
        
        type Item = &'a[Card];

        fn advance(&mut self) 
        { 
            todo!()
        }
        
        fn get(&self) -> std::option::Option<&<Self as streaming_iterator::StreamingIterator>::Item> 
        { 
            todo!() 
        }
    }

    let mut iterator = ChainIterator { 
        stack: Vec::new(),
        chain_length: 0,
        idx: 0,
        _marker: marker::PhantomData
    };

    iterator.stack.push(hand.iter().cloned().collect());

    iterator
}