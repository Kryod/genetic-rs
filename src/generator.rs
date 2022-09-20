use rand::{thread_rng, Rng, distributions::Alphanumeric};

pub trait Generator<T> {
    fn generator(&self) -> T;
}

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
