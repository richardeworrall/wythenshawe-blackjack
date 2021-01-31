pub mod human;
pub mod computer;

mod shared;

use std::collections::HashSet;

use crate::cards::*;
use crate::game::*;

use human::*;
use computer::*;

pub trait Strategy {
    fn choose_next(&self, hand: &HashSet<Card>, log: &[Turn]) -> Vec<Card>;
    fn choose_suit(&self, hand: &HashSet<Card>, log: &[Turn]) -> Suit;
    fn name(&self) -> &str;
}

#[derive(Clone,Copy,Debug)]
pub enum StrategyType
{
    Computer,
    Human
}

pub fn make_strategy<'a>(t: &StrategyType) -> Box<dyn Strategy + Send>
{
    match t {
        StrategyType::Computer => Box::new(ComputerStrategy {}),
        StrategyType::Human => Box::new(HumanStrategy {}),
    }
}