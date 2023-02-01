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

            let to_permute: Vec<_> = (0..9).into_iter().filter(|x| !cell.immutables.contains(x)).collect();
            for index in to_permute {
                if rng.gen_bool(0.10) {
                    cell.data[index] = parent2.cells[id].data[index];
                }
            }
        }
    
        result
    }
}