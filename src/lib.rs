use std::{fmt::Display};

use rand::{thread_rng, Rng};

pub mod criterion;
pub mod selector;
pub mod evaluator;
pub mod generator;
pub mod crossover;
pub mod mutation;

use criterion::Criterion;
use selector::Selector;
use evaluator::Evaluator;
use generator::Generator;
use crossover::Crossover;
use mutation::Mutation;

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

pub fn generate<T, G, E, S, C, M, F>(generator: &G, evaluator: &E, selector: &S,
    crossover: &C, mutation: &M, stop_crit: &mut F , pop_size: u32) -> (T, i32, f32)
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
    #[cfg(debug_assertions)]
    let mut last_best = 0.0;

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
            if rng.gen_range(1..=100) < 25 {
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
        #[cfg(debug_assertions)]
        {
            if best != last_best {
                println!("Gen: {gen}. Best rating: {best:.3}");
                last_best = best;
            }
        }

        gen += 1;
    }
    
    let (mut best, mut index) = (0.0, 0);
    ratings.iter().enumerate().for_each(|(i, v)| if *v > best {best = *v; index = i;});

    (pop.remove(index), gen, best)
}
