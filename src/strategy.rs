pub mod computer_v1;
pub mod computer_v2;

mod shared;

use std::collections::HashSet;

use crate::cards::*;
use crate::game::*;

use computer_v1::*;
use computer_v2::*;

pub trait Strategy {
    fn choose_next(&mut self, hand: &HashSet<Card>, log: &[Turn]) -> Vec<Card>;
    fn choose_suit(&mut self, hand: &HashSet<Card>, log: &[Turn]) -> Suit;
    fn name(&self) -> &str;
}

#[derive(Clone,Copy,Debug)]
pub enum StrategyType
{
    ComputerV1,
    ComputerV2
}

pub fn make_strategy<'a>(t: &StrategyType, num_players: usize) -> Box<dyn Strategy + Send>
{
    match t {
        StrategyType::ComputerV1 => Box::new(ComputerStrategyV1 {}),
        StrategyType::ComputerV2 => Box::new(ComputerStrategyV2::new(num_players))
    }
}