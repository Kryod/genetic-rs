use std::time::Instant;

use genetic_rs::criterion::Mark;
use genetic_rs::selector::*;

mod sudoku;
mod my_crossover;
mod my_generator;
mod my_evaluator;
mod my_mutation;

use my_crossover::*;
use my_evaluator::*;
use my_generator::*;
use my_mutation::*;

use sudoku::Sudoku;

fn main() {

    let cell1 = [0, 0, 4,
        0, 6, 0,
        8, 2, 0];
    let cell2 = [9, 6, 2,
        1, 0, 0,
        3, 7, 0];
    let cell3 = [3, 0, 0,
        4, 0, 0,
        0, 0, 6];
    let cell4 = [0, 0, 1,
        0, 0, 2,
        0, 9, 3];
    let cell5 = [4, 0, 6,
        7, 5, 0,
        2, 0, 0];
    let cell6 = [0, 2, 0,
        0, 0, 0,
        7, 0, 4];
    let cell7 = [2, 7, 0,
        1, 0, 0,
        9, 0, 0];
    let cell8 = [0, 3, 0,
        0, 0, 0,
        8, 2, 0];
    let cell9 = [9, 4, 0,
        2, 7, 5,
        0, 0, 1];
    let sudoku = Sudoku::new([cell1, cell2, cell3, cell4, cell5, cell6, cell7, cell8, cell9]);
    //let selector = Rating{ max_pop: 200 };
    //let selector = Elitism{ max_pop: 200 };
    let selector = Rank{ max_pop: 200 };
    //let selector = Tournament{ max_pop: 200 };
    let evaluator = BasicEvaluation;
    let generator = BasicGenerator{ sudoku };
    let stop_crit = Mark{ max_rating: u8::MAX as f32 };
    let crossover = BasicCrossover;
    let mutation = BasicMutation;
    let pop_size = 1000;

    let instant = Instant::now();

    let (solution, gen) = genetic_rs::generate(&generator, &evaluator, &selector,
        &crossover, &mutation, &stop_crit, pop_size);
        
    let time = instant.elapsed().as_millis();
    println!("Found solution: {solution} ; in {gen} generations and in {time}ms");
}
