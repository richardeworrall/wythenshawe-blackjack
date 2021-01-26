#[derive(PartialEq, Clone, Copy, Hash, Eq)]
pub enum Suit
{
    Diamonds,
    Hearts,
    Clubs,
    Spades
}

use std::fmt::Write;
impl std::fmt::Debug for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Suit::Diamonds => '♦',
            Suit::Hearts => '♥',
            Suit::Clubs => '♣',
            Suit::Spades => '♠'
        })
    }
}

#[derive(PartialEq, Clone, Copy, Hash, Eq)]
pub enum Rank
{
    Val(i32),
    Jack,
    Queen,
    King,
    Ace
}

impl std::fmt::Debug for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Rank::Val(10) => 'X',
            Rank::Val(i) => format!("{}",i).chars().nth(0).unwrap(),
            Rank::Jack => 'J',
            Rank::Queen => 'Q',
            Rank::King => 'K',
            Rank::Ace => 'A'
        })
    }
}

#[derive(PartialEq,Eq)]
pub enum Colour
{
    Red,
    Black
}

impl Suit {
    
    pub fn colour(&self) -> Colour
    {
        match self {
            Suit::Spades | Suit::Clubs => Colour::Black,
            Suit::Hearts | Suit::Diamonds => Colour::Red    
        }
    }

    pub fn is_black(&self) -> bool
    {
        self.colour() == Colour::Black
    }

    pub fn is_red(&self) -> bool
    {
        self.colour() == Colour::Red
    }

    pub fn all() -> [Suit;4]
    {
        return
        [
            Suit::Spades,
            Suit::Clubs,
            Suit::Hearts,
            Suit::Diamonds
        ]
    }
}

impl Rank {

    pub fn adjacent(x: Rank, y: Rank) -> bool
    {
        match y {
            Rank::Val(10) => x == Rank::Val(9) || x == Rank::Jack,
            Rank::Val(2) => x == Rank::Val(3) || x == Rank::Ace,
            Rank::Val(i) => x == Rank::Val(i-1) || x == Rank::Val(i+1),
            Rank::Jack => x == Rank::Val(10) || x == Rank::Queen,
            Rank::Queen => x == Rank::Jack || x == Rank::King,
            Rank::King => x == Rank::Queen || x == Rank::Ace,
            Rank::Ace => x == Rank::King || x == Rank::Val(2)
        }
    }

    pub fn face_value(&self) -> i32
    {
        match self {
            Rank::Val(v) => *v,
            Rank::Jack => 10,
            Rank::Queen => 10,
            Rank::King => 10,
            Rank::Ace => 10
        }
    }

    pub fn all() -> [Rank;13]
    {
        return
        [
            Rank::Val(2),
            Rank::Val(3),
            Rank::Val(4),
            Rank::Val(5),
            Rank::Val(6),
            Rank::Val(7),
            Rank::Val(8),
            Rank::Val(9),
            Rank::Val(10),
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Ace
        ]
    }
}

#[derive(PartialEq, Clone, Copy, Hash, Eq)]
pub struct Card {
    pub rank: Rank, 
    pub suit: Suit
}

impl Card {

    pub fn new(rank: Rank, suit: Suit) -> Card
    {
        Card {
            rank,
            suit
        }
    }

    pub fn full_deck() -> Vec<Card>
    {
        let suits = Suit::all();
        let ranks = Rank::all();
        
        suits
        .iter()
        .flat_map(|s| { ranks.iter().map(move |r| Card::new(*r, *s)) })
        .collect()
    }
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("⟨{:?}{:?}⟩", self.rank, self.suit))
    }
}