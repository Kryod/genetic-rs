use rand::{thread_rng, Rng, distributions::Alphanumeric};

pub trait Mutation<T> {
    fn mutation(&self, pop: &mut T) ;
}

pub struct BasicMutation;

impl Mutation<String> for BasicMutation
{
    fn mutation(&self, pop: &mut String) {
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
}