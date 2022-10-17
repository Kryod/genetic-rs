use rand::{thread_rng, Rng, distributions::Alphanumeric};

use genetic_rs::generator::Generator;

pub struct BasicGenerator {
    pub string_size: usize
}

impl Generator<String> for BasicGenerator {

    fn generator(&self) -> String {
        thread_rng()
        .sample_iter(&Alphanumeric)
        .take(self.string_size)
        .map(char::from)
        .collect()
    }
}