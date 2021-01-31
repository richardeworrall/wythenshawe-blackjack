use std::collections::HashSet;

use crate::cards::*;
use crate::game::*;
use crate::strategy::*;

pub struct Player
{
    pub name: String,
    pub hand: HashSet<Card>,
    pub score: i32,
    pub strategy: Box<dyn Strategy + Send>
}

impl std::fmt::Debug for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Player")
         .field("name", &self.name)
         .field("hand", &self.hand)
         .field("score", &self.score)
         .field("strategy", &self.strategy.name())
         .finish()
    }
}

impl Player
{
    pub fn new(name: String, strategy: Box<dyn Strategy + Send>) -> Player
    {
        Player 
        {
            name,
            hand: HashSet::<Card>::new(),
            score: 0,
            strategy
        }
    }

    pub fn choose_next(&self, log: &[Turn]) -> Vec<Card>
    {
        self.strategy.choose_next(&self.hand, log)
    }

    pub fn choose_suit(&self, log: &[Turn]) -> Suit
    {
        self.strategy.choose_suit(&self.hand, log)
    }
}