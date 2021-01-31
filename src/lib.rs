use pyo3::prelude::*;
use pyo3::types::PyType;

mod cards;
mod player;
mod game;
mod strategy;
mod blackjack;

use crate::game::*;
use crate::strategy::*;

const PRINT : bool = false;

#[pymodule]
fn blackjack(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<GameWrapper>()?;
    Ok(())
}

#[pyclass]
#[text_signature = "(g, /)"]
struct GameWrapper {
    g: Game
}

#[pymethods]
impl GameWrapper {
    
    #[new]
    fn new(c: i32) -> Self {
        Self {
            g: Game::new(&vec![StrategyType::ComputerV1; c as usize])
        }
    }
    
    #[text_signature = "($self)"]
    fn run(&mut self) {
        self.g.run();
    }
}