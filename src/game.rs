use rand::thread_rng;
use rand::Rng;
use rand::seq::SliceRandom;

use crate::player::*;
use crate::cards::*;
use crate::strategies::{computer::*,human::*};
use crate::blackjack::*;

use std::fmt::Debug;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Action
{
    Played(Vec<Card>),
    Nominated(Suit),
    PickedUp(i32),
    First(Card),
    Skipped
}

#[derive(Debug, PartialEq, Clone)]
pub struct Turn
{
    pub player: Option<usize>,
    pub action: Action
}

#[derive(Debug)]
pub struct Game<'a>
{
    pub players: Vec<Player<'a>>,
    pub deck: Vec<Card>,
    pub discard_pile: Vec<Card>,
    pub log: Vec<Turn>,
    pub curr_player_id: usize,
    pub penalty: i32
}

impl<'a> Game<'a>
{
    pub fn new(num_players: usize, first_to_play: usize) -> Game<'a>
    {
        if num_players < 2 { panic!("Two players min") };
        if num_players > 6 { panic!("Six players max") };
        if first_to_play >= num_players { panic!("Invalid first player") };

        let human_player = rand::thread_rng().gen_range(0..num_players);

        let mut players = Vec::<Player>::new();
        
        for i in 0..num_players {
            if i == human_player {
                players.push(Player::new(format!("Player {} (Human)", i), &HumanStrategy {}));
            } else {
                players.push(Player::new(format!("Player {}", i), &ComputerStrategy {}));
            }
        }

        let mut game = Game {
            players: players,
            deck: Vec::<Card>::new(),
            discard_pile: Vec::<Card>::new(),
            curr_player_id: first_to_play,
            log: Vec::<Turn>::new(),
            penalty: 0
        };

        game.populate_deck();
        game.deal();

        let first = game.draw();
        
        if is_penalty_card(first) {
            game.penalty += penalty_value(first).unwrap();
        }

        game.discard_pile.push(first);
        game.log.push(Turn { player: None, action: Action::First(first) });
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

    fn player_should_skip(&self) -> bool
    {
        match &self.log.last().unwrap().action {
            Action::Played(c) => match c.last().unwrap() {
                Card { rank: Rank::Val(8), suit: _ } => true,
                _ => false
            },
            _ => false
        }
    }

    pub fn run(&mut self)
    {
        println!("First card is {:?}", self.discard_pile.last().unwrap());

        if self.discard_pile.last().unwrap().rank == Rank::Ace {
            let suit = self.players[self.curr_player_id].choose_suit(&self.log);
            
            self.log.push(Turn { 
                player: Some(self.curr_player_id), 
                action: Action::Nominated(suit) 
            });

            println!("{} nominates: {:?}", self.players[self.curr_player_id].name, suit);
        }

        loop {
            
            let chain = self.players[self.curr_player_id].choose_next(&self.log);

            if chain.len() == 0 {
                
                if self.penalty > 0 {

                    for _ in 0..self.penalty {
                        let next = self.draw();
                        self.players[self.curr_player_id].hand.insert(next);
                    }

                    self.log.push(Turn {
                        player: Some(self.curr_player_id),
                        action: Action::PickedUp(self.penalty)
                    });

                    println!("{} picks up {}.", self.players[self.curr_player_id].name, self.penalty);

                    self.penalty = 0;

                } else if self.player_should_skip() {
                    
                    self.log.push(Turn {
                        player: Some(self.curr_player_id),
                        action: Action::Skipped
                    });

                    println!("{} misses a go.", self.players[self.curr_player_id].name);

                } else {
                    
                    let pick_up = self.draw();
                    self.players[self.curr_player_id].hand.insert(pick_up);
                    
                    self.log.push(Turn {
                        player: Some(self.curr_player_id),
                        action: Action::PickedUp(1)
                    });

                    println!("{} can't go; picks up {}.", self.players[self.curr_player_id].name, 1);
                }

            } else {

                if !is_valid(&self.log, &chain) { 
                    panic!("{} tried to play an invalid strategy!", self.players[self.curr_player_id].name);
                }
                else {

                    for c in chain.iter() {
                        if is_penalty_card(*c) {
                            self.penalty += penalty_value(*c).unwrap();
                        } else {
                            self.penalty = 0;
                        }
                    }
                    
                    self.discard_pile.extend(&chain);
                    
                    self.log.push(Turn { 
                        player: Some(self.curr_player_id), 
                        action: Action::Played(chain.clone()) 
                    });

                    println!("{} plays: {:?}", self.players[self.curr_player_id].name, &chain);

                    match chain.last().unwrap().rank {
                        Rank::Ace => {
                            let suit = self.players[self.curr_player_id].choose_suit(&self.log);
                        
                            self.log.push(Turn { 
                                player: Some(self.curr_player_id), 
                                action: Action::Nominated(suit) 
                            });
    
                            println!("{} nominates: {:?}", self.players[self.curr_player_id].name, suit);
                        },
                        Rank::King => {
                            
                            let pick_up = self.draw();
                            self.players[self.curr_player_id].hand.insert(pick_up);
                            
                            self.log.push(Turn {
                                player: Some(self.curr_player_id),
                                action: Action::PickedUp(1)
                            });

                            println!("{} finished with {:?} so picks up.", 
                                self.players[self.curr_player_id].name, chain.last().unwrap());
                        },
                        _ => ()
                    }
                    
                    for c in &chain { self.players[self.curr_player_id].hand.remove(&c); }
                }
            }

            if self.players[self.curr_player_id].hand.is_empty() {
                println!("{} wins!", self.players[self.curr_player_id].name);
                break;
            } else {
                self.curr_player_id += 1;
                if self.curr_player_id == self.players.len() { self.curr_player_id = 0; }
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