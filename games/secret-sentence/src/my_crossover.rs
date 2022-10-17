use rand::{thread_rng, Rng};

use genetic_rs::crossover::Crossover;

pub struct BasicCrossover;

impl Crossover<String> for BasicCrossover
{
    fn crossover(&self, parent1: &String, parent2: &String) -> String {
        let result = parent1.clone();
        let pos: usize = thread_rng().gen_range(0..parent1.len());
        let replace_len: usize = thread_rng().gen_range(1..=parent1.len() - pos);
    
        result.replace(&parent1[pos..pos+replace_len], &parent2[pos..pos+replace_len])
    }
}