use rand::{thread_rng, Rng, seq::SliceRandom};

use genetic_rs::mutation::Mutation;
use crate::sudoku::Sudoku;

pub struct BasicMutation;

impl Mutation<Sudoku> for BasicMutation
{
    fn mutation(&self, pop: &mut Sudoku) {
        let mut rng = thread_rng();

        for cell in &mut pop.cells {

            let mut to_permute: Vec<_> = (0..9).into_iter().filter(|x| !cell.immutables.contains(x)).collect();
            to_permute.shuffle(&mut rng);
            for _ in to_permute.clone() {
                if rng.gen_bool(0.20) {
                    if to_permute.len() > 1 {
                        cell.data.swap(to_permute.pop().unwrap(), to_permute.pop().unwrap());
                    }
                }
            }
        }
    }
}

pub struct SingleMutation;

impl Mutation<Sudoku> for SingleMutation
{
    fn mutation(&self, pop: &mut Sudoku) {
        let mut rng = thread_rng();
        let mutating_cell = pop.cells.choose_mut(&mut rng).unwrap();

        let mut to_permute: Vec<_> = (0..9).into_iter().filter(|x| !mutating_cell.immutables.contains(x)).collect();
        to_permute.shuffle(&mut rng);

        mutating_cell.data.swap(to_permute.pop().unwrap(), to_permute.pop().unwrap());
    }
}