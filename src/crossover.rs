pub trait Crossover<T> {
    fn crossover(&self, parent1: &T, parent2: &T) -> T;
}
