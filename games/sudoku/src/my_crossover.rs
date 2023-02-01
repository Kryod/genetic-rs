use rand::{thread_rng, Rng};

use genetic_rs::crossover::Crossover;
use crate::sudoku::Sudoku;

pub struct BasicCrossover;

impl Crossover<Sudoku> for BasicCrossover
{
    fn crossover(&self, parent1: &Sudoku, parent2: &Sudoku) -> Sudoku {
        let mut result = parent1.clone();
        let mut rng = thread_rng();

        for (id, cell) in result.cells.iter_mut().enumerate() {

            if rng.gen_bool(0.50) {
                cell.data = parent2.cells[id].data;
            }
        }
    
        result
    }
}

pub struct HalfCrossover;

impl Crossover<Sudoku> for HalfCrossover
{
    fn crossover(&self, parent1: &Sudoku, parent2: &Sudoku) -> Sudoku {
        let mut result = parent1.clone();
        let mut rng = thread_rng();
        let half = rng.gen_range(1..8);

        for (id, cell) in result.cells.iter_mut().enumerate() {

            if id >= half {
                cell.data = parent2.cells[id].data;
            }
        }
    
        result
    }
}