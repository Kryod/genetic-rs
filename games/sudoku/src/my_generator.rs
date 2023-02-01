use genetic_rs::generator::Generator;
use crate::sudoku::Sudoku;

pub struct BasicGenerator {
    pub sudoku: Sudoku
}

impl Generator<Sudoku> for BasicGenerator {

    fn generator(&self) -> Sudoku {
        self.sudoku.fill().randomize()
    }
}