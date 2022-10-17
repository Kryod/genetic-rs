
use std::time::Instant;

use genetic_rs::criterion::Mark;
use genetic_rs::selector::*;

mod my_crossover;
mod my_evaluator;
mod my_generator;
mod my_mutation;

use my_crossover::*;
use my_evaluator::*;
use my_generator::*;
use my_mutation::*;


fn main() {
    //let selector = Rating{ max_pop: 200 };
    //let selector = Elitism{ max_pop: 200 };
    //let selector = Rank{ max_pop: 200 };
    let selector = Tournament{ max_pop: 100 };
    let evaluator = BasicEvaluation{ solution: String::from("coucoualexjtmbb") };
    let generator = BasicGenerator{ string_size: evaluator.solution.len() };
    let stop_crit = Mark{ max_rating: evaluator.solution.len() as f32 };
    let crossover = BasicCrossover;
    let mutation = BasicMutation;
    let pop_size = 1000;

    let instant = Instant::now();

    let (solution, gen) = genetic_rs::generate(&generator, &evaluator, &selector,
        &crossover, &mutation, &stop_crit, pop_size);
        
    let time = instant.elapsed().as_millis();
    println!("Found solution: {solution} ; in {gen} generations and in {time}ms");
}
