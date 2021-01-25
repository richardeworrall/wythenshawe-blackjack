use std::collections::HashSet;

use crate::cards::*;
use crate::strategy::*;
use crate::game::*;

pub struct Player<'a>
{
    pub name: String,
    pub hand: HashSet<Card>,
    pub score: i32,
    pub strategy: &'a dyn Strategy
}

impl std::fmt::Debug for Player<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Player")
         .field("name", &self.name)
         .field("hand", &self.hand)
         .field("score", &self.score)
         .field("strategy", &self.strategy.name())
         .finish()
    }
}

impl Player<'_>
{
    pub fn new<'a>(name: String, strategy: &'a dyn Strategy) -> Player<'a>
    {
        Player 
        {
            name,
            hand: HashSet::<Card>::new(),
            score: 0,
            strategy
        }
    }

    pub fn choose_next(&self, game: &Game) -> Vec<Card>
    {
        self.strategy.choose_next(&self.hand, &game.log)
    }
}