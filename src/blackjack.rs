use crate::cards::*;

pub const STARTING_CARD_COUNT : usize = 7;

pub fn card_score(card: &Card) -> i32
{
    match card {
        Card { rank: Rank::Ace, suit: _ } => 25,
        Card { rank: Rank::Val(8), suit: _ } => 20,
        Card { rank: Rank::Val(2), suit: _ } => 20,
        Card { rank: Rank::Jack, suit: s } if s.colour() == Colour::Black => 15,
        Card { rank: r, suit: _ } => r.face_value() 
    }
}

pub fn chain_score<'a, T>(chain: T) -> i32
where T : Iterator<Item = &'a Card>
{
    chain.map(|c| card_score(c)).sum()
}

pub fn can_follow(prev: Card, next: Card) -> bool
{
    prev.suit == next.suit
}

pub fn can_link(prev: Card, next: Card) -> bool
{
    prev.rank == next.rank || (prev.suit == next.suit && Rank::adjacent(prev.rank, next.rank))
}