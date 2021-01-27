use crate::cards::*;
use crate::game::*;
use crate::player::*;
use crate::blackjack::*;

use std::collections::HashSet;

pub struct HumanStrategy {}

fn parse_chain(s: &str) -> Option<Vec<Card>>
{
    let mut chain = Vec::new();

    if s.trim().is_empty() { return Some(chain); }

    for ss in s.trim().split(",") {
        
        if ss.len() != 2 { return None; }
        
        let rank;
        let suit;
        
        match ss.chars().nth(0) {
            Some(rc) => match rc {
                'a' | 'A' => { rank = Rank::Ace; },
                'k' | 'K' => { rank = Rank::King; },
                'q' | 'Q' => { rank = Rank::Queen; },
                'j' | 'J' => { rank = Rank::Jack; },
                'x' | 'X' => { rank = Rank::Val(10); },
                s => match s.to_digit(10) {
                    Some(i) => match i {
                        1 => { return None; },
                        _ => { rank = Rank::Val(i as i32); } 
                    },
                    _ => { return None; }
                }
            },
            None => { return None; }
        }

        match ss.chars().nth(1) {
            Some(sc) => match parse_suit(sc) {
                    Some(s) => { suit = s; },
                    None => { return None; }
            },
            None => { return None; }
        }

        chain.push(Card { rank, suit });        
    }

    Some(chain)
}

fn hand_contains_chain(hand: &HashSet<Card>, chain: &[Card]) -> bool
{
    for card in chain { if !hand.contains(card) { return false; } }
    return true;
}

fn parse_suit(c: char) -> Option<Suit>
{
    match c {
        's' | 'S' => Some(Suit::Spades),
        'h' | 'H' => Some(Suit::Hearts),
        'd' | 'D' => Some(Suit::Diamonds),
        'c' | 'C' => Some(Suit::Clubs),
        _ => None
    }
}

impl Strategy for HumanStrategy
{
    fn choose_next(&self, hand: &HashSet<Card>, log: &[Turn]) -> Vec<Card>
    {
        loop {
            
            println!("Current hand: {:?}", hand);
            println!("Choose cards to put down:");
            
            let mut input = String::new();
            
            match std::io::stdin().read_line(&mut input) {
                Ok(_) => {
                    match parse_chain(&input) {
                        Some(chain) => {
                            if !hand_contains_chain(hand, &chain) {
                                println!("Cannot make chain from hand.");
                                continue;
                            }
                            if chain.len() > 0 {
                                if !is_valid(log, &chain) {
                                    println!("Chain is invalid.");
                                    continue;
                                }
                            } else {
                                if can_go(log, hand) {
                                    println!("Valid cards in deck - you must go.");
                                    continue;
                                }
                            }
                            return chain 
                        }, 
                        _ => {}
                    }
                }, 
                Err(_) => {
                    println!("Invalid input.");
                    continue;
                }
            }
        }   
    }

    fn choose_suit(&self, hand: &HashSet<Card>, _: &[Turn]) -> Suit
    {
        loop {
            
            println!("Current hand: {:?}", hand);
            println!("Choose suit:");
            
            let mut input = String::new();
            
            match std::io::stdin().read_line(&mut input) {
                
                Ok(_) => {

                    if let Some(c) = input.chars().nth(0) {
                        if let Some(suit) = parse_suit(c) {
                            return suit;
                        }
                    }
                    
                    println!("Invalid input.");
                    continue;
                }, 
                Err(_) => {}
            }
        }
    }

    fn name(&self) -> &str { "Human Player" }
}