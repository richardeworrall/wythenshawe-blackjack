use crate::cards::*;
use crate::game::*;

use std::collections::HashSet;

pub const STARTING_CARD_COUNT : usize = 7;

pub fn penalty_value(card: Card) -> usize
{
    match card {
        Card { rank: Rank::Jack, suit: s } if s.is_black() => 5,
        Card { rank: Rank::Val(2), suit: _ } => 2,
        _ => 0
    }
}

pub fn outstanding_penalty(log: &[Turn]) -> usize
{
    let mut penalty : usize = 0;

    for turn in log.iter().rev() {
        match &turn.action {
            Action::Played(chain) => {
                for i in (0..chain.len()).rev() {
                    let card = chain[i];
                    let this_card_penalty = penalty_value(card);
                    
                    if this_card_penalty > 0 {
                        penalty += this_card_penalty;
                    } else {
                        return penalty;
                    }
                }
            },
            Action::First(card) => {
                penalty += penalty_value(*card);
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