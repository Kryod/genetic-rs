use std::fmt::Display;

use rand::{thread_rng, Rng, distributions::Alphanumeric};

mod criterion;
mod selector;

use criterion::Criterion;
use selector::Selector;

fn generator() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect()
}

fn evaluator(pop: &String) -> f32 {
    let mut val = 0.0;
    let solution = String::from("iloveyou");

    for (c, cs) in pop.chars().zip(solution.chars()) {
        if c == cs {
            val += 1.0;
        } else {
            let diff = (c as u32).abs_diff(cs as u32);
            val += 0.5 - (0.5 * (1.0/diff as f32));
        }
    }

    val
}

fn crossover(parent1: &String, parent2: &String) -> String {
    let result = parent1.clone();
    let pos: usize = thread_rng().gen_range(0..parent1.len());
    let replace_len: usize = thread_rng().gen_range(1..=parent1.len() - pos);

    result.replace(&parent1[pos..pos+replace_len], &parent2[pos..pos+replace_len])
}

fn mutation(pop: &mut String) {
    let mut new_string = String::new();
    let mut rng = thread_rng();
    for c in pop.chars() {
        if rng.gen_range(0..100) < 10 {
            new_string.push(rng.sample(Alphanumeric) as char);
        } else {
            new_string.push(c);
        }
    }
    *pop = new_string;
}

fn generate<T, G, E, S, C, M, F>(generator: &G, evaluator: &E, selector: &S,
    crossover: &C, mutation: &M, stop_crit: &F , pop_size: u32) -> T
where 
    G: Fn() -> T,
    E: Fn(&T) -> f32,
    F: Criterion,
    S: Selector<T>,
    C: Fn(&T, &T) -> T,
    M: Fn(&mut T),
    T: Display {

        
    let mut rng = thread_rng();

    let mut pop = Vec::with_capacity(pop_size as usize);

    for _ in 0..pop_size {
        pop.push(generator());
    }

    let mut ratings = Vec::with_capacity(pop_size as usize);

    for p in &pop {
        ratings.push(evaluator(p));
    }

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
            let mut child = crossover(&parents[id1], &parents[id2]);

            // Chances of mutation happening
            if rng.gen_range(0..=100) < 20 {
                // Mutating the new element
                mutation(&mut child);
            }
            pop.push(child);
        }

        // Calculate fitness of new generation
        ratings.clear();
        for p in &pop {
            ratings.push(evaluator(p));
        }
    
        let (mut best, mut index) = (0.0, 0);
        ratings.iter().enumerate().for_each(|(i, v)| if *v > best {best = *v; index = i;});

        println!("Gen: {gen}. Best rating: {best:.2}");
        println!("Best element: {}", &pop[index]);

        gen += 1;
    }

    println!("Finished in {gen} generations.");
    
    let (mut best, mut index) = (0.0, 0);
    ratings.iter().enumerate().for_each(|(i, v)| if *v > best {best = *v; index = i;});

    pop.remove(index)
}

fn main() {
    let x = generate(&generator, &evaluator, &selector::Rating{ max_pop: 300 }, &crossover, &mutation, &criterion::Mark{ max_rating: 8.0 }, 1000);
    println!("Solution: {x}");

}
