use crate::cards::*;

pub const STARTING_CARD_COUNT : usize = 7;

pub fn card_score(card: &Card) -> i32
{
    match card {
        Card(Rank::Ace,_) => 25,
        Card(Rank::Val(8), _) => 20,
        Card(Rank::Val(2), _) => 20,
        Card(Rank::Jack, s) if s.colour() == Colour::Black => 15,
        Card(r,_) => r.face_value()
    }
}

pub fn chain_score(chain: &[Card]) -> i32
{
    chain.iter().map(|c| card_score(c)).sum()
}