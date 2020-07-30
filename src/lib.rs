pub mod genetic_algorithm_traits {
    extern crate rand;

    use rand::Rng;

    pub trait Gene<T: Ord> {
        fn cross_prob(&self) -> f64;
        fn mutate_prob(&self) -> f64;
        fn eval(&self) -> T;
        fn crossover(&self, rhs: &Self, rng: &mut impl Rng) -> Self;
        fn mutate(&mut self, rng: &mut impl Rng);
    }

    pub trait Group {
        fn select(&self, rng: &mut impl Rng) -> Self;
        fn next_generation(&self, rng: &mut impl Rng) -> Self;
    }
}
