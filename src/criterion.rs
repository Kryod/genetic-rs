pub trait Criterion {
    fn criterion(&mut self, ratings: &Vec<f32>) -> bool;
}

pub struct Mark {
    pub max_rating: f32,
}

impl Criterion for Mark
{
    fn criterion(&mut self, ratings: &Vec<f32>) -> bool {
        for r in ratings {
            if *r >= self.max_rating {
                return true;
            }
        }
        false
    }
}

#[derive(Default)]
pub struct Plateau {
    pub max_iterations: usize,
    iterations: usize,
    prev_rating: f32
}

impl Plateau {

    pub fn new(max_iterations: usize) -> Self {
        Self {
            max_iterations,
            iterations: 0,
            prev_rating: 0.0
        }
    }
}

impl Criterion for Plateau
{

    fn criterion(&mut self, ratings: &Vec<f32>) -> bool {
        let max = ratings.clone().into_iter().reduce(f32::max).unwrap();
        if self.prev_rating != max {
            self.prev_rating = max;
            self.iterations = 0;
        }
        else if self.prev_rating == max {
            self.iterations += 1;
        }

        self.max_iterations <= self.iterations
    }
}

#[derive(Default)]
pub struct Iterations {
    pub max_iterations: usize,
    iterations: usize
}

impl Criterion for Iterations
{
    fn criterion(&mut self, _ratings: &Vec<f32>) -> bool {
        self.iterations += 1;

        self.max_iterations <= self.iterations
    }
}

impl Iterations {

    pub fn new(max_iterations: usize) -> Self {
        Self {
            max_iterations,
            iterations: 0,
        }
    }
}