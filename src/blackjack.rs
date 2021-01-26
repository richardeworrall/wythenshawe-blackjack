use crate::cards::*;
use crate::game::*;

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
}

pub fn can_end_with(last: Card) -> bool
{
    match last {
        Card { rank: Rank::King, suit: _ } => false,
        _ => true
    }
}