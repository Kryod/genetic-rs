
pub trait Mutation<T> {
    fn mutation(&self, pop: &mut T) ;
}