use strum::IntoEnumIterator;

use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::player::*;
use crate::cards::*;
use crate::strategy::*;
use crate::blackjack::*;

use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone)]
pub struct Action(usize,Card);

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
                for i in 0..self.discard_pile.len() {
                    self.deck.push(self.discard_pile[i]);
                }
                self.discard_pile.clear();
                self.deck.shuffle(&mut thread_rng());
                self.deck.pop().unwrap()
            }
        }
    }
}