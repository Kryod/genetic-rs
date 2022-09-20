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

pub struct Elitism {
    pub max_pop: usize
}

impl<T: Clone> Selector<T> for Elitism {

    fn selector(&self, pop: &Vec<T>, ratings: &Vec<f32>) -> Vec<T> {
        let mut selected = Vec::with_capacity(self.max_pop);

        let mut pairs: Vec<(&T, &f32)> =pop.iter().zip(ratings.iter()).collect();
        pairs.sort_by(|a, b| (*a).1.partial_cmp((*b).1).unwrap());
        pairs.reverse();
        let mut pairs_iter = pairs.iter();
        for _ in 0..self.max_pop {
            selected.push(pairs_iter.next().unwrap().0.clone());
        }

        selected
    }
}


#[test]
fn test_elitism() {
    let selector = Elitism {max_pop: 2};

    let pop = vec![String::from("aaaa"), String::from("bbbb"), String::from("cccc"), String::from("dddd")];
    let ratings = vec![1.0, 4.7, 2.9, 0.2];

    let result = selector.selector(&pop, &ratings);
    assert_eq!(vec![String::from("bbbb"), String::from("cccc")], result);
}