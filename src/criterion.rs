

pub trait Criterion {
    fn criterion(&self, ratings: &Vec<f32>) -> bool;
}

pub struct Mark {
    pub max_rating: f32,
}

impl Criterion for Mark
{
    fn criterion(&self, ratings: &Vec<f32>) -> bool {
        for r in ratings {
            if *r >= self.max_rating {
                return true;
            }
        }
        false
    }
}