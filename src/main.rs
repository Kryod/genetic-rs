use std::{fmt::Display, time::Instant};

use rand::{thread_rng, Rng};

mod criterion;
mod selector;
mod evaluator;
mod generator;
mod crossover;
mod mutation;

use criterion::*;
use selector::*;
use evaluator::*;
use generator::*;
use crossover::*;
use mutation::*;

const NUM_THREADS: u32 = 8;

fn fill_ratings<T, E>(pop_size: u32, pop: &Vec<T>, evaluator: &E, ratings: &mut Vec<f32>)
where
    E: Evaluator<T> + Send + Sync,
    T: Send + Sync {
    std::thread::scope(|scope| {
        let mut pop_handled = pop_size / NUM_THREADS;
        let mut offset = 0;
        if pop_size % NUM_THREADS != 0 {
            offset += pop_size % NUM_THREADS;
        }
        let mut threads = vec![];

        for thread in 0..NUM_THREADS {
            
            let begin = (thread*pop_handled) as usize;
            if thread == NUM_THREADS - 1 {
                pop_handled += offset;
            }
            let thread_pop = &pop[begin .. begin + pop_handled as usize];
            threads.push(scope.spawn(move || {
                
                let mut buf = Vec::with_capacity(pop_handled as usize);

                for p in thread_pop {
                    buf.push(evaluator.evaluator(p));
                }

                buf
            }))
        }

        for thread in threads {
            let partial_data = thread.join().expect("Thread did not close correctly");

            ratings.extend(partial_data);
        }

    });
}

fn generate<T, G, E, S, C, M, F>(generator: &G, evaluator: &E, selector: &S,
    crossover: &C, mutation: &M, stop_crit: &F , pop_size: u32) -> (T, i32)
where 
    G: Generator<T>,
    E: Evaluator<T> + Send + Sync,
    F: Criterion,
    S: Selector<T>,
    C: Crossover<T>,
    M: Mutation<T>,
    T: Display + Send + Sync {

        
    let mut rng = thread_rng();

    let mut pop = Vec::with_capacity(pop_size as usize);

    for _ in 0..pop_size {
        pop.push(generator.generator());
    }

    let mut ratings = Vec::with_capacity(pop_size as usize);

    fill_ratings(pop_size, &pop, evaluator, &mut ratings);

    let mut gen = 0;

    // Check if criterion has been reached
    while !stop_crit.criterion(&ratings) {
        let parents = selector.selector(&pop, &ratings);
        let parents_size = parents.len();

        // New generation from the fittest individuals of the previous
        // population
        pop.clear();
        for _ in 0..pop_size {
            
            let id1: usize = rng.gen_range(0..parents_size);
            let mut id2: usize = rng.gen_range(0..parents_size);
            while id2 == id1 { id2 = rng.gen_range(0..parents_size) };

            // Crossing 2 parents to generate a new element
            let mut child = crossover.crossover(&parents[id1], &parents[id2]);

            // Chances of mutation happening
            if rng.gen_range(0..=100) < 20 {
                // Mutating the new element
                mutation.mutation(&mut child);
            }
            pop.push(child);
        }

        // Calculate fitness of new generation
        ratings.clear();
        fill_ratings(pop_size, &pop, evaluator, &mut ratings);
    
        let (mut best, mut index) = (0.0, 0);
        ratings.iter().enumerate().for_each(|(i, v)| if *v > best {best = *v; index = i;});

        //println!("Gen: {gen}. Best rating: {best:.3}");
        //println!("Best element: {}", &pop[index]);

        gen += 1;
    }
    
    let (mut best, mut index) = (0.0, 0);
    ratings.iter().enumerate().for_each(|(i, v)| if *v > best {best = *v; index = i;});

    (pop.remove(index), gen)
}

fn main() {
    //let selector = Rating{ max_pop: 200 };
    //let selector = Elitism{ max_pop: 200 };
    let selector = Rank{ max_pop: 200 };
    let evaluator = BasicEvaluation{ solution: String::from("coucoualexjtmbb") };
    let generator = BasicGenerator{ string_size: evaluator.solution.len() };
    let stop_crit = Mark{ max_rating: evaluator.solution.len() as f32 };
    let crossover = BasicCrossover;
    let mutation = BasicMutation;
    let pop_size = 1000;

    let instant = Instant::now();

    let (solution, gen) = generate(&generator, &evaluator, &selector,
        &crossover, &mutation, &stop_crit, pop_size);
        
    let time = instant.elapsed().as_millis();
    println!("Found solution: {solution} ; in {gen} generations and in {time}ms");
}
