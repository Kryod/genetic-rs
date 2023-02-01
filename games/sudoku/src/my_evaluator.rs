
use genetic_rs::evaluator::Evaluator;
use crate::sudoku::{Sudoku, Distance};

pub struct BasicEvaluation;

impl Evaluator<Sudoku> for BasicEvaluation {

    fn evaluator(&self, pop: &Sudoku) -> f32 {
        let mut val = 0;
    
        if let Err(Distance(d)) = pop.validate() {
            val += d;
        }
    
        (u8::MAX - val) as f32
    }
}