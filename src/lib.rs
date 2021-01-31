use pyo3::prelude::*;
use pyo3::types::PyType;

mod cards;
mod player;
mod game;
mod strategy;
mod blackjack;

use crate::game::*;
use crate::strategy::*;
use crate::cards::*;

const PRINT : bool = false;

#[pymodule]
fn blackjack(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<GameWrapper>()?;
    Ok(())
}

#[pyclass]
#[text_signature = "(player_count, external_player_idx, /)"]
struct GameWrapper {
    g: Game
}



#[pymethods]
impl GameWrapper {
    
    #[new]
    fn new(player_count: usize, external_player_idx: usize) -> Self {
        Self {
            g: Game::new(player_count, external_player_idx)
        }
    }
    
    #[text_signature = "($self)"]
    fn observable_state(&mut self) -> Vec<f32> {
        Vec::new()
    }

    #[text_signature = "($self, /)"]
    fn next_valid_action(&mut self) -> Vec<i32> {
        Vec::new()
    }

    #[text_signature = "($self, /)"]
    fn choose_action(&mut self) -> f32 { // (state,reward)
        0.0
    }
}