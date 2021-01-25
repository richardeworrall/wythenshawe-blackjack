use strum::IntoEnumIterator;

use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::player::*;
use crate::cards::*;
use crate::strategy::*;
use crate::blackjack::*;

use std::fmt::Debug;
use std::time::{Duration, Instant};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Action(pub usize, pub Card);

#[derive(Debug)]
pub struct Game<'a>
{
    pub players: Vec<Player<'a>>,
    pub deck: Vec<Card>,
    pub discard_pile: Vec<Card>,
    pub log: Vec<Action>,
    pub next_to_play: usize
}

impl<'a> Game<'a>
{
    pub fn new(num_players: usize, first_to_play: usize) -> Game<'a>
    {
        if num_players < 2 { panic!("Two players min") };
        if num_players > 6 { panic!("Six players max") };
        if first_to_play >= num_players { panic!("Invalid first player") };

        let mut players = Vec::<Player>::new();
        
        for i in 0..num_players {
            players.push(Player::new(format!("Player {}", i), &MinimiseScoreStrategy {}));
        }

        let mut game = Game {
            players: players,
            deck: Vec::<Card>::new(),
            discard_pile: Vec::<Card>::new(),
            next_to_play: first_to_play,
            log: Vec::<Action>::new()
        };

        game.populate_deck();
        game.deal();

        let first = game.draw();
        game.discard_pile.push(first);
        game.log.push(Action(usize::MAX,first));
        game
    }

    fn deal(&mut self)
    {
        for _ in 0..STARTING_CARD_COUNT {
            for p in 0..self.players.len() {
                let card = self.draw();
                self.players[p].hand.insert(card);
            }
        }
    }

    pub fn populate_deck(&mut self)
    {
        let mut deck : Vec<Card> = Card::full_deck();
        deck.shuffle(&mut thread_rng());
        self.deck = deck;
    }

    pub fn draw(&mut self) -> Card
    {
        match self.deck.pop() {
            Some(c) => return c,
            None => {
                
                for i in 0..self.discard_pile.len()-1 {
                    self.deck.push(self.discard_pile[i]);
                }

                let last = self.discard_pile[self.discard_pile.len()-1];
                self.discard_pile.clear();
                self.discard_pile.push(last);

                self.deck.shuffle(&mut thread_rng());
                self.deck.pop().unwrap()
            }
        }
    }

    pub fn is_valid(&self, chain: &[Card]) -> bool
    {
        if !can_follow(*self.discard_pile.last().unwrap(), chain[0]) { return false; }

        for i in 1..chain.len()
        {
            if !can_link(chain[i-1], chain[i]) { return false; }
        }
        
        true
    }

    pub fn run(&mut self)
    {
        println!("First card is {:?}", self.discard_pile.first().unwrap());

        loop {            
            let before = Instant::now();
            let chain = self.players[self.next_to_play].choose_next(&self);
            let after = Instant::now();

            if chain.len() == 0 {
                let pick_up = self.draw();
                let hand_before : Vec<Card> = self.players[self.next_to_play].hand.iter().cloned().collect();
                self.players[self.next_to_play].hand.insert(pick_up);
                let hand_after : Vec<Card> = self.players[self.next_to_play].hand.iter().cloned().collect();
                println!("{} can't go; picks up {:?}. Hand Before: {:?} Hand After: {:?}", 
                    self.players[self.next_to_play].name, pick_up, hand_before, hand_after);
            } else {
                if !self.is_valid(&chain) { panic!("{} played invalid strategy!", self.players[self.next_to_play].name) }
                else { 
                    self.discard_pile.extend(&chain);
                    for c in &chain { self.log.push(Action(self.next_to_play, *c)) }
                    let hand_before : Vec<Card> = self.players[self.next_to_play].hand.iter().cloned().collect();
                    for c in &chain { self.players[self.next_to_play].hand.remove(&c); }
                    let hand_after : Vec<Card> = self.players[self.next_to_play].hand.iter().cloned().collect();
                    println!("{} plays: {:?}. Hand Before: {:?} Hand After: {:?}", 
                        self.players[self.next_to_play].name, &chain, hand_before, hand_after);
                }
            }

            println!("(decision reached in {:?})", after - before);

            if self.players[self.next_to_play].hand.is_empty() {
                println!("{} wins!", self.players[self.next_to_play].name);
                break;
            } else {
                self.next_to_play += 1;
                if self.next_to_play == self.players.len() { self.next_to_play = 0; }
            }
        }

        for p in self.players.iter_mut()
        {
            p.score += chain_score(p.hand.iter())
        }

        let standings = self.players.iter().fold(HashMap::new(), |mut map, p| {
            map.insert(&p.name, p.score);
            map
        });

        println!("Standings: {:?}", standings);
    }
}