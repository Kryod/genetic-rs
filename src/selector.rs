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

pub struct Rank {
    pub max_pop: usize
}

impl Rank {

    fn calculate_rank(&self, ratings: &Vec<f32>) -> Vec<usize> {
        let mut ranks = vec![0; ratings.len()];

        let mut indices: Vec<usize> = (0..ratings.len()).collect();

        indices.sort_by(|a, b| ratings[*a].partial_cmp(&ratings[*b]).unwrap());

        for i in 0..ranks.len() {
            ranks[indices[i]] = i + 1;
        }

        ranks
    }
}

impl<T: Clone> Selector<T> for Rank {

    fn selector(&self, pop: &Vec<T>, ratings: &Vec<f32>) -> Vec<T> {
        let mut selected = Vec::with_capacity(self.max_pop);
        let mut rng = thread_rng();
        let ranks = self.calculate_rank(ratings);

        let dist = WeightedIndex::new(ranks).unwrap();

        for _ in 0..self.max_pop {
            selected.push(pop[dist.sample(&mut rng)].clone());
        }

        selected
    }
}

#[test]
fn test_calculate_ranks() {
    let selector = Rank {max_pop: 2};

    //let pop = vec![String::from("aaaa"), String::from("bbbb"), String::from("cccc"), String::from("dddd")];
    let ratings = vec![1.0, 4.7, 2.9, 0.2];

    let result = selector.calculate_rank(&ratings);
    assert_eq!(vec![2, 4, 3, 1], result);
}

#[test]
fn test_elitism() {
    let selector = Elitism {max_pop: 2};

    let pop = vec![String::from("aaaa"), String::from("bbbb"), String::from("cccc"), String::from("dddd")];
    let ratings = vec![1.0, 4.7, 2.9, 0.2];

    let result = selector.selector(&pop, &ratings);
    assert_eq!(vec![String::from("bbbb"), String::from("cccc")], result);
}