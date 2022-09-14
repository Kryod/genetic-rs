use rand::{thread_rng, distributions::WeightedIndex, prelude::Distribution};

pub trait Selector<T> {
    fn selector(&self, pop: &Vec<T>, ratings: &Vec<f32>) -> Vec<T>;
}

pub struct Rating {
    pub max_pop: usize
}

impl<T: Clone> Selector<T> for Rating {

    fn selector(&self, pop: &Vec<T>, ratings: &Vec<f32>) -> Vec<T> {
        let mut selected = Vec::with_capacity(self.max_pop);
        let mut rng = thread_rng();

        let dist = WeightedIndex::new(ratings).unwrap();

        for _ in 0..self.max_pop {
            selected.push(pop[dist.sample(&mut rng)].clone());
        }

        selected
    }
}