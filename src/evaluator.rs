pub trait Evaluator<T> {
    fn evaluator(&self, pop: &T) -> f32;
}