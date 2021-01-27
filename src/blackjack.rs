use crate::cards::*;
use crate::game::*;

use std::collections::HashSet;

pub const STARTING_CARD_COUNT : usize = 7;

pub fn is_penalty_card(card: Card) -> bool
{
    if let Some(_) = penalty_value(card) { return true; }
    false
}

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

fn can_follow_card(active: bool, prev: Card, next: Card) -> bool
{
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
            _ => prev.suit == next.suit || prev.rank == next.rank
        }
    } else {
        prev.suit == next.suit || prev.rank == next.rank
    }
}

pub fn can_follow(log: &[Turn], next: Card) -> bool
{
    let mut idx = log.len()-1;
    let mut is_active = true;

    loop {
        match &log[idx].action {
            Action::First(prev) => { return can_follow_card(is_active, *prev, next); },
            Action::Played(chain) => { return can_follow_card(is_active, *chain.last().unwrap(), next) },
            Action::Nominated(s) => { return next.suit == *s },
            _ => {
                is_active = false;
                idx -= 1;
                continue;
            }
        }
    }
}

pub fn can_link(prev: Card, next: Card) -> bool
{
    prev.rank == next.rank
    || (prev.suit == next.suit && Rank::adjacent(prev.rank, next.rank))
    || (prev.rank == Rank::King
        && prev.suit == next.suit 
        && match next.rank {
            Rank::Val(2) | Rank::Val(8) => false,
            Rank::Jack => next.suit.is_red(),
            _ => true
        })
    || prev.rank == Rank::Ace
}

pub fn can_go(log: &[Turn], hand: &HashSet<Card>) -> bool
{
    for card in hand
    {
        if can_follow(log, *card) { return true; }
    }

    return false;
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